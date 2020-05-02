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

    // fn init_logger() {
    //     let _ = env_logger::builder().is_test(true).try_init();
    // }

    // Check if a url with a trailing slash and one without trailing slash can both be parsed
    #[test]
    fn structs_test() {
        // init_logger();
        let mut _doc = Document::new();
        // let mut root = doc.root();
        // let mut root_content = root.mut_content();
    }
}
