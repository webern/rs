use std::default::Default;
use std::io::Write;

use crate::error::Result;
use crate::Node;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct Document {
    pub root: Node,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub enum Indent {
    None,
    Spaces(usize),
    Tab,
}

impl Default for Indent {
    fn default() -> Self {
        Indent::Spaces(2)
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub enum Newline {
    None,
    Newline,
    Windows,
}

impl Default for Newline {
    fn default() -> Self {
        Newline::Newline
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct WriteOpts {
    pub indent: Indent,
    pub newline: Newline,
}

impl WriteOpts {
    fn newline_str(&self) -> &'static str {
        match self.newline {
            Newline::None => "",
            Newline::Newline => "\n",
            Newline::Windows => "\n\r",
        }
    }

    fn write_repeatedly<W>(writer: &mut W, num: usize, s: &str) -> Result<()>
        where
            W: Write, {
        let s = std::iter::repeat(s).take(num).collect::<String>();
        if let Err(e) = write!(writer, "{}", s) { return wrap!(e); }
        Ok(())
    }

    pub(crate) fn indent<W>(&self, writer: &mut W, depth: usize) -> Result<()>
        where
            W: Write, {
        match self.indent {
            Indent::None => { return Ok(()); }
            Indent::Spaces(n) => {
                if let Err(e) = Self::write_repeatedly(writer, depth * n, " ") {
                    return wrap!(e);
                }
            }
            Indent::Tab => {
                if let Err(e) = Self::write_repeatedly(writer, depth, "\t") {
                    return wrap!(e);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn newline<W>(&self, writer: &mut W) -> Result<()>
        where
            W: Write, {
        if let Err(e) = write!(writer, "{}", self.newline_str()) {
            return wrap!(e);
        }
        Ok(())
    }
}

impl Document {
    pub fn new() -> Document {
        Document::default()
    }

    pub fn root(&self) -> &Node {
        return &self.root;
    }

    pub fn write<W>(&self, writer: &mut W) -> Result<()>
        where
            W: Write, {
        return self.write_opts(writer, &WriteOpts::default());
    }

    pub fn write_opts<W>(&self, writer: &mut W, opts: &WriteOpts) -> Result<()>
        where
            W: Write, {
        if let Node::Element(e) = self.root() {
            return e.write(writer, opts, 0);
        } else {
            return raise!("the root is not a node of element type.");
        }
    }
}

#[macro_export]
macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::*;

    fn assert_ezfile(doc: &Document) {
        let root = doc.root();
        if let Node::Element(root_data) = root {
            assert_eq!(root_data.name, "cats");
            assert_eq!(root_data.namespace, None);
            assert_eq!(root_data.attributes.map().len(), 0);
            assert_eq!(root_data.nodes.len(), 2);
            let bones_element = root_data.nodes.get(0).unwrap();
            if let Node::Element(bones) = bones_element {
                assert_eq!(bones.name, "cat");
                assert_eq!(bones.namespace, None);
                assert_eq!(bones.attributes.map().len(), 1);
                assert_eq!(bones.nodes.len(), 0);
                let name = bones.attributes.map().get("name").unwrap();
                assert_eq!(name, "bones");
            } else {
                panic!("bones was supposed to be an element but was not");
            }
            let bishop_element = root_data.nodes.get(1).unwrap();
            if let Node::Element(bishop) = bishop_element {
                assert_eq!(bishop.name, "cat");
                assert_eq!(bishop.namespace, None);
                assert_eq!(bishop.attributes.map().len(), 1);
                assert_eq!(bishop.nodes.len(), 0);
                let name = bishop.attributes.map().get("name").unwrap();
                assert_eq!(name, "bishop");
            } else {
                panic!("bones was supposed to be an element but was not");
            }
        } else {
            panic!("the root was not an element");
        }
    }

    fn create_ezfile() -> Document {
        let bones_data = ElementData {
            namespace: None,
            name: "cat".to_string(),
            attributes: OrdMap::from(map! { "name".to_string() => "bones".to_string() }),
            nodes: Vec::default(),
        };

        let bishop_data = ElementData {
            namespace: None,
            name: "cat".to_string(),
            attributes: OrdMap::from(map! { "name".to_string() => "bishop".to_string() }),
            nodes: Vec::default(),
        };

        let bones_element = Node::Element(bones_data);
        let bishop_element = Node::Element(bishop_data);

        let cats_data = ElementData {
            namespace: None,
            name: "cats".to_string(),
            attributes: Default::default(),
            nodes: vec![bones_element, bishop_element],
        };

        Document {
            root: Node::Element(cats_data),
        }
    }

    #[test]
    fn test_ezfile_create() {
        let ezfile = create_ezfile();
        assert_ezfile(&ezfile);
    }

    #[test]
    fn test_ezfile_to_string() {
        let doc = create_ezfile();
        let mut c = Cursor::new(Vec::new());
        let result = doc.write(&mut c);
        assert!(result.is_ok());
        let data = c.into_inner();
        let data_str = std::str::from_utf8(data.as_slice()).unwrap();
        let we = std::fs::write("/Users/mjb/Desktop/early.xml", data_str);
        assert!(we.is_ok());
        assert_eq!(data_str, "");
    }
}
