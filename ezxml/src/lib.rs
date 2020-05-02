/*!
Half baked work on XML in Rust.

# TODO

 * [x] xdoc: create the ezfile in a test using structs
 * [x] xdoc: write assertions for the ezfile structs
 * [ ] xdoc: serialize the ezfile to xml
 * [ ] xdoc: assert serialized xml equals a string contstant of the xml
 * [ ] xdoc: serialize the ezfile to json
 * [ ] xtest: add the serialize ezfile data to the metadata file as an assertion.
 * [ ] ezxml: generate an assertion of the ezfile using build.rs
 * [ ] ezxml: make the parser work so that the ezfile test passes
*/

#[macro_use]
extern crate log;

pub use parser::parse_str;
pub use xdoc::{Document, ElementData, Node};

pub mod error;
mod parser;
