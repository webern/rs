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

    // check and return early if it is an empty, self-closing tag that had attributes
    if iter.is('/') {
        println!("It is a self-closing tag with no attributes, i.e. an 'empty' element.");
        iter.advance_or_die();
        iter.expect('>')?;
        return Ok(element);
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
    let mut attributes = OrdMap::new();
    loop {
        iter.skip_whitespace();
        if iter.is('/') || iter.is('>') {
            break;
        }
        let mut key = String::default();
        if iter.is_name_start_char() {
            key = parse_name(iter)?;
        }
        iter.skip_whitespace();
        iter.expect('=')?;
        iter.advance_or_die()?;
        iter.skip_whitespace();
        iter.expect('"')?;
        iter.advance_or_die()?;
        let value = parse_attribute_value(iter)?;
        iter.expect('"')?;
        attributes.mut_map().insert(key, value);
        if !iter.advance() {
            break;
        }
    }
    Ok(attributes)
}

fn parse_attribute_value(iter: &mut Iter) -> Result<String> {
    let mut result = String::new();
    loop {
        if iter.is('"') {
            break;
        }
        // TODO - handle escapes
        result.push(iter.st.c);
        iter.advance_or_die()?;
    }
    Ok(result)
}

fn parse_children(iter: &mut Iter, parent: &mut ElementData) -> Result<()> {
    // TODO - support comments, processing instructions and whatever else
    loop {
        iter.skip_whitespace();
        if iter.is('<') {
            if let Some(node) = handle_left_angle(iter, parent)? {
                parent.nodes.push(node);
            } else {
                // we received 'None' which means that the end tag was parsed
                return Ok(());
            }
        } else {
            let text = parse_text(iter)?;
            parent.nodes.push(Node::String(text));
            if iter.is('<') {
                if let Some(node) = handle_left_angle(iter, parent)? {
                    parent.nodes.push(node);
                } else {
                    // we received 'None' which means that the end tag was parsed
                    return Ok(());
                }
            }
        }
        if !iter.advance() {
            break;
        }
    }
    Ok(())
}

fn handle_left_angle(iter: &mut Iter, parent: &mut ElementData) -> Result<Option<Node>> {
    if iter.peek_is('/') {
        let end_tag_name = parse_end_tag_name(iter)?;
        if end_tag_name != parent.fullname() {
            return Err(iter.err(file!(), line!()));
        }
        // return None to signal that we have parsed and end tag
        return Ok(None);
    }
    let element = parse_element(iter)?;
    Ok(Some(Node::Element(element)))
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
            return Err(iter.err(file!(), line!()));
        }
    }
    iter.skip_whitespace();
    iter.expect('>')?;
    Ok(name)
}

fn parse_text(iter: &mut Iter) -> Result<String> {
    let mut result = String::new();
    loop {
        if iter.is('<') {
            break;
        }
        // TODO - handle escapes
        result.push(iter.st.c);
        iter.advance_or_die()?;
    }
    Ok(result)
}