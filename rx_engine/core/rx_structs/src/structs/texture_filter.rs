#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextureFilterMode {
    Nearest,
    #[default]
    Linear,
}

impl TextureFilterMode {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nearest => "nearest",
            Self::Linear => "linear",
        }
    }

    #[inline]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "nearest" | "point" | "pixel" | "pixel_art" => Some(Self::Nearest),
            "linear" | "bilinear" | "smooth" => Some(Self::Linear),
            _ => None,
        }
    }
}
