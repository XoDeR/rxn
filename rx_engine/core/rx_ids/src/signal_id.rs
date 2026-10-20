use crate::{generational::define_generational, hash::string_to_u64};

define_generational!(
    SignalID,
    "Signal ID - hash of signal name (or generational). Used for connect/emit."
);

impl SignalID {
    // Deterministic ID from signal name. Uses hash; generation 0.
    pub const fn from_string(s: &str) -> Self {
        Self::from_u64(string_to_u64(s))
    }
}
