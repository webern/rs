use std::fs::read_to_string;
use std::path::PathBuf;

use crate::Metadata;

/// Represents a test file including paths to the test file and its metadata file.
#[derive(Debug, Clone)]
pub struct XmlFile {
    pub name: String,
    pub xml_path: PathBuf,
    pub metadata_path: PathBuf,
    pub metadata: Metadata,
}

impl XmlFile {
    pub fn read_xml_file(&self) -> String {
        read_to_string(&self.xml_path).unwrap()
    }
}