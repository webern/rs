extern crate env_logger;

use std::iter::Peekable;
use std::str::Chars;

use snafu::{Backtrace, GenerateBacktrace};

pub use ds::Stack;
use xdoc::{Declaration, Document, ElementData, Encoding, PIData, Version};

use crate::error::{self, Result};
use crate::Node;
use crate::parser::chars::{is_name_char, is_name_start_char};
use crate::parser::element::parse_element;
use crate::parser::pi::parse_pi;

mod chars;
mod pi;
mod element;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
pub struct Position {
    pub line: u64,
    pub column: u64,
    pub absolute: u64,
}

impl Default for Position {
    fn default() -> Self {
        // These are the magic values needed to make the Position values 1-based.
        Position {
            line: 1,
            column: 1,
            absolute: 0, // this gets advanced when we start parsing (?)
        }
    }
}

impl Position {
    fn increment(&mut self, current_char: char) {
        self.absolute += 1;
        if current_char == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub(crate) struct XXXParserState {
    pub(crate) position: Position,
    pub(crate) c: char,
    pub(crate) doc_status: DocStatus,
    pub(crate) tag_status: TagStatus,
    pub(crate) stack: Option<Stack<crate::Node>>,
}

pub(crate) struct Iter<'a> {
    pub(crate) it: Peekable<Chars<'a>>,
    pub(crate) st: XXXParserState,
}

impl<'a> Iter<'a> {
    /// Returns an `Iter` primed with the first character, otherwise returns an error.
    fn new(s: &'a str) -> Result<Self> {
        let mut i = Iter {
            it: s.chars().peekable(),
            st: XXXParserState {
                position: Default::default(),
                c: 'x',
                doc_status: Default::default(),
                tag_status: Default::default(),
                stack: None,
            },
        };
        if !i.advance() {
            return Err(error::Error::Parse { position: Position::default(), backtrace: Backtrace::generate() });
        }
        Ok(i)
    }

