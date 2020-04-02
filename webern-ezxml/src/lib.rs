#[macro_use]
extern crate log;

mod error;
mod parser;
mod structure;

pub use structure::Attribute;
pub use structure::Element;
pub use structure::ElementContent;
pub use structure::Namespace;
