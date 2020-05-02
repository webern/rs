/*!
Half baked work on XML in Rust.
*/
#[macro_use]
extern crate log;

pub use parser::parse_str;
pub use xml_struct::{Document, ElementData, Node};

pub mod error;
mod parser;
