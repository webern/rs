use xml_files::{Syntax, TestMetadata};

fn main() {
    let t = TestMetadata {
        description: "the things are described here".to_string(),
        syntax: Syntax::Bad {
            character_position: 21,
            line: 2,
            column: 10,
        },
    };

    println!("{}", serde_json::to_string_pretty(&t).unwrap());
}