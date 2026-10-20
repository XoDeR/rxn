// Types:
// NodeID
// TextureID
// FontID
// SignalID
// TagID

mod font_id;
mod generational;
mod hash;
mod macros;
mod node_id;
mod signal_id;
mod tag_id;
mod texture_id;

pub use font_id::*;
pub use hash::*;
pub use node_id::*;
pub use signal_id::*;
pub use tag_id::*;
pub use texture_id::*;

pub mod prelude {
    pub use crate::{FontID, IntoTagID, NodeID, NodeTag, SignalID, TagID, TextureID};
    pub use crate::{hash_str, signal, tag, tags};
}
