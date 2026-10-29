use ahash::{AHashMap, AHashSet};
use rx_ids::NodeID;
use rx_render_bridge::{Camera2DState, RenderEvent, RenderRequestID, Sprite2DCommand};

pub fn sprite_2d_texture_request(node: NodeID) -> RenderRequestID {
    RenderRequestID::new((node.as_u64() << 8) | 0x2D)
}

pub fn decode_2d_texture_request_node(request: RenderRequestID) -> Option<NodeID> {
    if (request.0 & 0xFF) != 0x2D {
        return None;
    }
    Some(NodeID::from_u64(request.0 >> 8))
}

pub fn decode_render_request_node(request: RenderRequestID) -> Option<NodeID> {
    decode_2d_texture_request_node(request)
}

pub fn decode_render_request_node_from_event(event: &RenderEvent) -> Option<NodeID> {
    let request = match event {
        RenderEvent::TextureCreated { request, .. } | RenderEvent::Failed { request, .. } => {
            *request
        }
        RenderEvent::TextureLoaded { .. } | RenderEvent::TextureDropped { .. } => return None,
    };
    decode_render_request_node(request)
}

fn collect_tree_traversal_with_seen<I, A, F>(
    traversal_ids: &mut Vec<NodeID>,
    seen: &mut AHashSet<NodeID>,
    child_scratch: &mut Vec<NodeID>,
    seed_ids: I,
    all_ids: A,
    include_all: bool,
    mut children_of: F,
) where
    I: IntoIterator<Item = NodeID>,
    A: IntoIterator<Item = NodeID>,
    F: FnMut(NodeID, &mut Vec<NodeID>),
{
    traversal_ids.clear();
    seen.clear();
    for id in seed_ids {
        if seen.insert(id) {
            traversal_ids.push(id);
        }
    }
    if include_all {
        for id in all_ids {
            if seen.insert(id) {
                traversal_ids.push(id);
            }
        }
    }

    child_scratch.clear();
    let mut traversal_cursor = 0usize;
    while traversal_cursor < traversal_ids.len() {
        let node = traversal_ids[traversal_cursor];
        traversal_cursor += 1;
        child_scratch.clear();
        children_of(node, child_scratch);
        for child in child_scratch.iter().copied() {
            if seen.insert(child) {
                traversal_ids.push(child);
            }
        }
    }
}

pub struct Render2DState {
    pub traversal_ids: Vec<NodeID>,
    pub traversal_seen: AHashSet<NodeID>,
    pub traversal_child_scratch: Vec<NodeID>,
    pub dirty_ids_scratch: Vec<NodeID>,
    pub visible_now: AHashSet<NodeID>,
    pub prev_visible: AHashSet<NodeID>,
    pub retained_sprites: AHashMap<NodeID, Sprite2DCommand>,
    pub texture_sources: AHashMap<NodeID, String>,
    pub last_camera: Option<Camera2DState>,
    pub removed_nodes: Vec<NodeID>,
    pub force_full_scan_once: bool,
}

impl Render2DState {
    pub fn new() -> Self {
        Self {
            traversal_ids: Vec::new(),
            traversal_seen: AHashSet::default(),
            traversal_child_scratch: Vec::new(),
            dirty_ids_scratch: Vec::new(),
            visible_now: AHashSet::default(),
            prev_visible: AHashSet::default(),
            retained_sprites: AHashMap::default(),
            texture_sources: AHashMap::default(),
            last_camera: None,
            removed_nodes: Vec::new(),
            force_full_scan_once: false,
        }
    }

    pub fn note_removed_node(&mut self, node: NodeID) {
        self.removed_nodes.push(node);
        self.retained_sprites.remove(&node);
        self.texture_sources.remove(&node);
    }

    pub fn request_full_scan_once(&mut self) {
        self.force_full_scan_once = true;
    }

    pub fn collect_traversal<I, A, F>(
        &mut self,
        dirty_ids: I,
        all_ids: A,
        bootstrap_scan: bool,
        children_of: F,
    ) -> Vec<NodeID>
    where
        I: IntoIterator<Item = NodeID>,
        A: IntoIterator<Item = NodeID>,
        F: FnMut(NodeID, &mut Vec<NodeID>),
    {
        let include_all = self.force_full_scan_once || bootstrap_scan;
        self.force_full_scan_once = false;
        let mut traversal_ids = std::mem::take(&mut self.traversal_ids);
        let mut seen = std::mem::take(&mut self.traversal_seen);
        let mut child_scratch = std::mem::take(&mut self.traversal_child_scratch);
        collect_tree_traversal_with_seen(
            &mut traversal_ids,
            &mut seen,
            &mut child_scratch,
            dirty_ids,
            all_ids,
            include_all,
            children_of,
        );
        child_scratch.clear();
        seen.clear();
        self.traversal_child_scratch = child_scratch;
        self.traversal_seen = seen;
        traversal_ids
    }

    pub fn take_dirty_ids_scratch(&mut self) -> Vec<NodeID> {
        std::mem::take(&mut self.dirty_ids_scratch)
    }

    pub fn restore_dirty_ids_scratch(&mut self, mut dirty_ids: Vec<NodeID>) {
        dirty_ids.clear();
        self.dirty_ids_scratch = dirty_ids;
    }

    pub fn collect_traversal_with_scratch<A, F>(
        &mut self,
        dirty_ids: &[NodeID],
        all_ids: A,
        bootstrap_scan: bool,
        children_of: F,
    ) -> Vec<NodeID>
    where
        A: IntoIterator<Item = NodeID>,
        F: FnMut(NodeID, &mut Vec<NodeID>),
    {
        let include_all = self.force_full_scan_once || bootstrap_scan;
        self.force_full_scan_once = false;
        let mut traversal_ids = std::mem::take(&mut self.traversal_ids);
        let mut seen = std::mem::take(&mut self.traversal_seen);
        let mut child_scratch = std::mem::take(&mut self.traversal_child_scratch);
        collect_tree_traversal_with_seen(
            &mut traversal_ids,
            &mut seen,
            &mut child_scratch,
            dirty_ids.iter().copied(),
            all_ids,
            include_all,
            children_of,
        );
        child_scratch.clear();
        seen.clear();
        self.traversal_child_scratch = child_scratch;
        self.traversal_seen = seen;
        traversal_ids
    }

    pub fn begin_visible_pass(&mut self) -> AHashSet<NodeID> {
        let mut visible_now = std::mem::take(&mut self.visible_now);
        visible_now.clear();
        visible_now.extend(self.prev_visible.iter().copied());
        for node in self.removed_nodes.drain(..) {
            visible_now.remove(&node);
        }
        visible_now
    }

    pub fn collect_removed_visible_nodes(&self, visible_now: &AHashSet<NodeID>) -> Vec<NodeID> {
        self.prev_visible
            .iter()
            .copied()
            .filter(|node| !visible_now.contains(node))
            .collect()
    }

    pub fn finish_visible_pass(
        &mut self,
        mut traversal_ids: Vec<NodeID>,
        mut visible_now: AHashSet<NodeID>,
    ) {
        std::mem::swap(&mut self.prev_visible, &mut visible_now);
        visible_now.clear();
        self.visible_now = visible_now;
        traversal_ids.clear();
        self.traversal_ids = traversal_ids;
    }
}

impl Default for Render2DState {
    fn default() -> Self {
        Self::new()
    }
}
