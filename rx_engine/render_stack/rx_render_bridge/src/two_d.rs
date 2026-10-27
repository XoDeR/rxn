use super::*;
use rx_structs::BitMask;

#[derive(Debug, Clone, PartialEq)]
pub struct Camera2DState {
    pub position: [f32; 2],
    pub rotation_radians: f32,
    pub zoom: f32,
    pub render_mask: BitMask,
}

impl Default for Camera2DState {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            rotation_radians: 0.0,
            zoom: 1.0,
            render_mask: BitMask::NONE,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect2DCommand {
    pub center: [f32; 2],
    pub size: [f32; 2],
    pub color: Color,
    pub z_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite2DCommand {
    pub texture: TextureID,
    pub model: [[f32; 3]; 3],
    pub tint: Color,
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub size: [f32; 2],
    // Anchor in sprite-local pixels from bottom-left. Center when `[size.x/2, size.y/2]`.
    pub anchor_point: [f32; 2],
    pub z_index: i32,
    // Particle additive blend (`SRC_ALPHA`, `ONE`). World sprites stay false.
    // Will be used later for particles
    pub additive: bool,
}

impl Default for Sprite2DCommand {
    fn default() -> Self {
        Self {
            texture: TextureID::nil(),
            model: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            tint: Color::WHITE,
            uv_min: [0.0, 0.0],
            uv_max: [0.0, 0.0],
            size: [0.0, 0.0],
            anchor_point: [-1.0, -1.0],
            z_index: 0,
            additive: false,
        }
    }
}
