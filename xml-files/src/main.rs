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

    let x = TestMetadata {
        description: "x".to_string(),
        syntax: Syntax::Good {},
    };

    println!("{}", serde_json::to_string_pretty(&x).unwrap());
}