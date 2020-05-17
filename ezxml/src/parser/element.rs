use std::str::Chars;

use snafu::{Backtrace, GenerateBacktrace};

use xdoc::{ElementData, Node, OrdMap};

use crate::error::{Error, Result};
use crate::parser::{Iter, parse_name, ParserState};
use crate::parser::chars::is_name_start_char;

pub(crate) fn parse_element(iter: &mut Iter) -> Result<ElementData> {
    iter.expect('<');
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

    // absorb whitespace
    iter.skip_whitespace()?;

    // check and return early if it is an empty, self-closing tag
    if iter.is('/') {
        println!("It is a self-closing tag with no attributes, i.e. an 'empty' element.");
        iter.advance_or_die();
        iter.expect('>')?;
        return Ok(element);
    }

    // now the only valid chars are '>' or the start of an attribute name
    if iter.is_name_start_char() {
        element.attributes = parse_attributes(iter)?;
    }

    // now the only valid char is '>' and we reach the child nodes
    iter.expect('>')?;
    iter.advance_or_die()?;
    parse_children(iter, &mut element)?;
    // TODO - expect to be pointing either at '>' or the iter is at the end
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

fn parse_attributes(iter: &mut Iter) -> Result<OrdMap> {
    // TODO - implement
    // skipping attributes for now
    loop {
        if iter.is('/') || iter.is('>') {
            break;
        }
        if !iter.advance() {
            break;
        }
    }
    Ok(OrdMap::new())
}

fn parse_children(iter: &mut Iter, parent: &mut ElementData) -> Result<()> {
    // TODO - support comments, processing instructions and whatever else
    loop {
        iter.skip_whitespace();
        if iter.is('<') {
            if iter.peek_is('/') {
                let end_tag_name = parse_end_tag_name(iter)?;
                if end_tag_name != parent.fullname() {
                    return Err(iter.err());
                }
            }
            let element = parse_element(iter)?;
            parent.nodes.push(Node::Element(element));
        } else {
            let text = parse_text(iter)?;
            parent.nodes.push(Node::String(text));
        }
        if !iter.advance() {
            break;
        }
    }
    Ok(())
}

fn parse_end_tag_name(iter: &mut Iter) -> Result<String> {
    iter.expect('<')?;
    iter.advance_or_die()?;
    iter.expect('/')?;
    iter.advance_or_die()?;
    iter.skip_whitespace();
    iter.expect_name_start_char()?;
    let mut name = String::default();
    name.push(iter.st.c);
    loop {
        iter.advance_or_die()?;
        if iter.is('>') {
            break;
        } else if iter.is_whitespace() {
            break;
        } else if iter.is_name_char() {
            name.push(iter.st.c);
        } else {
            return Err(iter.err());
        }
    }
    iter.skip_whitespace();
    iter.expect('>')?;
    Ok(name)
}

fn parse_text(iter: &mut Iter) -> Result<String> {
    Ok("".to_owned())
}