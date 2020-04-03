// Automatically generate README.md from rustdoc.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use xml_files;

fn main() {
    generate_readme();
    generate_tests();
}

fn generate_readme() {
    // Check for environment variable "SKIP_README". If it is set, skip README generation.
    if env::var_os("SKIP_README").is_some() {
        return;
    }

    let mut source = File::open("src/lib.rs").unwrap();
    let mut template = File::open("readme.template").unwrap();

    let content = cargo_readme::generate_readme(
        &PathBuf::from("."), // root
        &mut source,         // source
        Some(&mut template), // template
        // The "add x" arguments don't apply when using a template.
        true,  // add title
        false, // add badges
        false, // add license
        true,  // indent headings
    )
        .unwrap();

    let mut readme = File::create("README.md").unwrap();
    readme.write_all(content.as_bytes()).unwrap();
}

fn generate_tests() {
    // Check for environment variable "SKIP_TEST_GENERATION". If it is set, skip test generation.
    if env::var_os("SKIP_TEST_GENERATION").is_some() {
        return;
    }
    let xml_files = xml_files::list_test_files();

    for xml_file in xml_files.iter() {
        let test_file_path = integ_test_dir().join("parse_tests.rs");
        let _ = std::fs::remove_file(&test_file_path);
        std::fs::File::create(&test_file_path).unwrap();
    }
}

fn integ_test_dir() -> PathBuf {
    let mycrate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
    mycrate_dir.join("tests")
}
