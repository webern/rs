#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::hash::Hash;
use std::io::Write;

pub use doc::Document;
pub use node::Node;
pub use nodes::Nodes;
pub use ord_map::OrdMap;

use crate::error::Result;

#[macro_use]
pub mod error;

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PIData {}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Attribute {
    key: String,
    value: String,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElementData {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: OrdMap,
    pub nodes: Vec<Node>,
}

const SMALLEST_ELEMENT: usize = 4; // <x/>

impl ElementData {
    pub fn to_writer<W>(&self, writer: &mut W) -> Result<()>
        where W: AsMut<dyn Write>, {
        let write_result = writer.as_mut().write(b"poo");
        if let Some(err) = write_result.err() {
            return wrap!(err);
        }
        let size = write_result.unwrap();
        if size < SMALLEST_ELEMENT {
            return raise!("Failed to successfully write element, too small.");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structs_test() {
        let mut _doc = Document::new();
    }
}
