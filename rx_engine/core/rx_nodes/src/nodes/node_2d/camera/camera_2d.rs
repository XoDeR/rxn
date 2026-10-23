use std::ops::{Deref, DerefMut};

use crate::Node2D;
use rx_structs::BitMask;

#[derive(Clone, Debug)]
pub struct Camera2D {
    pub base: Node2D,
    pub zoom: f32,
    pub active: bool,
    pub render_mask: BitMask,
}

impl Deref for Camera2D {
    type Target = Node2D;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Camera2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            base: Node2D::new(),
            zoom: 0f32,
            active: false,
            render_mask: BitMask::NONE,
        }
    }
}
