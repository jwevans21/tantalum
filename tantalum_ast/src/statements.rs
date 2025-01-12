use tantalum_span::Spanned;

use crate::{Expression, Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement<'file_name, 'source> {
    Block(#[cfg_attr(feature = "serde", serde(borrow))] Block<'file_name, 'source>),
    VariableDeclaration(VariableDeclaration<'file_name, 'source>),
    If(If<'file_name, 'source>),
    While(While<'file_name, 'source>),
    ForInitCondUpdate(ForInitCondUpdate<'file_name, 'source>),
    Break,
    Continue,
    Return(Return<'file_name, 'source>),
    Expression(Expression<'file_name, 'source>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub statements: Vec<Spanned<'file_name, Statement<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariableDeclaration<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Spanned<'file_name, &'source str>,
    pub ty: Option<Spanned<'file_name, Type<'file_name, 'source>>>,
    pub value: Spanned<'file_name, Expression<'file_name, 'source>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct If<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub condition: Spanned<'file_name, Expression<'file_name, 'source>>,
    pub body: Box<Spanned<'file_name, Statement<'file_name, 'source>>>,
    pub else_branch: Option<Box<Spanned<'file_name, Statement<'file_name, 'source>>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct While<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub condition: Spanned<'file_name, Expression<'file_name, 'source>>,
    pub body: Box<Spanned<'file_name, Statement<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForInitCondUpdate<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub init: Box<Spanned<'file_name, Statement<'file_name, 'source>>>,
    pub condition: Box<Spanned<'file_name, Statement<'file_name, 'source>>>,
    pub update: Box<Spanned<'file_name, Statement<'file_name, 'source>>>,
    pub body: Box<Spanned<'file_name, Statement<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Return<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Option<Spanned<'file_name, Expression<'file_name, 'source>>>,
}