    /// Returns `false` if the iterator could not be advanced (end).
    pub(crate) fn advance(&mut self) -> bool {
        let option_char = self.it.next();
        match option_char {
            Some(c) => {
                self.st.c = c;
                self.st.position.increment(self.st.c);
                true
            }
            None => false,
        }
    }
}

pub fn parse_str(s: &str) -> Result<Document> {
    // let mut state = XXXParserState {
    //     position: Default::default(),
    //     c: '\0',
    //     doc_status: DocStatus::default(),
    //     tag_status: TagStatus::OutsideTag,
    //     stack: None,
    // };
    //
    // let mut iter = s.chars();
    let mut iter = Iter::new(s)?;
    let mut document = Document::new();
    while iter.advance() {
        parse_document(&mut iter, &mut document)?;
        trace!("{:?}", iter.st);
    }

    Ok(document)
}

// TODO - disallow dead code
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
pub(crate) enum TagStatus {
    TagOpen(u64),
    InsideTag(u64),
    InsideProcessingInstruction(u64),
    TagClose(u64, u64),
    OutsideTag,
}

impl Default for TagStatus {
    fn default() -> Self {
        TagStatus::OutsideTag
    }
}

// TODO - disallow dead code
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
pub(crate) enum DocStatus {
    BeforeDeclaration,
    AfterDeclaration,
    BeforeRoot,
    ProcessingRoot,
    AfterRoot,
}

impl Default for DocStatus {
    fn default() -> Self {
        DocStatus::BeforeDeclaration
    }
}

fn parse_document(
    iter: &mut Iter,
    document: &mut Document,
) -> Result<()> {
    loop {
        if iter.st.c.is_ascii_whitespace() {
            if !iter.advance() {
                break;
            }
            continue;
        } else if iter.st.c != '<' {
            return Err(error::Error::Parse {
                position: iter.st.position,
                backtrace: Backtrace::generate(),
            });
        }
        let next = peek_or_die(iter)?;
        match next {
            '?' => {
// currently only one processing instruction is supported. no comments are
// supported. the xml declaration must either be the first thing in the document
// or else omitted.
                state_must_be_before_declaration(iter)?;
                advance_parser_or_die(iter)?;
                let pi_data = parse_pi(iter)?;
                document.declaration = parse_declaration(&pi_data)?;
                iter.st.doc_status = DocStatus::AfterDeclaration;
            }
            '-' => no_comments()?,
            _ => {
                document.root = parse_element(iter)?;
            }
        }

        if !iter.advance() {
            break;
        }
    }
    Ok(())
}

pub(crate) fn advance_parser_or_die(iter: &mut Iter<'_>) -> Result<()> {
    if iter.advance() {
        Ok(())
    } else {
        Err(error::Error::Parse {
            position: iter.st.position,
            backtrace: Backtrace::generate(),
        })
    }
}

fn parse_declaration(pi_data: &PIData) -> Result<Declaration> {
    let mut declaration = Declaration::default();
    if pi_data.target != "xml" {
        return Err(error::Error::Bug {
            message: "TODO - better message".to_string(),
        });
    }
    if pi_data.instructions.map().len() > 2 {
        return Err(error::Error::Bug {
            message: "TODO - better message".to_string(),
        });
    }
    if let Some(val) = pi_data.instructions.map().get("version") {
        match val.as_str() {
            "1.0" => {
                declaration.version = Version::One;
            }
            "1.1" => {
                declaration.version = Version::OneDotOne;
            }
            _ => {
                return Err(error::Error::Bug {
                    message: "TODO - better message".to_string(),
                });
            }
        }
    }
    if let Some(val) = pi_data.instructions.map().get("encoding") {
        match val.as_str() {
            "UTF-8" => {
                declaration.encoding = Encoding::Utf8;
            }
            _ => {
                return Err(error::Error::Bug {
                    message: "TODO - better message".to_string(),
                });
            }
        }
    }
    Ok(declaration)
}

fn state_must_be_before_declaration(iter: &Iter) -> Result<()> {
    if iter.st.doc_status != DocStatus::BeforeDeclaration {
        Err(error::Error::Bug {
            message: "TODO - better message".to_string(),
        })
    } else {
        Ok(())
    }
}

pub(crate) fn peek_or_die(iter: &mut Iter) -> Result<char> {
    let opt = iter.it.peek();
    match opt {
        Some(c) => Ok(*c),
        None => Err(error::Error::Bug {
            message: "TODO - better message".to_string(),
        }),
    }
}

fn no_comments() -> Result<()> {
    Err(error::Error::Bug {
        message: "comments are not supported".to_string(),
    })
}

fn parse_name(iter: &mut Iter) -> Result<String> {
    let mut name = String::default();
    if !is_name_start_char(iter.st.c) {
        return Err(error::Error::Parse {
            position: iter.st.position,
            backtrace: Backtrace::generate(),
        });
    }
    name.push(iter.st.c);
    loop {
        if iter.st.c.is_ascii_whitespace() {
            return Ok(name);
        }
        if iter.st.c == '=' {
            return Ok(name);
        }
        if !is_name_char(iter.st.c) {
            return Err(error::Error::Parse {
                position: iter.st.position,
                backtrace: Backtrace::generate(),
            });
        }
        name.push(iter.st.c);
        if !iter.advance() {
            return Ok(name);
        }
    }
    Ok(name)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    const XML1: &str = r##"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE something PUBLIC "-//Some//Path//EN" "http://www.example.org/dtds/partwise.dtd">
<cats>
  <cat id="b1">
    <name>
        Bones
    </name>
  <birthdate>2008-06-01</birthdate>
  </cat>
  <cat id="b2">
    <name>Bishop</name>
    <birthdate>2012-01-01</birthdate>
  </cat>
</cats>
    "##;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    // Check if a url with a trailing slash and one without trailing slash can both be parsed
    #[test]
    fn parse_a_doo_dah() {
        init_logger();
        let the_thing = XML1;
        let _ = parse_str(the_thing).unwrap();
    }
}
