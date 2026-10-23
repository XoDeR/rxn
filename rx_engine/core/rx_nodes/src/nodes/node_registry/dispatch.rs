use super::{InternalUpdate, NodeType, Renderable, SceneNodeData, Spatial};

pub trait NodeTypeDispatch: Sized {
    const NODE_TYPE: NodeType;
    const SPATIAL: Spatial;
    const RENDERABLE: Renderable;
    const INTERNAL_UPDATE: InternalUpdate;
    type TransformSnapshot: Copy + PartialEq;

    fn with_ref<R>(data: &SceneNodeData, f: impl FnOnce(&Self) -> R) -> Option<R>;
    fn with_mut<R>(data: &mut SceneNodeData, f: impl FnOnce(&mut Self) -> R) -> Option<R>;

    #[inline]
    fn snapshot_transform(_value: &Self) -> Option<Self::TransformSnapshot> {
        None
    }
}

pub trait NodeBaseDispatch: Sized {
    const BASE_NODE_TYPE: NodeType;

    fn with_base_ref<R>(data: &SceneNodeData, f: impl FnOnce(&Self) -> R) -> Option<R>;
    fn with_base_mut<R>(data: &mut SceneNodeData, f: impl FnOnce(&mut Self) -> R) -> Option<R>;
}
