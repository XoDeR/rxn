use rx_ids::{NodeID, NodeTag, TagID};
use rx_structs::Transform2D;
use std::borrow::Cow;

use crate::{Camera2D, Node2D, Sprite2D};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
// Used by runtime transform + query paths
pub enum Spatial {
    None,
    TwoD,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Renderable {
    False,
    True,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InternalUpdate {
    False,
    True,
}

#[macro_export]
macro_rules! __node_parent_opt {
    (None) => {
        None
    };
    ($parent:ident) => {
        Some(NodeType::$parent)
    };
}

#[macro_export]
macro_rules! __node2d_base_expr {
    (Node2D, None, $inner:ident, $f:ident) => {
        Some($f($inner))
    };
    ($_variant:ident, None, $inner:ident, $_f:ident) => {{
        let _ = &$inner;
        None
    }};
    ($_variant:ident, $parent:ident, $inner:ident, $f:ident) => {
        Some($f($inner))
    };
}

#[macro_export]
macro_rules! __impl_exact_node_base_dispatch_2d {
    (Node2D, $ty_2d:ty, $variant_2d:ident) => {};
    ($variant:ident, $ty_2d:ty, $variant_2d:ident) => {
        impl NodeBaseDispatch for $ty_2d {
            const BASE_NODE_TYPE: NodeType = NodeType::$variant_2d;

            fn with_base_ref<R>(data: &SceneNodeData, f: impl FnOnce(&Self) -> R) -> Option<R> {
                match data {
                    SceneNodeData::$variant_2d(inner) => Some(f(inner)),
                    _ => None,
                }
            }

            fn with_base_mut<R>(
                data: &mut SceneNodeData,
                f: impl FnOnce(&mut Self) -> R,
            ) -> Option<R> {
                match data {
                    SceneNodeData::$variant_2d(inner) => Some(f(inner)),
                    _ => None,
                }
            }
        }
    };
}

#[macro_export]
// Creates SceneNode. Includes node enum, node type metadata, and typed dispatch impls
macro_rules! define_scene_nodes {
    (
        base: { $($base_variant:ident $(=> $base_ty:ty)?),* $(,)? }
        2d: { $($variant_2d:ident => ($parent_2d:ident, $ty_2d:ty, $renderable_2d:expr, $internal_update_2d:expr)),* $(,)? }
    ) => {
        #[derive(Clone, Debug)]
        pub struct SceneNode {
            pub data: SceneNodeData,
            pub id: NodeID,
            pub name: Cow<'static, str>,
            pub parent: NodeID,
            pub children: Vec<NodeID>,
            pub tags: Vec<NodeTag>,
        }

        #[derive(Clone, Debug)]
        #[allow(clippy::large_enum_variant)]
        pub enum SceneNodeData {
            $(
                $base_variant$(($base_ty))?,
            )*
            $($variant_2d($ty_2d),)*
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum NodeType {
            $($base_variant,)*
            $($variant_2d,)*
        }

        impl SceneNode {
            pub fn new(data: SceneNodeData) -> Self {
                Self {
                    id: NodeID::nil(),
                    name: Cow::Borrowed("Node"),
                    parent: NodeID::nil(),
                    children: Vec::new(),
                    tags: Vec::new(),
                    data,
                }
            }

            pub const fn has_parent(&self) -> bool {
                !self.parent.is_nil()
            }

            pub fn get_name(&self) -> &str {
                self.name.as_ref()
            }

            pub fn set_name<S>(&mut self, name: S)
            where
                S: Into<Cow<'static, str>>,
            {
                self.name = name.into();
            }

            pub const fn get_parent(&self) -> NodeID {
                self.parent
            }

            pub fn get_children_ids(&self) -> &[NodeID] {
                &self.children
            }

            pub fn set_children_ids<C>(&mut self, children: Option<C>)
            where
                C: Into<Vec<NodeID>>,
            {
                self.children = children.map(Into::into).unwrap_or_default();
            }

            pub fn get_tags(&self) -> &[NodeTag] {
                &self.tags
            }

            pub fn get_tag_ids(&self) -> Vec<TagID> {
                self.tags.iter().map(NodeTag::id).collect()
            }

            pub fn set_tag_ids<T>(&mut self, tags: Option<T>)
            where
                T: Into<Vec<TagID>>,
            {
                self.tags = tags
                    .map(Into::into)
                    .unwrap_or_default()
                    .into_iter()
                    .map(NodeTag::from)
                    .collect();
            }

            pub fn set_tags<T>(&mut self, tags: Option<T>)
            where
                T: Into<Vec<NodeTag>>,
            {
                self.tags = tags.map(Into::into).unwrap_or_default();
            }

            pub const fn node_type(&self) -> NodeType {
                match &self.data {
                    $(
                        SceneNodeData::$base_variant { .. } =>
                            NodeType::$base_variant,
                    )*
                    $(
                        SceneNodeData::$variant_2d(_) =>
                            NodeType::$variant_2d,
                    )*
                }
            }

            pub const fn spatial(&self) -> Spatial {
                match &self.data {
                    $(
                        SceneNodeData::$base_variant { .. } => Spatial::None,
                    )*
                    $(SceneNodeData::$variant_2d(_) => Spatial::TwoD,)*
                }
            }

            pub const fn is_2d(&self) -> bool {
                matches!(self.spatial(), Spatial::TwoD)
            }

            pub const fn is_spatial(&self) -> bool {
                matches!(self.spatial(), Spatial::TwoD)
            }

            pub fn add_child(&mut self, child: NodeID) {
                self.children.push(child);
            }

            pub fn remove_child(&mut self, child: NodeID) {
                self.children.retain(|&c| c != child);
            }

            pub fn clear_children(&mut self) {
                self.children.clear();
            }

            pub fn children_slice(&self) -> &[NodeID] {
                self.get_children_ids()
            }

            pub fn add_tag<T>(&mut self, tag: T)
            where
                T: Into<NodeTag>,
            {
                self.tags.push(tag.into());
            }

            pub fn remove_tag(&mut self, tag: TagID) {
                self.tags.retain(|t| t.id != tag);
            }

            pub fn clear_tags(&mut self) {
                self.tags.clear();
            }

            pub fn has_tag(&self, tag: TagID) -> bool {
                self.tags.iter().any(|node_tag| node_tag.id == tag)
            }

            pub fn tags_slice(&self) -> &[NodeTag] {
                self.get_tags()
            }

            pub fn with_typed_ref<T: NodeTypeDispatch, R>(
                &self,
                f: impl FnOnce(&T) -> R,
            ) -> Option<R> {
                T::with_ref(&self.data, f)
            }

            pub fn with_typed_mut<T: NodeTypeDispatch, R>(
                &mut self,
                f: impl FnOnce(&mut T) -> R,
            ) -> Option<R> {
                T::with_mut(&mut self.data, f)
            }

            pub fn with_base_ref<T: NodeBaseDispatch, R>(
                &self,
                f: impl FnOnce(&T) -> R,
            ) -> Option<R> {
                T::with_base_ref(&self.data, f)
            }

            pub fn with_base_mut<T: NodeBaseDispatch, R>(
                &mut self,
                f: impl FnOnce(&mut T) -> R,
            ) -> Option<R> {
                T::with_base_mut(&mut self.data, f)
            }
        }

        pub trait NodeTypeDispatch: Sized {
            const NODE_TYPE: NodeType;
            const SPATIAL: Spatial;
            const RENDERABLE: Renderable;
            const INTERNAL_UPDATE: InternalUpdate;
            type TransformSnapshot: Copy + PartialEq;

            fn with_ref<R>(data: &SceneNodeData, f: impl FnOnce(&Self) -> R) -> Option<R>;
            fn with_mut<R>(data: &mut SceneNodeData, f: impl FnOnce(&mut Self) -> R)
                -> Option<R>;

            #[inline]
            fn snapshot_transform(_value: &Self) -> Option<Self::TransformSnapshot> {
                None
            }
        }

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

        impl std::str::FromStr for NodeType {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($base_variant) => Ok(NodeType::$base_variant),)*
                    $(stringify!($variant_2d) => Ok(NodeType::$variant_2d),)*
                    _ => Err(format!("Unknown node type: {}", s)),
                }
            }
        }

        impl NodeType {
            pub const ALL: &'static [NodeType] = &[
                $(NodeType::$base_variant,)*
                $(NodeType::$variant_2d,)*
            ];

            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(NodeType::$base_variant => stringify!($base_variant),)*
                    $(NodeType::$variant_2d => stringify!($variant_2d),)*
                }
            }

            pub const fn name(&self) -> &'static str {
                self.as_str()
            }

            pub const fn parent_type(&self) -> Option<NodeType> {
                match self {
                    $(NodeType::$base_variant => None,)*
                    $(NodeType::$variant_2d => $crate::__node_parent_opt!($parent_2d),)*
                }
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

            pub const fn get_spatial(&self) -> Spatial {
                match self {
                    $(NodeType::$base_variant => Spatial::None,)*
                    $(NodeType::$variant_2d => Spatial::TwoD,)*
                }
            }

            pub const fn get_internal_update(&self) -> InternalUpdate {
                match self {
                    $(NodeType::$base_variant => InternalUpdate::False,)*
                    $(NodeType::$variant_2d => $internal_update_2d,)*
                }
            }

            pub const fn is_2d(&self) -> bool {
                matches!(self.get_spatial(), Spatial::TwoD)
            }

            pub const fn is_spatial(&self) -> bool {
                matches!(self.get_spatial(), Spatial::TwoD)
            }

        }

        $(impl From<$ty_2d> for SceneNodeData {
            fn from(value: $ty_2d) -> Self {
                SceneNodeData::$variant_2d(value)
            }
        })*

        impl From<SceneNodeData> for SceneNode {
            fn from(value: SceneNodeData) -> Self {
                SceneNode::new(value)
            }
        }

        $(impl NodeTypeDispatch for $ty_2d {
            const NODE_TYPE: NodeType = NodeType::$variant_2d;
            const SPATIAL: Spatial = Spatial::TwoD;
            const RENDERABLE: Renderable = $renderable_2d;
            const INTERNAL_UPDATE: InternalUpdate = $internal_update_2d;
            type TransformSnapshot = Transform2D;

            fn with_ref<R>(data: &SceneNodeData, f: impl FnOnce(&Self) -> R) -> Option<R> {
                match data {
                    SceneNodeData::$variant_2d(inner) => Some(f(inner)),
                    _ => None,
                }
            }

            fn with_mut<R>(
                data: &mut SceneNodeData,
                f: impl FnOnce(&mut Self) -> R,
            ) -> Option<R> {
                match data {
                    SceneNodeData::$variant_2d(inner) => Some(f(inner)),
                    _ => None,
                }
            }

            #[inline]
            fn snapshot_transform(value: &Self) -> Option<Self::TransformSnapshot> {
                Some(value.transform)
            }
        })*


        $($crate::__impl_exact_node_base_dispatch_2d!($variant_2d, $ty_2d, $variant_2d);)*

        impl NodeBaseDispatch for Node2D {
            const BASE_NODE_TYPE: NodeType = NodeType::Node2D;

            fn with_base_ref<R>(data: &SceneNodeData, f: impl FnOnce(&Self) -> R) -> Option<R> {
                match data {
                    $(SceneNodeData::$variant_2d(inner) => {
                        $crate::__node2d_base_expr!($variant_2d, $parent_2d, inner, f)
                    },)*
                    _ => None,
                }
            }

            fn with_base_mut<R>(
                data: &mut SceneNodeData,
                f: impl FnOnce(&mut Self) -> R,
            ) -> Option<R> {
                match data {
                    $(SceneNodeData::$variant_2d(inner) => {
                        $crate::__node2d_base_expr!($variant_2d, $parent_2d, inner, f)
                    },)*
                    _ => None,
                }
            }
        }
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

pub trait NodeBaseDispatch: Sized {
    const BASE_NODE_TYPE: NodeType;

    fn with_base_ref<R>(data: &SceneNodeData, f: impl FnOnce(&Self) -> R) -> Option<R>;
    fn with_base_mut<R>(data: &mut SceneNodeData, f: impl FnOnce(&mut Self) -> R) -> Option<R>;
}
