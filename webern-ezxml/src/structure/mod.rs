extern crate env_logger;

pub struct ParserMetadata {}

pub enum ElementContent {
    Empty,
    Text(String),
    Parent(Vec<Element>),
}

pub struct Namespace {
    namespace: String,
}

pub struct Attribute {
    parser_metadata: ParserMetadata,
    namespace: Option<String>,
    name: String,
    value: String,
}

pub struct Element {
    parser_metadata: ParserMetadata,
    namespace: Option<String>,
    name: String,
    content: ElementContent,
}

pub enum Location {
    BeforeElement,
    AfterElement,
}

pub struct ProcessingInstruction {
    parser_metadata: ParserMetadata,
    location: Location,
    target: String,
    data: String,
}

pub enum XmlVersion {
    Version10,
    Version11,
}

pub enum Encoding {
    UTF8,
}

pub struct Document {
    version: Option<XmlVersion>,
    encoding: Option<Encoding>,
    root: Element,
}

pub fn max_element_depth(element: &Element) -> u64 {
    match &element.content {
        ElementContent::Parent(children) => {
            let mut max_found: u64 = 0;
            children.iter().for_each(|item| {
                let curr = max_element_depth(item);
                if curr > max_found {
                    info!("curr > max_found: {}, {}", curr, max_found);
                    max_found = curr;
                }
            });
            return max_found + 1;
        }
        _ => {}
    }
    1
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
        let doc = Document {
            version: None,
            encoding: None,
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
                    }
                ]),
            },
        };

        let max_depth = max_element_depth(&doc.root);
        assert_eq!(max_depth, 2);
    }
}