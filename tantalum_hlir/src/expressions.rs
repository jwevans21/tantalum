use std::rc::Rc;

use crate::{HLIRLiteral, HLIRType, ScopedValueIndex};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HLIRExpression {
    Variable(ScopedValueIndex),
    Literal(HLIRLiteral),
    FunctionCall(HLIRFunctionCall),
    MemberAccess(HLIRMemberAccess),
    Index(HLIRIndex),
    UnaryOperation(HLIRUnaryOperation),
    BinaryOperation(HLIRBinaryOperation),
    TypeCast(HLIRTypeCast),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRFunctionCall {
    pub function: Box<HLIRExpression>,
    pub arguments: Vec<HLIRExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRMemberAccess {
    pub object: Box<HLIRExpression>,
    pub member: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRIndex {
    pub object: Box<HLIRExpression>,
    pub index: Box<HLIRExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRUnaryOperation {
    pub operator: HLIRUnaryOperator,
    pub operand: Box<HLIRExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRBinaryOperation {
    pub left: Box<HLIRExpression>,
    pub operator: HLIRBinaryOperator,
    pub right: Box<HLIRExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRTypeCast {
    pub expression: Box<HLIRExpression>,
    pub ty: Rc<HLIRType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HLIRUnaryOperator {
    Negation,
    LogicalNegation,
    BitwiseNegation,
    Deref,
    Ref,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HLIRBinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,

    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,

    Assignment,
}
