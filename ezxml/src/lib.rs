/*!
Half baked work on XML in Rust.

# TODO

 * [x] xml-struct: create the ezfile in a test using structs
 * [x] xml-struct: write assertions for the ezfile structs
 * [ ] xml-struct: serialize the ezfile to xml
 * [ ] xml-struct: assert serialized xml equals a string contstant of the xml
 * [ ] xml-struct: serialize the ezfile to json
 * [ ] xml-files: add the serialize ezfile data to the metadata file as an assertion.
 * [ ] ezxml: generate an assertion of the ezfile using build.rs
 * [ ] ezxml: make the parser work so that the ezfile test passes
*/

#[macro_use]
extern crate log;

pub use parser::parse_str;
pub use xdoc::{Document, ElementData, Node};

pub mod error;
mod parser;
