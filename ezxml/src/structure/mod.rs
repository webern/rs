extern crate env_logger;

use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct Name {
    pub namespace: Option<String>,
    pub name: String,
}

// #[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
// pub struct Element {
//     namespace: Option<String>,
//     name: String,
//     content: ElementContent,
// }
//
// impl Element {
//     pub fn name<'a>(&'a self) -> &'a str {
//         self.name.as_str()
//     }
//
//     pub fn content<'a>(&'a self) -> &'a ElementContent {
//         &self.content
//     }
//
//     pub fn mut_content<'a>(&'a mut self) -> &'a mut ElementContent {
//         &mut self.content
//     }
//
//     pub fn text<'a>(&'a self) -> Option<&'a str> {
//         if let ElementContent::Text(s) = &self.content {
//             return Some(s.as_str());
//         }
//         None
//     }
// }

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
struct PIData {}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
struct Attribute {
    key: String,
    value: String,
}

#[derive(Debug, Eq, Hash)]
pub struct Nodes(VecDeque<Node>);

impl Clone for Nodes {
    fn clone(&self) -> Nodes {
        let mut result = VecDeque::new();
        for &node in self.0.iter() {
            result.push_back(node.clone())
        }
        Nodes(result)
    }
}

impl Default for Nodes {
    fn default() -> Nodes {
        Nodes(VecDeque::new())
    }
}

impl PartialEq for Nodes {
    fn eq(&self, other: &Self) -> bool {
        if (self.0.len() != other.0.len()) {
            return false;
        }
        for i in 0..self.0.len() {
            if (self.0.get(i).unwrap().eq(other.0.get(i).unwrap())) {
                return false;
            }
        }
        true
    }
}

impl Nodes {
    fn size_le(&self, other: &Self) -> bool {
        self.0.len() < other.0.len()
    }

    fn size_gt(&self, other: &Self) -> bool {
        self.0.len() > other.0.len()
    }
}

impl PartialOrd for Nodes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.size_le(other)) {
            return Some(Ordering::Less);
        } else if (self.size_gt(other)) {
            return Some(Ordering::Greater);
        }
        for i in 0..self.0.len() {
            if (self.0.get(i).unwrap() < other.0.get(i).unwrap()) {
                return Some(Ordering::Less);
            } else if (self.0.get(i).unwrap() > other.0.get(i).unwrap()) {
                return Some(Ordering::Greater);
            }
        }
        Some(Ordering::Equal)
    }

    fn lt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering == Ordering::Less
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering == Ordering::Less || ordering == Ordering::Equal
        }
        false
    }

    fn gt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering == Ordering::Greater
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering == Ordering::Greater || ordering == Ordering::Equal
        }
        false
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct ElementData {
    pub namespace: Option<String>,
    pub name: String,
    // TODO - introduce Attribute type
    pub attributes: HashMap<String, String>,
    pub nodes: VecDeque<Node>,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub enum Node {
    // <element>
    Element(ElementData),

    // normal text data, i.e. 'text &lt;'
    String(String),

    // <![CDATA[text]]>
    CData(String),

    // <!-- comment -->
    Comment(String),

    // <?target data1 data2 data3?>'
    ProcessingInstruction(PIData),

    // <!DOCTYPE doc> Contents are as a blob
    DocType(String), // yuck
}

impl Default for Node {
    fn default() -> Self {
        Node::Element(ElementData::default())
    }
}

// impl Clone for Node {
//     fn clone(&self) -> Self {
//         match self {
//             Node::Element(element_data) => Node::Element(element_data.clone()),
//             Node::String(s) => Node::String(s.clone()),
//             Node::CData(s) => Node::CData(s.clone()),
//             Node::Comment(s) => Node::Comment(s.clone()),
//             Node::ProcessingInstruction(pi_data) => Node::ProcessingInstruction(pi_data.clone()),
//             Node::DocType(s) => Node::DocType(s.clone()),
//         }
//     }
// }

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct Document {
    pub root: Node,
}

impl Document {
    pub fn new() -> Document {
        Document::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    // Check if a url with a trailing slash and one without trailing slash can both be parsed
    #[test]
    fn structs_test() {
        init_logger();
        let mut doc = Document::new();
        let mut root = doc.mut_root();
        let mut root_content = root.mut_content();
    }
}
