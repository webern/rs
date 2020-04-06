/*!
Half baked work on XML in Rust.
*/
#[macro_use]
extern crate log;

pub use parser::parse_str;
pub use structure::ElementData;
pub use structure::Node;

pub mod error;
mod parser;
mod structure;
