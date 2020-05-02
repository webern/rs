use std::default::Default;

use crate::Node;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct Document {
    pub root: Node,
}

impl Document {
    pub fn new() -> Document {
        Document::default()
    }

    pub fn root(&self) -> &Node {
        return &self.root;
    }
}
