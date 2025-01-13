use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRLiteral(pub Rc<HLIRLiteralInner>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HLIRLiteralInner {
    Integer { value: String, radix: u32 },
    Float { value: String },
    Boolean { value: bool },
    Character { value: char },
    String { value: String },
}
