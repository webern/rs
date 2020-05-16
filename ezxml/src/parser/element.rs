use std::str::Chars;

use snafu::{Backtrace, GenerateBacktrace};

use xdoc::ElementData;

use crate::error::{Error, Result};
use crate::parser::chars::is_name_start_char;
use crate::parser::{parse_name, Iter, ParserState};

pub(crate) fn parse_element(iter: &mut Iter) -> Result<ElementData> {
    // it is required that the input be the opening '<'
    if iter.st.c != '<' {
        return Err(Error::Bug {
            message: "Bad string cannot be split".to_string(),
        });
    }

    // advance one character to the first position inside the element tag
    iter.advance_or_die()?;

    // ignore whitespace before the element name
    loop {
        if !iter.st.c.is_ascii_whitespace() {
            break;
        }
        iter.advance_or_die()?;
    }

    let name = parse_name(iter)?;
    let mut element = make_named_element(name.as_str())?;

    // TODO - implement
    while iter.advance() {}
    Ok(element)
}

fn split_element_name(input: &str) -> Result<(&str, &str)> {
    let split: Vec<&str> = input.split(':').collect();
    match split.len() {
        1 => return Ok(("", split.first().unwrap())),
        2 => return Ok((split.first().unwrap(), split.last().unwrap())),
        _ => Err(Error::Bug {
            message: "Bad string cannot be split".to_string(),
        }),
    }
}

fn make_named_element(input: &str) -> Result<ElementData> {
    let split = split_element_name(input)?;
    Ok(ElementData {
        namespace: match split.0 {
            "" => None,
            _ => Some(split.0.to_owned()),
        },
        name: split.1.to_string(),
        attributes: Default::default(),
        nodes: vec![],
    })
}
