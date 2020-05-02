#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::collections::VecDeque;
use std::hash::Hash;

pub use doc::Document;
pub use node::Node;
pub use nodes::Nodes;
pub use ord_map::OrdMap;

mod doc;
mod node;
mod nodes;
mod ord_map;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Name {
    pub namespace: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct PIData {}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
struct Attribute {
    key: String,
    value: String,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct ElementData {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: OrdMap,
    pub nodes: VecDeque<Node>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structs_test() {
        let mut _doc = Document::new();
    }
}
