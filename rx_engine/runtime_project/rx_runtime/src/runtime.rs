use crate::containers::NodeArena;
use rx_runtime_render::Render2DState;

pub struct RuntimeInner {
    pub nodes: NodeArena,
    render: RenderState,
    render_2d: Render2DState,
}

impl RuntimeInner {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            render: Default::default(),
            render_2d: Render2DState::new(),
        }
    }
}
