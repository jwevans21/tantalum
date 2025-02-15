use crate::expressions::Expression;
use crate::types::TypeScopeId;
use crate::variables::{VariableId, VariableScopeBlockId};
use std::fmt::Formatter;

#[derive(Clone, PartialEq, Eq)]
pub enum Statement {
    Block(Block),
    Let(Let),
    If(If),
    While(While),
    Return(Return),
    Expression(Expression),
}

impl core::fmt::Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Block(block) => block.fmt(f),
            Statement::Let(let_) => let_.fmt(f),
            Statement::If(if_) => if_.fmt(f),
            Statement::While(while_) => while_.fmt(f),
            Statement::Return(return_) => return_.fmt(f),
            Statement::Expression(expression) => expression.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub variable_scope: VariableScopeBlockId,
    pub type_scope: TypeScopeId,
    pub statements: Vec<Statement>,
}

impl From<Block> for Statement {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    pub variable: VariableId,
    pub value: Expression,
}

impl From<Let> for Statement {
    fn from(variable_declaration: Let) -> Self {
        Self::Let(variable_declaration)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

impl From<If> for Statement {
    fn from(if_statement: If) -> Self {
        Self::If(if_statement)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While {
    pub condition: Expression,
    pub body: Box<Statement>,
}

impl From<While> for Statement {
    fn from(while_statement: While) -> Self {
        Self::While(while_statement)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Return {
    pub value: Option<Expression>,
}

impl Return {
    #[must_use]
    pub fn void() -> Self {
        Self { value: None }
    }
}

impl From<Return> for Statement {
    fn from(return_statement: Return) -> Self {
        Self::Return(return_statement)
    }
}

impl core::fmt::Debug for Return {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Return")
            .field_with("value", |f| match &self.value {
                None => self.value.fmt(f),
                Some(value) => value.fmt(f),
            })
            .finish()
    }
}
