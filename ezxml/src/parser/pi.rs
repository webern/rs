use std::str::Chars;

use snafu::{Backtrace, GenerateBacktrace};

use xdoc::PIData;

use crate::error::{Error, Result};
use crate::parser::{Iter, ParserState};

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
            return Err(Error::Bug {
                message: "Empty key - this is a bug and should have been detected sooner."
                    .to_string(),
            });
        }
        if self
            .pi_data
            .instructions
            .mut_map()
            .insert(self.key_buffer.clone(), self.value_buffer.clone())
            .is_some()
        {
            // TODO - better error
            return Err(Error::Bug {
                message: "Duplicate key".to_string(),
            });
        }
        self.key_buffer.clear();
        self.value_buffer.clear();
        Ok(())
    }
}

pub(crate) fn parse_pi(iter: &mut Iter) -> Result<PIData> {
    let mut processor = PIProcessor::new();
    loop {
        if let Err(e) = take_processing_instruction_char(iter, &mut processor) {
            return Err(e);
        }
        if processor.status == PIStatus::Close {
            break;
        }

        if !iter.advance() {
            return Err(Error::Parse {
                position: iter.st.position,
                backtrace: Backtrace::generate(),
            });
        }
    }

    Ok(processor.pi_data)
}

fn take_processing_instruction_char(
    iter: &mut Iter,
    processor: &mut PIProcessor,
) -> Result<()> {
    let ch = iter.st.c;
    println!("{}", ch);
    match processor.status {
        PIStatus::BeforeTarget => {
            if !is_name_start_char(iter.st.c) {
                return Err(Error::Parse {
                    position: iter.st.position,
                    backtrace: Backtrace::generate(),
                });
            } else {
                processor.pi_data.target.push(iter.st.c);
                processor.status = PIStatus::InsideTarget;
            }
        }
        PIStatus::InsideTarget => {
            if iter.st.c.is_ascii_whitespace() {
                processor.status = PIStatus::AfterTarget;
            } else if !is_name_char(iter.st.c) {
                return Err(Error::Parse {
                    position: iter.st.position,
                    backtrace: Backtrace::generate(),
                });
            } else {
                processor.pi_data.target.push(iter.st.c);
            }
        }
        PIStatus::AfterTarget => {
            if is_name_start_char(iter.st.c) {
                processor.key_buffer.push(iter.st.c);
                processor.status = PIStatus::InsideKey;
            } else if !iter.st.c.is_ascii_whitespace() {
                return Err(Error::Parse {
                    position: iter.st.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::InsideKey => {
            if is_name_char(iter.st.c) {
                processor.key_buffer.push(iter.st.c);
                processor.status = PIStatus::InsideKey;
            } else if iter.st.c == '=' {
                processor.status = PIStatus::Equals;
            } else if iter.st.c.is_ascii_whitespace() {
                processor.status = PIStatus::AfterKey;
            } else {
                return Err(Error::Parse {
                    position: iter.st.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::AfterKey => {
            if iter.st.c == '=' {
                processor.status = PIStatus::Equals;
            } else if !iter.st.c.is_ascii_whitespace() {
                return Err(Error::Parse {
                    position: iter.st.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::Equals | PIStatus::AfterEquals => {
            if iter.st.c == '"' {
                processor.status = PIStatus::ValOpenQuote;
            } else if iter.st.c.is_ascii_whitespace() {
                processor.status = PIStatus::AfterEquals;
            } else {
                return Err(Error::Parse {
                    position: iter.st.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::ValOpenQuote | PIStatus::InsideVal => {
            if iter.st.c == '"' {
                processor.take_buffers()?;
                processor.status = PIStatus::ValCloseQuote;
            } else {
                // TODO - handle escape sequences
                processor.value_buffer.push(iter.st.c);
                processor.status = PIStatus::InsideVal;
            }
        }
        PIStatus::ValCloseQuote => {
            if iter.st.c.is_ascii_whitespace() {
                processor.status = PIStatus::AfterVal;
            } else if iter.st.c == '?' {
                processor.status = PIStatus::QuestionMark;
            } else {
                return Err(Error::Parse {
                    position: iter.st.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::AfterVal => {
            if iter.st.c.is_ascii_whitespace() {
                processor.status = PIStatus::AfterVal;
            } else if iter.st.c == '?' {
                processor.status = PIStatus::QuestionMark;
            } else if is_name_start_char(iter.st.c) {
                processor.key_buffer.push(iter.st.c);
                processor.status = PIStatus::InsideKey;
            } else {
                return Err(Error::Parse {
                    position: iter.st.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::QuestionMark => {
            if iter.st.c == '>' {
                processor.status = PIStatus::Close;
            } else {
                return Err(Error::Parse {
                    position: iter.st.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        PIStatus::Close => { /* done */ }
    }
    Ok(())
}
