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
