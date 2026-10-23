// Node registry. The list of node types at the bottom drives everything
//
// Plain code (no macros):
//   types.rs      Spatial, Renderable, InternalUpdate
//   scene_node.rs SceneNode struct and its methods
//   node_type.rs  NodeType methods that do not depend on the list
//   dispatch.rs   NodeTypeDispatch, NodeBaseDispatch traits
//
// Generated from the list (one macro per file):
//   gen_node_data.rs  SceneNodeData
//   gen_node_type.rs  NodeType
//   gen_dispatch.rs   dispatch impls for every node type
mod dispatch;
mod gen_dispatch;
mod gen_node_data;
mod gen_node_type;
mod node_type;
mod scene_node;
mod types;

pub use dispatch::*;
pub use scene_node::*;
pub use types::*;

use crate::{Camera2D, Node2D, Sprite2D};
use rx_structs::Transform2D;

// Forwards the node list to each generator
#[macro_export]
macro_rules! define_scene_nodes {
    ($($list:tt)*) => {
        $crate::__define_node_data!($($list)*);
        $crate::__define_node_type!($($list)*);
        $crate::__define_node_dispatch!($($list)*);
    };
}

define_scene_nodes! {
    base: {
        Node,
    }
    2d: {
        // core
        Node2D => (None, Node2D, Renderable::False, InternalUpdate::False),

        // camera
        Camera2D => (Node2D, Camera2D, Renderable::True, InternalUpdate::False),

        // visual
        Sprite2D => (Node2D, Sprite2D, Renderable::True, InternalUpdate::False),
    }
}
