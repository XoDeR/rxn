#[macro_export]
macro_rules! hash_str {
    ($value:expr) => {
        const { $crate::string_to_u64($value) }
    };
}

#[macro_export]
macro_rules! signal {
    ($name:expr) => {
        $crate::SignalID::from_string($name)
    };
}

#[macro_export]
macro_rules! tag {
    ($name:expr) => {
        $crate::TagID::from_string($name)
    };
}

#[macro_export]
macro_rules! tags {
    ($($name:literal),* $(,)?) => {{
        const __TAGS: &[$crate::NodeTag] = &[$($crate::NodeTag::borrowed($name)),*];
        __TAGS
    }};
    ($($name:expr),* $(,)?) => {
        &[$($crate::NodeTag::new($name)),*]
    };
}
