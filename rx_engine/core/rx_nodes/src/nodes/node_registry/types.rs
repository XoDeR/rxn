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
