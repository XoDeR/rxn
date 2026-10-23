// Parts of NodeType that do not depend on the node list.
// The enum itself and the per-variant tables are generated in gen_node_type.rs
use super::{NodeType, Spatial};

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Node
    }
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl NodeType {
    pub const fn name(&self) -> &'static str {
        self.as_str()
    }

    pub const fn is_a(&self, base: NodeType) -> bool {
        if *self as u8 == base as u8 {
            return true;
        }

        let mut cursor = *self;
        loop {
            match cursor.parent_type() {
                Some(parent) => {
                    if parent as u8 == base as u8 {
                        return true;
                    }
                    cursor = parent;
                }
                None => return false,
            }
        }
    }

    pub const fn is_2d(&self) -> bool {
        matches!(self.get_spatial(), Spatial::TwoD)
    }

    pub const fn is_spatial(&self) -> bool {
        matches!(self.get_spatial(), Spatial::TwoD)
    }
}
