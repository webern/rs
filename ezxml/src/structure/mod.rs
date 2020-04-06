extern crate env_logger;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct ParserMetadata {}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub enum ElementContent {
    Empty,
    Text(String),
    Parent(Vec<Element>),
}

impl Default for ElementContent {
    fn default() -> Self {
        ElementContent::Empty
    }
}

pub struct Attribute {
    pub parser_metadata: ParserMetadata,
    pub namespace: Option<String>,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct Element {
    parser_metadata: ParserMetadata,
    namespace: Option<String>,
    name: String,
    content: ElementContent,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct Document {
    pub root: Element,
}

impl Document {
    pub fn new() -> Document {
        Document::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    // Check if a url with a trailing slash and one without trailing slash can both be parsed
    #[test]
    fn structs_test() {
        init_logger();
        let _doc = Document {
            // version: None,
            // encoding: None,
            root: Element {
                parser_metadata: ParserMetadata {},
                namespace: None,
                name: "the-root".into(),
                content: ElementContent::Parent(vec![
                    Element {
                        parser_metadata: ParserMetadata {},
                        namespace: Some("ns1".into()),
                        name: "a".into(),
                        content: ElementContent::Text("1".into()),
                    },
                    Element {
                        parser_metadata: ParserMetadata {},
                        namespace: None,
                        name: "b".into(),
                        content: ElementContent::Text("2".into()),
                    },
                    Element {
                        parser_metadata: ParserMetadata {},
                        namespace: None,
                        name: "c".into(),
                        content: ElementContent::Text("2".into()),
                    },
                ]),
            },
        };
    }
}
