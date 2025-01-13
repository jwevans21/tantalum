use std::rc::Rc;

use crate::HLIRFunctionPrototypeAnonymous;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HLIRType {
    Builtin(HLIRBuiltinType),

    Function(HLIRFunctionType),
    SizedArray(Rc<HLIRType>, usize),
    UnsizedArray(Rc<HLIRType>),
    Pointer(Rc<HLIRType>),
    Const(Rc<HLIRType>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HLIRBuiltinType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    Bool,
    Char,
    Str,
    Void,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRFunctionType {
    pub prototype: Rc<HLIRFunctionPrototypeAnonymous>,
}
