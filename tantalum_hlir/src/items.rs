use core::cell::RefCell;
use std::rc::Rc;

use crate::{scope::HLIRScopeBlock, HLIRStatement, HLIRType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRFunctionPrototype {
    pub name: String,
    pub inner: Rc<HLIRFunctionPrototypeAnonymous>,
    pub parameter_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRFunctionPrototypeAnonymous {
    pub parameters: Vec<Rc<HLIRType>>,
    pub is_variadic: bool,
    pub return_type: Rc<HLIRType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRFunction {
    pub prototype: Rc<HLIRFunctionPrototype>,
    pub scope: Rc<RefCell<HLIRScopeBlock>>,
    pub body: HLIRStatement,
}
