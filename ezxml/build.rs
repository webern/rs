// Automatically generate README.md from rustdoc.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

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
    let test_file_path = integ_test_dir().join("parse_tests.rs");
    let _ = std::fs::remove_file(&test_file_path);
    let mut f = std::fs::File::create(&test_file_path).unwrap();
    f.write(b"//! `parse_tests.rs` is generated by build.rs\n\n");

    for xml_file in xml_files.iter() {
        f.write(b"#[test]\n");
        let test_fn = format!("fn {}_test() {{\n", xml_file.name.replace("-", "_"));
        write!(f, "{}", test_fn);
        write!(
            f,
            "    let info = xml_files::get_test_info(\"{}\");\n",
            xml_file.name
        );
        write!(f, "    let f = info.open_xml_file();\n");
        f.write(b"}\n\n\n");
        f.write(b"");
    }

    // Command::new("cargo fmt")
    //     .args([""])
    //     .output()

    Command::new("cargo")
        .args(&["fmt", "--", test_file_path.to_str().unwrap()])
        .output()
        .expect("failed to execute process");
    // .expect("failed to execute process")
    // .stdout(Stdio::pipe())
    // .stderr(Stdio::from(stderr_file))
    // .spawn()
    // .context(error::CommandSpawn {
    //     command: command.clone(),
    // })?
    // .wait_with_output()
    // .context(error::CommandFinish {
    //     command: command.clone(),
    // })?;
}

fn integ_test_dir() -> PathBuf {
    let mycrate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    mycrate_dir.join("tests")
}
