#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BitMask(u32);

pub trait IntoBitMaskLayer {
    fn into_bitmask_layer(self) -> Option<u8>;
}

pub trait IntoBitMaskLayers {
    fn into_bitmask(self) -> Option<BitMask>;
}

macro_rules! impl_into_bitmask_layer_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoBitMaskLayer for $ty {
                #[inline]
                fn into_bitmask_layer(self) -> Option<u8> {
                    if (1..=32).contains(&self) {
                        Some(self as u8)
                    } else {
                        None
                    }
                }
            }

            impl IntoBitMaskLayer for &$ty {
                #[inline]
                fn into_bitmask_layer(self) -> Option<u8> {
                    (*self).into_bitmask_layer()
                }
            }

            impl IntoBitMaskLayers for $ty {
                #[inline]
                fn into_bitmask(self) -> Option<BitMask> {
                    BitMask::try_layer(self.into_bitmask_layer()?)
                }
            }

            impl IntoBitMaskLayers for &$ty {
                #[inline]
                fn into_bitmask(self) -> Option<BitMask> {
                    (*self).into_bitmask()
                }
            }
        )*
    };
}

impl_into_bitmask_layer_unsigned!(u8, u16, u32, u64, usize);

macro_rules! impl_into_bitmask_layer_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoBitMaskLayer for $ty {
                #[inline]
                fn into_bitmask_layer(self) -> Option<u8> {
                    if (1..=32).contains(&self) {
                        Some(self as u8)
                    } else {
                        None
                    }
                }
            }

            impl IntoBitMaskLayer for &$ty {
                #[inline]
                fn into_bitmask_layer(self) -> Option<u8> {
                    (*self).into_bitmask_layer()
                }
            }

            impl IntoBitMaskLayers for $ty {
                #[inline]
                fn into_bitmask(self) -> Option<BitMask> {
                    BitMask::try_layer(self.into_bitmask_layer()?)
                }
            }

            impl IntoBitMaskLayers for &$ty {
                #[inline]
                fn into_bitmask(self) -> Option<BitMask> {
                    (*self).into_bitmask()
                }
            }
        )*
    };
}

impl_into_bitmask_layer_signed!(i8, i16, i32, i64, isize);

impl<L, const N: usize> IntoBitMaskLayers for [L; N]
where
    L: IntoBitMaskLayer,
{
    #[inline]
    fn into_bitmask(self) -> Option<BitMask> {
        BitMask::try_from_layers(self)
    }
}

impl<L> IntoBitMaskLayers for &[L]
where
    for<'a> &'a L: IntoBitMaskLayer,
{
    #[inline]
    fn into_bitmask(self) -> Option<BitMask> {
        BitMask::try_from_layers(self)
    }
}

impl<L> IntoBitMaskLayers for &Vec<L>
where
    for<'a> &'a L: IntoBitMaskLayer,
{
    #[inline]
    fn into_bitmask(self) -> Option<BitMask> {
        BitMask::try_from_layers(self)
    }
}

impl BitMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u32::MAX);

    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn bit(bit: u8) -> Self {
        if bit < u32::BITS as u8 {
            Self(1u32 << bit)
        } else {
            Self::NONE
        }
    }

    #[inline]
    pub const fn layer(layer: u8) -> Self {
        assert!(layer >= 1 && layer <= 32, "BitMask layer must be 1..=32");
        Self::bit(layer - 1)
    }

    #[inline]
    pub const fn try_layer(layer: u8) -> Option<Self> {
        if layer >= 1 && layer <= 32 {
            Some(Self::bit(layer - 1))
        } else {
            None
        }
    }

    #[inline]
    pub const fn with<const N: usize>(layers: [u8; N]) -> Self {
        let mut bits = 0u32;
        let mut i = 0usize;
        while i < N {
            bits |= Self::layer(layers[i]).bits();
            i += 1;
        }
        Self(bits)
    }

    #[inline]
    pub fn from_layers<I, L>(layers: I) -> Self
    where
        I: IntoIterator<Item = L>,
        L: IntoBitMaskLayer,
    {
        Self::try_from_layers(layers).expect("BitMask layer must be 1..=32")
    }

    #[inline]
    pub fn try_from_layers<I, L>(layers: I) -> Option<Self>
    where
        I: IntoIterator<Item = L>,
        L: IntoBitMaskLayer,
    {
        let mut bits = 0u32;
        for layer in layers {
            let layer = layer.into_bitmask_layer()?;
            bits |= Self::layer(layer).bits();
        }
        Some(Self(bits))
    }

    #[inline]
    pub const fn with_bits<const N: usize>(bits_in: [u8; N]) -> Self {
        let mut bits = 0u32;
        let mut i = 0usize;
        while i < N {
            bits |= Self::bit(bits_in[i]).bits();
            i += 1;
        }
        Self(bits)
    }

    #[inline]
    pub const fn without_layers<const N: usize>(self, layers: [u8; N]) -> Self {
        Self(self.bits() & !Self::with(layers).bits())
    }

    #[inline]
    pub fn push<L>(&mut self, layers: L)
    where
        L: IntoBitMaskLayers,
    {
        *self = self.pushed(layers);
    }

    #[inline]
    pub fn pushed<L>(self, layers: L) -> Self
    where
        L: IntoBitMaskLayers,
    {
        Self(
            self.bits()
                | layers
                    .into_bitmask()
                    .expect("BitMask layer must be 1..=32")
                    .bits(),
        )
    }

    #[inline]
    pub fn pop<L>(&mut self, layers: L)
    where
        L: IntoBitMaskLayers,
    {
        *self = self.popped(layers);
    }

    #[inline]
    pub fn popped<L>(self, layers: L) -> Self
    where
        L: IntoBitMaskLayers,
    {
        Self(
            self.bits()
                & !layers
                    .into_bitmask()
                    .expect("BitMask layer must be 1..=32")
                    .bits(),
        )
    }

    #[inline]
    pub fn without<L>(layers: L) -> Self
    where
        L: IntoBitMaskLayers,
    {
        Self::ALL.popped(layers)
    }

    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }

    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }

    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }

    #[inline]
    pub const fn intersects(self, other: Self) -> bool {
        (self.bits() & other.bits()) != 0
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.bits() == 0
    }
}

impl Default for BitMask {
    #[inline]
    fn default() -> Self {
        Self::ALL
    }
}

impl From<u32> for BitMask {
    #[inline]
    fn from(value: u32) -> Self {
        Self::from_bits(value)
    }
}

impl From<BitMask> for u32 {
    #[inline]
    fn from(value: BitMask) -> Self {
        value.bits()
    }
}

#[macro_export]
macro_rules! bitmask {
    ([]) => {
        $crate::BitMask::NONE
    };
    () => {
        $crate::BitMask::NONE
    };
    ([$($layer:expr),+ $(,)?]) => {
        $crate::BitMask::with([$($layer),+])
    };
}
