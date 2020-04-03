use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use test_dir;

fn get_files() -> Vec<PathBuf> {
    let mut result = Vec::new();
    let dir = test_dir::xml_syntax_errors();
    let entries = fs::read_dir(dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        // check that it's a file
        let file_type = entry.file_type().unwrap();
        if !file_type.is_file() {
            continue;
        }

        result.push(path);
    }
    result
}

pub struct TestXmlFile {
    pub xml: PathBuf,
    pub metadata: PathBuf,
}

pub fn get_xml_files(listing: &Vec<PathBuf>) -> Vec<TestXmlFile> {
    let mut map: HashMap<String, TestXmlFile> = HashMap::new();
    for x in listing.iter() {}
    Vec::new()
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

#[test]
fn test() {
    let files = get_files();
}