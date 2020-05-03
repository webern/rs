use std::io::Write;

use crate::doc::WriteOpts;
use crate::error::Result;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Node {
    // <element>
    Element(crate::ElementData),

    // normal text data, i.e. 'text &lt;'
    String(String),

    // <![CDATA[text]]>
    CData(String),

    // <!-- comment -->
    Comment(String),

    // <?target data1 data2 data3?>'
    ProcessingInstruction(crate::PIData),

    // <!DOCTYPE doc> Contents are a blob
    DocType(String),
}

impl Default for Node {
    fn default() -> Self {
        Node::Element(crate::ElementData::default())
    }
}

impl Node {
    pub fn write<W>(&self, writer: &mut W, opts: &WriteOpts, depth: usize) -> Result<()>
        where W: Write, {
        match self {
            Node::Element(data) => { return data.write(writer, opts, depth); }
            Node::String(s) => {
                // TODO - escape string
                if let Err(e) = write!(writer, "{}", s) {
                    return wrap!(e);
                }
                return Ok(());
            }
            Node::CData(_) => { return Ok(());/*TODO - implement*/ }
            Node::Comment(_) => { return Ok(());/*TODO - implement*/ }
            Node::ProcessingInstruction(_) => { return Ok(());/*TODO - implement*/ }
            Node::DocType(_) => { return Ok(()); /*TODO - implement*/ }
        }
        Ok(())
    }
}