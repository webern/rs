use std::str::Chars;

use snafu::{Backtrace, GenerateBacktrace};

use xdoc::PIData;

use crate::error::{Error, Result};
use crate::parser::{advance_parser, ParserState};

use super::chars::{is_name_char, is_name_start_char};

#[derive(PartialEq)]
enum PIStatus {
    BeforeTarget,
    InsideTarget,
    AfterTarget,
    InsideKey,
    AfterKey,
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
    key_buffer: String,
    value_buffer: String,
    pi_data: PIData,
}

impl PIProcessor {
    fn new() -> Self {
        Self {
            status: PIStatus::BeforeTarget,
            key_buffer: "".to_string(),
            value_buffer: "".to_string(),
            pi_data: PIData::default(),
        }
    }

    /// Takes the current strings from `key_buffer` and `value_buffer` and adds them to the
    /// `instructions`. Clears these buffers to begin processing the next key/value pair.
    fn take_buffers(&mut self) -> Result<()> {
        if self.key_buffer.is_empty() {
            // TODO - better error
            return Err(Error::Bug { message: "Empty key - this is a bug and should have been detected sooner.".to_string() });
        }
        if self.pi_data.instructions.mut_map().insert(self.key_buffer.clone(), self.value_buffer.clone()).is_some() {
            // TODO - better error
            return Err(Error::Bug { message: "Duplicate key".to_string() });
        }
        self.key_buffer.clear();
        self.value_buffer.clear();
        Ok(())
    }
}

pub(crate) fn parse_pi(iter: &mut Chars, state: &mut ParserState) -> Result<PIData> {
    let mut processor = PIProcessor::new();
    loop {
        if let Err(e) = take_processing_instruction_char(iter, state, &mut processor) {
            return Err(e);
        }
        if processor.status == PIStatus::Close {
            break;
        }

        if !advance_parser(iter, state) {
            return Err(Error::Parse {
                position: state.position,
                backtrace: Backtrace::generate(),
            });
        }
    }

    Ok(processor.pi_data)
}

fn take_processing_instruction_char(_iter: &mut Chars, state: &mut ParserState, processor: &mut PIProcessor) -> Result<()> {
    let ch = state.current_char;
    println!("{}", ch);
    match processor.status {
        PIStatus::BeforeTarget => {
            if !is_name_start_char(state.current_char) {
                return Err(Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            } else {
                processor.pi_data.target.push(state.current_char);
                processor.status = PIStatus::InsideTarget;
            }
        }
        PIStatus::InsideTarget => {
            if state.current_char.is_ascii_whitespace() {
                processor.status = PIStatus::AfterTarget;
            } else if !is_name_char(state.current_char) {
                return Err(Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            } else {
                processor.pi_data.target.push(state.current_char);
            }
        }
        PIStatus::AfterTarget => {
            if is_name_start_char(state.current_char) {
                processor.key_buffer.push(state.current_char);
                processor.status = PIStatus::InsideKey;
            } else if !state.current_char.is_ascii_whitespace() {
                return Err(Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::InsideKey => {
            if is_name_char(state.current_char) {
                processor.key_buffer.push(state.current_char);
                processor.status = PIStatus::InsideKey;
            } else if state.current_char == '=' {
                processor.status = PIStatus::Equals;
            } else if state.current_char.is_ascii_whitespace() {
                processor.status = PIStatus::AfterKey;
            } else {
                return Err(Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::AfterKey => {
            if state.current_char == '=' {
                processor.status = PIStatus::Equals;
            } else if !state.current_char.is_ascii_whitespace() {
                return Err(Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::Equals | PIStatus::AfterEquals => {
            if state.current_char == '"' {
                processor.status = PIStatus::ValOpenQuote;
            } else if state.current_char.is_ascii_whitespace() {
                processor.status = PIStatus::AfterEquals;
            } else {
                return Err(Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::ValOpenQuote | PIStatus::InsideVal => {
            if state.current_char == '"' {
                processor.take_buffers()?;
                processor.status = PIStatus::ValCloseQuote;
            } else {
                // TODO - handle escape sequences
                processor.value_buffer.push(state.current_char);
                processor.status = PIStatus::InsideVal;
            }
        }
        PIStatus::ValCloseQuote => {
            if state.current_char.is_ascii_whitespace() {
                processor.status = PIStatus::AfterVal;
            } else if state.current_char == '?' {
                processor.status = PIStatus::QuestionMark;
            } else {
                return Err(Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::AfterVal => {
            if state.current_char.is_ascii_whitespace() {
                processor.status = PIStatus::AfterVal;
            } else if state.current_char == '?' {
                processor.status = PIStatus::QuestionMark;
            } else if is_name_start_char(state.current_char) {
                processor.key_buffer.push(state.current_char);
                processor.status = PIStatus::InsideKey;
            } else {
                return Err(Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::QuestionMark => {
            if state.current_char == '>' {
                processor.status = PIStatus::Close;
            } else {
                return Err(Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::Close => { /* done */ }
    }
    Ok(())
}