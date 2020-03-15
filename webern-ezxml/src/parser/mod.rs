extern crate env_logger;

use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::string::ParseError;

use snafu::{Backtrace, GenerateBacktrace, ResultExt};

use crate::error::{self, Result};
use crate::structure;
use crate::structure::{Element, ElementContent, ParserMetadata};

// mod error;

#[derive(Debug)]
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

#[derive(Debug)]
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
struct ParserState {
    position: Position,
}

pub fn parse_str(s: &str) -> error::Result<structure::Document> {
    let mut state = ParserState { position: Default::default() };
    for c in s.chars() {
        trace!("{}: {:?}", c, state);
        state.position.absolute += 1;
        if c == '\n' {
            state.position.line += 1;
            state.position.column = 0;
        } else {
            state.position.column += 1;
        }
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    // Check if a url with a trailing slash and one without trailing slash can both be parsed
    #[test]
    fn parse_a_doo_dah() {
        init_logger();
        let the_thing = "Hello World!\nMy Name is Bones.\nI am a cat!";
        let _ = parse_str(the_thing).unwrap();
    }
}