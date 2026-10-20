use crate::generational::define_generational;

define_generational!(
    TextureID,
    "Texture ID — allocated by TextureManager. Index + generation."
);

impl TextureID {
    // Parse hex string (8 or 16 chars, optional 0x prefix) into TextureID
    pub fn parse_str(s: &str) -> Result<Self, String> {
        let s = s.strip_prefix("0x").unwrap_or(s).replace('-', "");
        if s.len() <= 8 {
            u32::from_str_radix(&s, 16)
                .map(Self::from_u32)
                .map_err(|e| format!("Invalid TextureID string: {}", e))
        } else {
            u64::from_str_radix(&s[..16.min(s.len())], 16)
                .map(Self::from_u64)
                .map_err(|e| format!("Invalid TextureID string: {}", e))
        }
    }
}
