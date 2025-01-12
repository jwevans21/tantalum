use tantalum_span::Spanned;

use crate::{Literal, Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression<'file_name, 'source> {
    Variable(#[cfg_attr(feature = "serde", serde(borrow))] Variable<'file_name, 'source>),
    Literal(Literal<'file_name, 'source>),
    FunctionCall(FunctionCall<'file_name, 'source>),
    MemberAccess(MemberAccess<'file_name, 'source>),
    Index(Index<'file_name, 'source>),
    UnaryOperation(UnaryOperation<'file_name, 'source>),
    BinaryOperation(BinaryOperation<'file_name, 'source>),
    TypeCast(TypeCast<'file_name, 'source>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variable<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Spanned<'file_name, &'source str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionCall<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub function: Box<Spanned<'file_name, Expression<'file_name, 'source>>>,
    pub arguments: Vec<Spanned<'file_name, Expression<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MemberAccess<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub object: Box<Spanned<'file_name, Expression<'file_name, 'source>>>,
    pub member: Spanned<'file_name, &'source str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Index<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub object: Box<Spanned<'file_name, Expression<'file_name, 'source>>>,
    pub index: Box<Spanned<'file_name, Expression<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnaryOperation<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub operator: Spanned<'file_name, UnaryOperator>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub operand: Box<Spanned<'file_name, Expression<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryOperation<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub left: Box<Spanned<'file_name, Expression<'file_name, 'source>>>,
    pub operator: Spanned<'file_name, BinaryOperator>,
    pub right: Box<Spanned<'file_name, Expression<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnaryOperator {
    Negation,
    LogicalNegation,
    BitwiseNegation,
    Deref,
    Ref,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BinaryOperator {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeCast<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ty: Spanned<'file_name, Type<'file_name, 'source>>,
    pub value: Box<Spanned<'file_name, Expression<'file_name, 'source>>>,
}
