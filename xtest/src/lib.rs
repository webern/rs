//! `xtest` provides a set of XML files for testing. Each file comes with a manifest (in JSON
//! format) which describes the XML file. For example, some XML files include intentional syntax
//! errors, and the the accompanying JSON manifest will make this apparent.

#[macro_use]
extern crate serde;

use std::fs;
use std::fs::{File, read_to_string};
use std::io::BufReader;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub use {io::load, io::load_all};
pub use metadata::Metadata;
pub use xml_file::{Syntax, XmlFile};

mod io;
mod metadata;
mod xml_file;

