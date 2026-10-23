use rx_ids::{NodeID, NodeTag, TagID};
use std::borrow::Cow;

use super::{NodeBaseDispatch, NodeType, NodeTypeDispatch, SceneNodeData, Spatial};

#[derive(Clone, Debug)]
pub struct SceneNode {
    pub data: SceneNodeData,
    pub id: NodeID,
    pub name: Cow<'static, str>,
    pub parent: NodeID,
    pub children: Vec<NodeID>,
    pub tags: Vec<NodeTag>,
}

impl SceneNode {
    pub fn new(data: SceneNodeData) -> Self {
        Self {
            id: NodeID::nil(),
            name: Cow::Borrowed("Node"),
            parent: NodeID::nil(),
            children: Vec::new(),
            tags: Vec::new(),
            data,
        }
    }

    pub const fn has_parent(&self) -> bool {
        !self.parent.is_nil()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn set_name<S>(&mut self, name: S)
    where
        S: Into<Cow<'static, str>>,
    {
        self.name = name.into();
    }

    pub const fn get_parent(&self) -> NodeID {
        self.parent
    }

    pub fn get_children_ids(&self) -> &[NodeID] {
        &self.children
    }

    pub fn set_children_ids<C>(&mut self, children: Option<C>)
    where
        C: Into<Vec<NodeID>>,
    {
        self.children = children.map(Into::into).unwrap_or_default();
    }

    pub fn get_tags(&self) -> &[NodeTag] {
        &self.tags
    }

    pub fn get_tag_ids(&self) -> Vec<TagID> {
        self.tags.iter().map(NodeTag::id).collect()
    }

    pub fn set_tag_ids<T>(&mut self, tags: Option<T>)
    where
        T: Into<Vec<TagID>>,
    {
        self.tags = tags
            .map(Into::into)
            .unwrap_or_default()
            .into_iter()
            .map(NodeTag::from)
            .collect();
    }

    pub fn set_tags<T>(&mut self, tags: Option<T>)
    where
        T: Into<Vec<NodeTag>>,
    {
        self.tags = tags.map(Into::into).unwrap_or_default();
    }

    pub const fn node_type(&self) -> NodeType {
        self.data.node_type()
    }

    pub const fn spatial(&self) -> Spatial {
        self.data.spatial()
    }

    pub const fn is_2d(&self) -> bool {
        matches!(self.spatial(), Spatial::TwoD)
    }

    pub const fn is_spatial(&self) -> bool {
        matches!(self.spatial(), Spatial::TwoD)
    }

    pub fn add_child(&mut self, child: NodeID) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: NodeID) {
        self.children.retain(|&c| c != child);
    }

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    pub fn children_slice(&self) -> &[NodeID] {
        self.get_children_ids()
    }

    pub fn add_tag<T>(&mut self, tag: T)
    where
        T: Into<NodeTag>,
    {
        self.tags.push(tag.into());
    }

    pub fn remove_tag(&mut self, tag: TagID) {
        self.tags.retain(|t| t.id != tag);
    }

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    pub fn has_tag(&self, tag: TagID) -> bool {
        self.tags.iter().any(|node_tag| node_tag.id == tag)
    }

    pub fn tags_slice(&self) -> &[NodeTag] {
        self.get_tags()
    }

    pub fn with_typed_ref<T: NodeTypeDispatch, R>(
        &self,
        f: impl FnOnce(&T) -> R,
    ) -> Option<R> {
        T::with_ref(&self.data, f)
    }

    pub fn with_typed_mut<T: NodeTypeDispatch, R>(
        &mut self,
        f: impl FnOnce(&mut T) -> R,
    ) -> Option<R> {
        T::with_mut(&mut self.data, f)
    }

    pub fn with_base_ref<T: NodeBaseDispatch, R>(
        &self,
        f: impl FnOnce(&T) -> R,
    ) -> Option<R> {
        T::with_base_ref(&self.data, f)
    }

    pub fn with_base_mut<T: NodeBaseDispatch, R>(
        &mut self,
        f: impl FnOnce(&mut T) -> R,
    ) -> Option<R> {
        T::with_base_mut(&mut self.data, f)
    }
}

impl From<SceneNodeData> for SceneNode {
    fn from(value: SceneNodeData) -> Self {
        SceneNode::new(value)
    }
}
