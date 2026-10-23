// Generates SceneNodeData: one variant per node type, plus From impls
#[macro_export]
macro_rules! __define_node_data {
    (
        base: { $($base_variant:ident $(=> $base_ty:ty)?),* $(,)? }
        2d: { $($variant_2d:ident => ($parent_2d:ident, $ty_2d:ty, $renderable_2d:expr, $internal_update_2d:expr)),* $(,)? }
    ) => {
        #[derive(Clone, Debug)]
        #[allow(clippy::large_enum_variant)]
        pub enum SceneNodeData {
            $(
                $base_variant$(($base_ty))?,
            )*
            $($variant_2d($ty_2d),)*
        }

        impl SceneNodeData {
            pub const fn node_type(&self) -> NodeType {
                match self {
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
                match self {
                    $(
                        SceneNodeData::$base_variant { .. } => Spatial::None,
                    )*
                    $(SceneNodeData::$variant_2d(_) => Spatial::TwoD,)*
                }
            }
        }

        $(impl From<$ty_2d> for SceneNodeData {
            fn from(value: $ty_2d) -> Self {
                SceneNodeData::$variant_2d(value)
            }
        })*
    };
}
