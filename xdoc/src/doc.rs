use std::default::Default;
use std::io::Write;

use crate::error::Result;
use crate::Node;

// const SMALLEST_POSSIBLE_XML_FILE: u64 = 4; // <x/>

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

    pub fn to_writer<W>(&self, _writer: W) -> Result<()>
        where
            W: AsMut<dyn Write>, {
        if let Node::Element(_e) = self.root() {
            // return e.to_writer(writer);
            return raise!("not implemented");
        } else {
            return raise!("the root is not a node of element type.");
        }
    }
}

#[macro_export]
macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[cfg(test)]
mod tests {
    use crate::*;

    fn assert_ezfile(doc: &Document) {
        let root = doc.root();
        if let Node::Element(root_data) = root {
            assert_eq!(root_data.name, "cats");
            assert_eq!(root_data.namespace, None);
            assert_eq!(root_data.attributes.map().len(), 0);
            assert_eq!(root_data.nodes.len(), 2);
            let bones_element = root_data.nodes.get(0).unwrap();
            if let Node::Element(bones) = bones_element {
                assert_eq!(bones.name, "cat");
                assert_eq!(bones.namespace, None);
                assert_eq!(bones.attributes.map().len(), 1);
                assert_eq!(bones.nodes.len(), 0);
                let name = bones.attributes.map().get("name").unwrap();
                assert_eq!(name, "bones");
            } else {
                panic!("bones was supposed to be an element but was not");
            }
            let bishop_element = root_data.nodes.get(1).unwrap();
            if let Node::Element(bishop) = bishop_element {
                assert_eq!(bishop.name, "cat");
                assert_eq!(bishop.namespace, None);
                assert_eq!(bishop.attributes.map().len(), 1);
                assert_eq!(bishop.nodes.len(), 0);
                let name = bishop.attributes.map().get("name").unwrap();
                assert_eq!(name, "bishop");
            } else {
                panic!("bones was supposed to be an element but was not");
            }
        } else {
            panic!("the root was not an element");
        }
    }

    fn create_ezfile() -> Document {
        let bones_data = ElementData {
            namespace: None,
            name: "cat".to_string(),
            attributes: OrdMap::from(map! { "name".to_string() => "bones".to_string() }),
            nodes: Vec::default(),
        };

        let bishop_data = ElementData {
            namespace: None,
            name: "cat".to_string(),
            attributes: OrdMap::from(map! { "name".to_string() => "bishop".to_string() }),
            nodes: Vec::default(),
        };

        let bones_element = Node::Element(bones_data);
        let bishop_element = Node::Element(bishop_data);

        let cats_data = ElementData {
            namespace: None,
            name: "cats".to_string(),
            attributes: Default::default(),
            nodes: vec![bones_element, bishop_element],
        };

        Document {
            root: Node::Element(cats_data),
        }
    }

    #[test]
    fn test_ezfile_create() {
        let ezfile = create_ezfile();
        assert_ezfile(&ezfile);
    }
}
