use core::cell::RefCell;
use std::rc::Rc;

use crate::{HLIRExpression, HLIRScopeBlock, HLIRType, ScopedValueIndex};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HLIRStatement {
    Block(HLIRBlock),
    VariableDeclaration(HLIRVariableDeclaration),
    If(HLIRIf),
    While(HLIRWhile),
    ForInitCondUpdate(HLIRForInitCondUpdate),
    Break,
    Continue,
    Return(HLIRReturn),
    Expression(HLIRExpression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRBlock {
    pub scope: Rc<RefCell<HLIRScopeBlock>>,
    pub statements: Vec<HLIRStatement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRVariableDeclaration {
    pub index: ScopedValueIndex,
    pub ty: Option<Rc<HLIRType>>,
    pub value: HLIRExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRIf {
    pub condition: HLIRExpression,
    pub body: Box<HLIRStatement>,
    pub else_branch: Option<Box<HLIRStatement>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRWhile {
    pub condition: HLIRExpression,
    pub body: Box<HLIRStatement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRForInitCondUpdate {
    pub init: Box<HLIRStatement>,
    pub condition: Box<HLIRStatement>,
    pub update: Box<HLIRStatement>,
    pub body: Box<HLIRStatement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRReturn {
    pub value: Option<HLIRExpression>,
}
