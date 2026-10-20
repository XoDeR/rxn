use std::borrow::Cow;

use crate::{generational::define_generational, hash::string_to_u64};

define_generational!(
    TagID,
    "Tag ID - deterministic ID from tag name. Used for scene node tags and queries."
);

impl TagID {
    // Deterministic ID from tag name. Uses hash; generation 0.
    pub const fn from_string(s: &str) -> Self {
        Self::from_u64(string_to_u64(s))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeTag {
    pub id: TagID,
    pub name: Cow<'static, str>,
}

impl NodeTag {
    pub const fn borrowed(name: &'static str) -> Self {
        Self {
            id: TagID::from_string(name),
            name: Cow::Borrowed(name),
        }
    }

    pub fn new<S>(name: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        let name = name.into();
        Self {
            id: TagID::from_string(name.as_ref()),
            name,
        }
    }

    pub const fn id(&self) -> TagID {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl From<TagID> for NodeTag {
    fn from(id: TagID) -> Self {
        Self {
            id,
            name: Cow::Borrowed(""),
        }
    }
}

impl From<&TagID> for NodeTag {
    fn from(id: &TagID) -> Self {
        (*id).into()
    }
}

impl From<&str> for NodeTag {
    fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
}

impl From<String> for NodeTag {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<&String> for NodeTag {
    fn from(name: &String) -> Self {
        Self::new(name.clone())
    }
}

pub trait IntoTagID {
    fn into_tag_id(self) -> TagID;
}

impl IntoTagID for TagID {
    #[inline]
    fn into_tag_id(self) -> TagID {
        self
    }
}

impl IntoTagID for &TagID {
    #[inline]
    fn into_tag_id(self) -> TagID {
        *self
    }
}

impl IntoTagID for NodeTag {
    #[inline]
    fn into_tag_id(self) -> TagID {
        self.id
    }
}

impl IntoTagID for &NodeTag {
    #[inline]
    fn into_tag_id(self) -> TagID {
        self.id
    }
}

impl IntoTagID for &str {
    #[inline]
    fn into_tag_id(self) -> TagID {
        TagID::from_string(self)
    }
}

impl IntoTagID for String {
    #[inline]
    fn into_tag_id(self) -> TagID {
        TagID::from_string(self.as_str())
    }
}

impl IntoTagID for &String {
    #[inline]
    fn into_tag_id(self) -> TagID {
        TagID::from_string(self.as_str())
    }
}
