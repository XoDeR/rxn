// How the initial desktop window size is chosen from the virtual canvas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowSizeMode {
    // Scale to ~75% of monitor while preserving aspect ratio
    #[default]
    FitMonitor,
    // Open at `virtual_width` x `virtual_height` with no monitor scaling
    Exact,
}

impl WindowSizeMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FitMonitor => "fit_monitor",
            Self::Exact => "exact",
        }
    }
}

// World-space origin for 2D node positions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CoordinateOrigin2D {
    // `(0, 0)` at viewport center. X right, Y up
    #[default]
    Center,
    // `(0, 0)` at viewport bottom-left. X right, Y up
    BottomLeft,
}

impl CoordinateOrigin2D {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Center => "center",
            Self::BottomLeft => "bottom_left",
        }
    }

    #[inline]
    pub fn render_space_offset(self, virtual_width: u32, virtual_height: u32) -> [f32; 2] {
        match self {
            Self::Center => [0.0, 0.0],
            Self::BottomLeft => [
                -(virtual_width.max(1) as f32 * 0.5),
                -(virtual_height.max(1) as f32 * 0.5),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub window_size: WindowSizeMode,
    pub coordinate_origin: CoordinateOrigin2D,
    pub vsync: bool,
    pub frame_rate_cap: FrameRateCap,
    pub msaa: bool,
    pub texture_filter: rx_structs::TextureFilterMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FrameRateCap {
    #[default]
    Unlimited,
    Fps(f32),
    RefreshRate,
}

impl ProjectConfig {
    pub fn default_for_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            virtual_width: 1024,
            virtual_height: 768,
            window_size: WindowSizeMode::Exact,
            coordinate_origin: CoordinateOrigin2D::Center,
            vsync: false,
            frame_rate_cap: FrameRateCap::Unlimited,
            msaa: true,
            texture_filter: rx_structs::TextureFilterMode::Linear,
        }
    }
}
