// Generates NodeType: the enum and the per-variant tables
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
macro_rules! __define_node_type {
    (
        base: { $($base_variant:ident $(=> $base_ty:ty)?),* $(,)? }
        2d: { $($variant_2d:ident => ($parent_2d:ident, $ty_2d:ty, $renderable_2d:expr, $internal_update_2d:expr)),* $(,)? }
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum NodeType {
            $($base_variant,)*
            $($variant_2d,)*
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

            pub const fn parent_type(&self) -> Option<NodeType> {
                match self {
                    $(NodeType::$base_variant => None,)*
                    $(NodeType::$variant_2d => $crate::__node_parent_opt!($parent_2d),)*
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
        }
    };
}
