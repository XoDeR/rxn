use std::ops::{Deref, DerefMut};

use crate::node_2d::Node2D;
use rx_ids::TextureID;

// Uses `Node2D` position / rotation / scale / z
#[derive(Clone, Debug, Default)]
pub struct Sprite2D {
    pub base: Node2D,
    pub texture: TextureID,
    pub texture_region: Option<[f32; 4]>,
    // Anchor in sprite-local pixels from bottom-left
    // `None` => sprite center
    pub anchor_point: Option<[f32; 2]>,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Deref for Sprite2D {
    type Target = Node2D;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Sprite2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Sprite2D {
    pub const fn new() -> Self {
        Self {
            base: Node2D::new(),
            texture: TextureID::nil(),
            texture_region: None,
            anchor_point: None,
            flip_x: false,
            flip_y: false,
        }
    }
}
