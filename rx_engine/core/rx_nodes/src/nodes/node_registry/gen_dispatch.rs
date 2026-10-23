// Generates NodeTypeDispatch / NodeBaseDispatch impls for every node type
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
macro_rules! __define_node_dispatch {
    (
        base: { $($base_variant:ident $(=> $base_ty:ty)?),* $(,)? }
        2d: { $($variant_2d:ident => ($parent_2d:ident, $ty_2d:ty, $renderable_2d:expr, $internal_update_2d:expr)),* $(,)? }
    ) => {
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
