extern crate env_logger;

use std::io::prelude::*;
use std::str::Chars;

use snafu::{Backtrace, GenerateBacktrace, ResultExt};

use xdoc::{Document, OrdMap};

use crate::error::{self, Result};
use crate::parser::TagStatus::OutsideTag;

// Comparison traits: Eq, PartialEq, Ord, PartialOrd.
// Clone, to create T from &T via a copy.
// Copy, to give a type 'copy semantics' instead of 'move semantics'.
// Hash, to compute a hash from &T.
// Default, to create an empty instance of a data type.
// Debug, to format a value using the {:?} formatter.
// #[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]

const _BUFF_SIZE: usize = 1024;

pub fn _parse<R: BufRead>(r: &mut R) -> error::Result<Document> {
    let mut s = String::new();
    let _ = r.read_to_string(&mut s).context(error::IoRead {
        parse_location: error::ParseLocation { line: 0, column: 0 },
    })?;
    parse_str(&s)
}

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

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash, Default)]
struct ParserState {
    position: Position,
    // doc_state: DocState,
    current_char: char,
    tag_status: TagStatus,
}

pub fn parse_str(s: &str) -> Result<Document> {
    let mut state = ParserState {
        position: Default::default(),
        // doc_state: DocState::BeforeFirstTag,
        current_char: '\0',
        tag_status: OutsideTag,
    };

    let mut iter = s.chars();
    while advance_parser(&mut iter, &mut state) {
        let _state = format!("{:?}", state);
        process_char(&mut iter, &mut state)?;
        trace!("{:?}", state);
    }

    Ok(Document::new())
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
enum TagStatus {
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

fn is_space_or_alpha(c: char) -> bool {
    c.is_alphabetic() || c.is_ascii_whitespace()
}

fn is_pi_indicator(c: char) -> bool {
    c == '?'
}

fn process_char(iter: &mut Chars, state: &mut ParserState) -> Result<()> {
    let _state_str = format!("{:?}", state);
    match state.tag_status {
        TagStatus::TagOpen(pos) => {
            if state.current_char != '/'
                && !is_space_or_alpha(state.current_char)
                && !is_pi_indicator(state.current_char)
            {
                return Err(error::Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            } else if is_pi_indicator(state.current_char) {
                state.tag_status = TagStatus::InsideProcessingInstruction(pos);
                if advance_parser(iter, state) {
                    take_processing_instruction(iter, state);
                } else {
                    return Err(error::Error::Parse {
                        position: state.position,
                        backtrace: Backtrace::generate(),
                    });
                }
            } else {
                state.tag_status = TagStatus::InsideTag(pos)
            }
        }
        TagStatus::InsideTag(pos) => {
            if state.current_char == '>' {
                state.tag_status = TagStatus::TagClose(pos, state.position.absolute)
            } else if state.current_char == '<' {
                return Err(error::Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        TagStatus::InsideProcessingInstruction(pos) => {}
        TagStatus::TagClose(_start, _end) => {
            if state.current_char == '<' {
                state.tag_status = TagStatus::TagOpen(state.position.absolute);
            } else if state.current_char == '>' {
                return Err(error::Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            } else {
                state.tag_status = TagStatus::OutsideTag;
            }
            // TODO pop the start and stop locations over to a tag parser?
        }
        OutsideTag => {
            if state.current_char == '<' {
                state.tag_status = TagStatus::TagOpen(state.position.absolute);
            } else if state.current_char == '>' {
                return Err(error::Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
    }
    Ok(())
}

#[derive(PartialEq)]
enum PIStatus {
    BeforeTarget,
    InsideTarget,
    AfterTarget,
    KeyOpenQuote,
    InsideKey,
    KeyCloseQuote,
    AfterKeyCloseQuote,
    Equals,
    AfterEquals,
    ValOpenQuote,
    InsideVal,
    ValCloseQuote,
    AfterVal,
    QuestionMark,
    Close,
}

struct PIProcessor {
    status: PIStatus,
    buffer: String,
    target: String,
    instructions: OrdMap,
    // done: bool,
}

impl PIProcessor {
    fn new() -> Self {
        Self {
            status: PIStatus::BeforeTarget,
            buffer: "".to_string(),
            target: "".to_string(),
            instructions: Default::default(),
            // done: false,
        }
    }
}

fn take_processing_instruction(iter: &mut Chars, state: &mut ParserState) -> Result<()> {
    let mut processor = PIProcessor::new();
    loop {
        if processor.status == PIStatus::Close {
            break;
        }
        if let Err(e) = take_processing_instruction_char(iter, state, &mut processor) {
            return Err(e);
        }

        // advance state here?

        // if processor.done {
        //     return Ok(());
        // }

        if !advance_parser(iter, state) {
            return Err(error::Error::Parse {
                position: state.position,
                backtrace: Backtrace::generate(),
            });
        }
    }

    Ok(())
}

// for valid name start char ranges
const U_00C0: char = '\u{00C0}';
const U_00D6: char = '\u{00D6}';
const U_00D8: char = '\u{00D8}';
const U_00F6: char = '\u{00F6}';
const U_00F8: char = '\u{00F8}';
const U_02FF: char = '\u{02FF}';
const U_0370: char = '\u{0370}';
const U_037D: char = '\u{037D}';
const U_037F: char = '\u{037F}';
const U_1FFF: char = '\u{1FFF}';
const U_200C: char = '\u{200C}';
const U_200D: char = '\u{200D}';
const U_2070: char = '\u{2070}';
const U_218F: char = '\u{218F}';
const U_2C00: char = '\u{2C00}';
const U_2FEF: char = '\u{2FEF}';
const U_3001: char = '\u{3001}';
const U_D7FF: char = '\u{D7FF}';
const U_F900: char = '\u{F900}';
const U_FDCF: char = '\u{FDCF}';
const U_FDF0: char = '\u{FDF0}';
const U_FFFD: char = '\u{FFFD}';
const U_10000: char = '\u{10000}';
const U_EFFFF: char = '\u{EFFFF}';

// for valid name char ranges
const U_00B7: char = '\u{00B7}';
const U_0300: char = '\u{0300}';
const U_036F: char = '\u{036F}';
const U_203F: char = '\u{203F}';
const U_2040: char = '\u{2040}';


fn take_processing_instruction_char(iter: &mut Chars, state: &mut ParserState, processor: &mut PIProcessor) -> Result<()> {
    match processor.status {
        PIStatus::BeforeTarget => {
            if !state.current_char.is_alphabetic() {
                let c = state.current_char;

                return Err(error::Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            } else if state.current_char.is_ascii_whitespace() {
                return Ok(());
            } else {
                let x = U_EFFFF as u32;
                println!("{}", x);
                processor.target.push(state.current_char);
            }
        }
        PIStatus::InsideTarget => {}
        PIStatus::AfterTarget => {}
        PIStatus::KeyOpenQuote => {}
        PIStatus::InsideKey => {}
        PIStatus::KeyCloseQuote => {}
        PIStatus::AfterKeyCloseQuote => {}
        PIStatus::Equals => {}
        PIStatus::AfterEquals => {}
        PIStatus::ValOpenQuote => {}
        PIStatus::InsideVal => {}
        PIStatus::ValCloseQuote => {}
        PIStatus::AfterVal => {}
        PIStatus::QuestionMark => {}
        PIStatus::Close => {}
    }
    Ok(())
}

// is there really no built-in function?
// fn is_space(c: char) -> bool {
//     c == ' ' || c == '\t' || c == '\n'
// }

fn advance_parser(iter: &mut Chars<'_>, state: &mut ParserState) -> bool {
    let option_char = iter.next();
    match option_char {
        Some(c) => {
            state.current_char = c;
            state.position.increment(state.current_char);
            true
        }
        None => false,
    }
}

// const Z: u32 = 0xEFFFF;
// // const X: char = char::from(Z);
// const U_EFFFF: char = '󯿿';


fn is_name_start_char(c: char) -> bool {
// https://www.w3.org/TR/2008/REC-xml-20081126/#NT-NameStartChar
// [4]   	NameStartChar	   ::=   	":" | [A-Z] | "_" | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] |
// [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] |
// [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
// let x = c as u64;
    match c {
        'A'..='Z' => true,
        'a'..='z' => true,
        ':' => true,
        '_' => true,
        U_00C0..=U_00D6 => true,
        U_00D8..=U_00F6 => true,
        U_00F8..=U_02FF => true,
        U_0370..=U_037D => true,
        U_037F..=U_1FFF => true,
        U_200C..=U_200D => true,
        U_2070..=U_218F => true,
        U_2C00..=U_2FEF => true,
        U_3001..=U_D7FF => true,
        U_F900..=U_FDCF => true,
        U_FDF0..=U_FFFD => true,
        U_10000..=U_EFFFF => true,
        _ => false,
    }
}

fn is_name_char(c: char) -> bool {
// https://www.w3.org/TR/2008/REC-xml-20081126/#NT-NameChar
// [4a] NameChar ::= NameStartChar | "-" | "." | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
    if is_name_start_char(c) {
        return true;
    }
    match c {
        U_00B7 => true,
        U_0300..=U_036F => true,
        U_203F..=U_2040 => true,
        '0'..='9' => true,
        '-' => true,
        '.' => true,
        _ => false,
    }
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
