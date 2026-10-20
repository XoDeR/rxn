// Type-safe generational identifiers (slotmap-style) for arenas.
// All IDs use u64 = index (low 32 bits) | generation (high 32 bits).
// Index 0 = nil.
// IDs are created by their owning arena/manager;
// slot reuse bumps generation so stale IDs are invalid.

// Defines a generational ID type (NodeID, TextureID, etc.).
// All such IDs use index + generation for safe arena slot reuse
macro_rules! define_generational {
    ($type_name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type_name(pub u64);

        impl $type_name {
            #[inline]
            pub const fn new(id: u32) -> Self {
                Self::from_parts(id, 0)
            }

            #[inline]
            pub const fn nil() -> Self {
                Self(0)
            }

            #[inline]
            pub const fn index(self) -> u32 {
                (self.0 & 0xFFFF_FFFF) as u32
            }

            #[inline]
            pub const fn generation(self) -> u32 {
                (self.0 >> 32) as u32
            }

            #[inline]
            pub const fn from_parts(index: u32, generation: u32) -> Self {
                Self((index as u64) | ((generation as u64) << 32))
            }

            #[inline]
            pub const fn as_u64(self) -> u64 {
                self.0
            }

            #[inline]
            pub const fn from_u64(value: u64) -> Self {
                Self(value)
            }

            #[inline]
            pub const fn is_nil(self) -> bool {
                self.0 == 0
            }

            // index in low 32, generation 0 (e.g. deserialization)
            #[inline]
            pub const fn from_u32(index: u32) -> Self {
                Self::from_parts(index, 0)
            }
        }

        impl Default for $type_name {
            fn default() -> Self {
                Self::nil()
            }
        }

        impl ::std::fmt::Debug for $type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    concat!(stringify!($type_name), "({}:{})"),
                    self.index(),
                    self.generation()
                )
            }
        }

        impl ::std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}:{}", self.index(), self.generation())
            }
        }
    };
}

pub(crate) use define_generational;
