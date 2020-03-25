extern crate env_logger;

use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::string::ParseError;

use snafu::{Backtrace, GenerateBacktrace, ResultExt};

use crate::error::{self, Result, Error};
use crate::structure;
use crate::structure::{Element, ElementContent, ParserMetadata};
use crate::parser::DocState::BeforeFirstTag;
use std::str::Chars;

// mod error;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
enum DocState {
    BeforeFirstTag,
    XmlDeclaration,
    Doctype,
    RootElement,
}

impl Default for DocState {
    fn default() -> Self { DocState::BeforeFirstTag }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
enum Processing {
    Unknown,
    DocStart,
    XmlDeclaration,
    DoctypeDeclaration,
    RootElement,
    // Tag,
    // TagNameOrNamespace,
    // TagName,
    // DoneProcessingNamespace,
    // DoneProcessingTagName,
    // AttributeName,
    // AttributeOpenQuotes,
    // AttributeValue,
    // Processing
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
enum GrammarItem {
    ElementOpen,
    ElementClose,
    ElementSelfClosing,
    Attribute,
    TextData,
    ProcessingOpen,
    ProcessingClose,
    ProcessingData,
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
enum CharItem {
    OpenAngle,
    ClosingSlash,
    ClosingAngle,
    IgnorableSpace,
    TextData,
    NamespaceColon,
    AttributeEquals,
    AttributeOpenQuotes,
    AttributeCloseQuotes,
}

const BUFF_SIZE: usize = 1024;

pub fn parse<R: BufRead>(r: &mut R) -> error::Result<structure::Document> {
    let mut s = String::new();
    let _ = r.read_to_string(&mut s).context(error::IoRead {
        parse_location: error::ParseLocation { line: 0, column: 0 },
    })?;
    parse_str(&s)
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash, Default)]
struct Position {
    line: u64,
    column: u64,
    absolute: u64,
}

impl Position {
    fn increment(&mut self, current_char: &char) {
        self.absolute += 1;
        if current_char == &'\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
}

// Comparison traits: Eq, PartialEq, Ord, PartialOrd.
// Clone, to create T from &T via a copy.
// Copy, to give a type 'copy semantics' instead of 'move semantics'.
// Hash, to compute a hash from &T.
// Default, to create an empty instance of a data type.
// Debug, to format a value using the {:?} formatter.

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct ParserState {
    pub position: Position,
    pub doc_state: DocState,
    pub current_char: char,
}

pub fn parse_str(s: &str) -> error::Result<structure::Document> {
    let mut state = ParserState {
        position: Default::default(),
        doc_state: DocState::BeforeFirstTag,
        current_char: '\0',
    };

    let mut iter = s.chars();
    while let Some(c) = iter.next() {
        state.current_char = c;
        process_current(&mut iter, &mut state)?;
    }

    Ok(structure::Document {
        version: None,
        encoding: None,
        root: structure::Element {
            parser_metadata: ParserMetadata {},
            namespace: None,
            name: "x".to_string(),
            content: ElementContent::Empty,
        },
    })
}

fn process_current(iter: &mut Chars, state: &mut ParserState) -> Result<()> {
    trace!("{:?}", state);
    let c = state.current_char.unwrap();
    state.position.absolute += 1;
    advance_state_position(c, state);
    match state.doc_state {
        DocState::RootElement => {},
        BeforeFirstTag => {
            if c.is_ascii_whitespace() {
                // Keep advancing until we get to the first tag.
                return Ok(())
            } else if c == '<' {

            } else {
                return Err(Error::Parse{ parser_state: state.into(), backtrace: Backtrace::generate() })
            }
        },
        DocState::XmlDeclaration => {},
        DocState::Doctype => {},
    }
    Ok(())
}

fn advance_state_position(current: char, state: &mut ParserState) {
    if current == '\n' {
        state.position.line += 1;
        state.position.column = 0;
    } else {
        state.position.column += 1;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

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

    use super::*;

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