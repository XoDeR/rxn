use rx_ids::NodeID;
use rx_nodes::SceneNode;

// Generational node arena
// Used by runtime hot paths

// Slot 0 is always empty
// so `NodeID::nil()` and raw index 0 never alias a real node
// Removing a node bumps its generation and adds the slot to the free list.
// Every public lookup checks both slot bounds and generation before it
// returns a node reference.
pub struct NodeArena {
    nodes: Vec<Option<SceneNode>>,
    generations: Vec<u32>,
    free_indices: Vec<usize>,
    active_len: usize,
}

impl Default for NodeArena {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeArena {
    // Slot 0 is reserved as the nil sentinel
    // The first inserted node uses index 1
    pub fn new() -> Self {
        let mut nodes = Vec::with_capacity(2);
        let mut generations = Vec::with_capacity(2);
        nodes.push(None);
        generations.push(0);
        Self {
            nodes,
            generations,
            free_indices: Vec::new(),
            active_len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity.saturating_add(1));
        let mut generations = Vec::with_capacity(capacity.saturating_add(1));
        nodes.push(None);
        generations.push(0);
        Self {
            nodes,
            generations,
            free_indices: Vec::new(),
            active_len: 0,
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
        self.generations.reserve(additional);
    }

    // Reuses a free slot when available.
    // Otherwise appends a new slot.
    pub fn insert(&mut self, node: SceneNode) -> NodeID {
        if let Some(index) = self.free_indices.pop() {
            self.nodes[index] = Some(node);
            self.active_len = self.active_len.saturating_add(1);
            let generation = self.generations[index];
            return NodeID::from_parts(index as u32, generation);
        }

        let index = self.nodes.len();
        self.nodes.push(Some(node));
        self.generations.push(0);
        self.active_len = self.active_len.saturating_add(1);
        NodeID::from_parts(index as u32, 0)
    }

    // Returns `None` for:
    // nil ids, stale generations, out-of-bounds/empty slots
    pub fn get(&self, id: NodeID) -> Option<&SceneNode> {
        let index = self.valid_slot(id)?;
        self.nodes[index].as_ref()
    }

    pub fn get_mut(&mut self, id: NodeID) -> Option<&mut SceneNode> {
        let index = self.valid_slot(id)?;
        self.nodes[index].as_mut()
    }

    pub fn remove(&mut self, id: NodeID) -> Option<SceneNode> {
        let index = self.valid_slot(id)?;
        self.generations[index] = self.generations[index].wrapping_add(1);
        let removed = self.nodes[index].take();
        if removed.is_some() {
            self.active_len = self.active_len.saturating_sub(1);
            self.free_indices.push(index);
        }
        removed
    }

    // Check if a `NodeID` currently points at a live node
    pub fn contains(&self, id: NodeID) -> bool {
        self.valid_slot(id)
            .is_some_and(|index| self.nodes[index].is_some())
    }

    pub fn iter(&self) -> impl Iterator<Item = (NodeID, &SceneNode)> {
        self.nodes
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(index, node)| {
                node.as_ref()
                    .map(|n| (NodeID::from_parts(index as u32, self.generations[index]), n))
            })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (NodeID, &mut SceneNode)> {
        self.nodes
            .iter_mut()
            .zip(self.generations.iter())
            .enumerate()
            .skip(1)
            .filter_map(|(index, (node, &generation))| {
                node.as_mut()
                    .map(|n| (NodeID::from_parts(index as u32, generation), n))
            })
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.generations.clear();
        self.free_indices.clear();
        self.active_len = 0;
        self.nodes.push(None);
        self.generations.push(0);
    }

    pub fn len(&self) -> usize {
        self.active_len
    }

    pub fn is_empty(&self) -> bool {
        self.active_len == 0
    }

    pub fn slot_count(&self) -> usize {
        self.nodes.len()
    }

    // Returns node at a raw slot index if occupied
    // Intended for fast linear scans
    pub fn slot_get(&self, index: usize) -> Option<(NodeID, &SceneNode)> {
        if index == 0 || index >= self.nodes.len() {
            return None;
        }
        let node = self.nodes[index].as_ref()?;
        Some((
            NodeID::from_parts(index as u32, self.generations[index]),
            node,
        ))
    }

    // Returns node at a known slot when generation matches
    // Intended for scheduler fast paths
    #[inline]
    pub fn slot_get_checked(&self, index: usize, generation: u32) -> Option<&SceneNode> {
        if index == 0 || index >= self.nodes.len() {
            return None;
        }
        if self.generations[index] != generation {
            return None;
        }
        self.nodes[index].as_ref()
    }

    #[inline]
    pub fn slot_get_mut_checked(
        &mut self,
        index: usize,
        generation: u32,
    ) -> Option<&mut SceneNode> {
        if index == 0 || index >= self.nodes.len() {
            return None;
        }
        if self.generations[index] != generation {
            return None;
        }
        self.nodes[index].as_mut()
    }

    // Validate a public id and return its raw slot index
    #[inline]
    fn valid_slot(&self, id: NodeID) -> Option<usize> {
        let index = id.index() as usize;
        if id.is_nil()
            || index == 0
            || index >= self.nodes.len()
            || self.generations[index] != id.generation()
        {
            return None;
        }
        Some(index)
    }
}
