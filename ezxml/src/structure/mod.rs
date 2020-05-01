extern crate env_logger;

use core::fmt;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};

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

#[derive(Debug, Eq, Hash)]
pub struct Nodes(VecDeque<Node>);

impl Clone for Nodes {
    fn clone(&self) -> Nodes {
        let mut result = VecDeque::new();
        for node in self.0.iter() {
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
            return ordering == Ordering::Less;
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Less || ordering == Ordering::Equal;
        }
        false
    }

    fn gt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater;
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater || ordering == Ordering::Equal;
        }
        false
    }
}

// TODO - extract key and value types
pub struct OrdMap(HashMap<String, String>);

impl Clone for OrdMap {
    fn clone(&self) -> Self {
        let mut result = HashMap::new();
        for (k, v) in self.0.iter() {
            result.insert(k.clone(), v.clone());
        }
        Self(result)
    }
}

impl Default for OrdMap {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl PartialEq for OrdMap {
    fn eq(&self, other: &Self) -> bool {
        if (self.0.len() != other.0.len()) {
            return false;
        }
        for (k, v) in self.0.iter() {
            if let Some(other_v) = other.0.get(k) {
                if other_v != v {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl Eq for OrdMap {}

impl OrdMap {
    pub fn map(&self) -> &HashMap<String, String> {
        return &self.0;
    }

    pub fn mut_map(&mut self) -> &mut HashMap<String, String> {
        return &mut self.0;
    }

    fn size_le(&self, other: &Self) -> bool {
        self.0.len() < other.0.len()
    }

    fn size_gt(&self, other: &Self) -> bool {
        self.0.len() > other.0.len()
    }
}

impl PartialOrd for OrdMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.size_le(other)) {
            return Some(Ordering::Less);
        } else if (self.size_gt(other)) {
            return Some(Ordering::Greater);
        }
        for (k, v) in self.0.iter() {
            if let Some(other_v) = other.0.get(v.as_str()) {
                if v < other_v {
                    return Some(Ordering::Less);
                } else if v > other_v {
                    return Some(Ordering::Greater);
                }
            } else {
                // we will define the hash map that first has a value that the other one doesn't as
                // being 'larger'.
                return Some(Ordering::Greater);
            }
        }
        Some(Ordering::Equal)
    }

    fn lt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Less;
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Less || ordering == Ordering::Equal;
        }
        false
    }

    fn gt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater;
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater || ordering == Ordering::Equal;
        }
        false
    }
}

impl fmt::Debug for OrdMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Hash for OrdMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.0.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct ElementData {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: OrdMap,
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

    // <!DOCTYPE doc> Contents are a blob
    DocType(String),
}

impl Default for Node {
    fn default() -> Self {
        Node::Element(ElementData::default())
    }
}

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
        let mut _doc = Document::new();
        // let mut root = doc.root();
        // let mut root_content = root.mut_content();
    }
}
