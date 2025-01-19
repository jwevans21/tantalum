use crate::inference::InferenceId;
use crate::literals::Literal;
use crate::variables::VariableId;
use crate::{HLIRPackage, TypeId};
use crate::functions::FunctionId;

/// The expressions used in HLIR
///
/// They will contain a [`TypeId`] that is the result of the expression.
///
/// [`TypeId`]: crate::types::TypeId
#[derive(Clone, PartialEq, Eq)]
pub enum Expression {
    Variable(VariableId),
    Literal(Literal),
    FunctionCall(FunctionCall),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    TypeCast(TypeCast),
}

impl Expression {
    /// Get the type of the expression
    ///
    /// # Panics
    ///
    /// Panics if the variable does not exist
    #[must_use]
    pub fn ty(&self, package: &HLIRPackage) -> InferenceId {
        match self {
            Expression::Variable(variable_id) => package
                .variables
                .get_type(*variable_id)
                .expect("expected variable to exist"),
            Expression::Literal(literal) => literal.ty(),
            Expression::FunctionCall(function_call) => function_call.result,
            Expression::UnaryOperation(unary_operation) => unary_operation.result,
            Expression::BinaryOperation(binary_operation) => binary_operation.result,
            Expression::TypeCast(type_cast) => type_cast.target_type,
        }
    }
}

impl core::fmt::Debug for Expression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Expression::Variable(variable_id) => variable_id.fmt(f),
            Expression::Literal(literal) => literal.fmt(f),
            Expression::FunctionCall(function_call) => function_call.fmt(f),
            Expression::UnaryOperation(unary_operation) => unary_operation.fmt(f),
            Expression::BinaryOperation(binary_operation) => binary_operation.fmt(f),
            Expression::TypeCast(type_cast) => type_cast.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub function: FunctionId,
    pub arguments: Vec<Expression>,
    pub result: InferenceId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
    pub result: InferenceId,
}

impl From<UnaryOperation> for Expression {
    fn from(unary_operation: UnaryOperation) -> Self {
        Self::UnaryOperation(unary_operation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negation,
    BitwiseNot,
    LogicalNot,
    Deref,
    Ref,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
    pub result: InferenceId,
}

impl From<BinaryOperation> for Expression {
    fn from(binary_operation: BinaryOperation) -> Self {
        Self::BinaryOperation(binary_operation)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,

    LogicalAnd,
    LogicalOr,

    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeCast {
    pub expression: Box<Expression>,
    pub target_type: InferenceId,
}
