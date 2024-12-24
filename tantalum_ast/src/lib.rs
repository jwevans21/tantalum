//! # Tantalum Abstract Syntax Tree
//!
//! Provides the abstract syntax tree for the Tantalum language.

use tantalum_span::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Expression<'file_name, 'source> {
    /// The source code range that this expression covers
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub span: Span<'file_name>,
    /// The kind of expression that was found
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub kind: ExpressionKind<'file_name, 'source>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionKind<'file_name, 'source> {
    Variable {
        /// The name of the variable
        name: &'source str,
    },
    Path {
        components: Vec<PathComponent<'file_name, 'source>>,
    },
    IntegerLiteral {
        value: &'source str,
        radix: u32,
    },
    FloatLiteral {
        value: &'source str,
    },
    StringLiteral {
        value: &'source str,
    },
    CharacterLiteral {
        value: &'source str,
    },
    BooleanLiteral {
        value: bool,
    },
    FunctionCall {
        /// The function being called, which can be a variable, a member access, or the result of another function call
        source: Box<Expression<'file_name, 'source>>,
        /// The arguments to the function
        #[cfg_attr(feature = "serde", serde(borrow))]
        arguments: Vec<Expression<'file_name, 'source>>,
    },
    MemberAccess {
        /// The source of the member access, which can be a variable, a member access, or the result of another function call
        source: Box<Expression<'file_name, 'source>>,
        /// The name of the member being accessed
        member: &'source str,
    },
    ArrayAccess {
        /// The source of the array access, which can be a variable, a member access, or the result of another function call
        source: Box<Expression<'file_name, 'source>>,
        /// The index being accessed
        #[cfg_attr(feature = "serde", serde(borrow))]
        index: Box<Expression<'file_name, 'source>>,
    },
    UnaryOperation {
        /// The operator being applied
        operator: UnaryOperator<'file_name>,
        /// The operand of the operator
        #[cfg_attr(feature = "serde", serde(borrow))]
        operand: Box<Expression<'file_name, 'source>>,
    },
    BinaryOperation {
        /// The left-hand side of the operation
        #[cfg_attr(feature = "serde", serde(borrow))]
        left: Box<Expression<'file_name, 'source>>,
        /// The operator being applied
        operator: BinaryOperator<'file_name>,
        /// The right-hand side of the operation
        #[cfg_attr(feature = "serde", serde(borrow))]
        right: Box<Expression<'file_name, 'source>>,
    },
    TypeCast {
        /// The type being cast to
        #[cfg_attr(feature = "serde", serde(borrow))]
        ty: Type<'file_name, 'source>,
        /// The expression being cast
        #[cfg_attr(feature = "serde", serde(borrow))]
        expression: Box<Expression<'file_name, 'source>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PathComponent<'file_name, 'source> {
    /// The source code range that this path component covers
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub span: Span<'file_name>,
    /// The name of the component
    pub name: &'source str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnaryOperator<'file_name> {
    /// The source code range that this operator covers
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub span: Span<'file_name>,
    /// The kind of operator that was found
    pub kind: UnaryOperatorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnaryOperatorKind {
    Negation,
    LogicalNegation,
    BitwiseNegation,
    Deref,
    AddressOf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryOperator<'file_name> {
    /// The source code range that this operator covers
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub span: Span<'file_name>,
    /// The kind of operator that was found
    pub kind: BinaryOperatorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BinaryOperatorKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    LogicalAnd,
    LogicalOr,
    Equality,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    Assignment,

    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Statement<'file_name, 'source> {
    /// The source code range that this statement covers
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub span: Span<'file_name>,
    /// The kind of statement that was found
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub kind: StatementKind<'file_name, 'source>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementKind<'file_name, 'source> {
    /// A block of statements
    ///
    /// ```tantalum
    /// {
    ///     let x = 5;
    ///     let y = 10;
    /// }
    /// ```
    Block {
        /// The statements in the block
        #[cfg_attr(feature = "serde", serde(borrow))]
        statements: Vec<Statement<'file_name, 'source>>,
    },
    /// A variable declaration
    ///
    /// ```tantalum
    /// let x = 5;
    /// ```
    VariableDeclaration {
        /// The name of the variable
        name: &'source str,
        /// The type of the variable
        #[cfg_attr(feature = "serde", serde(borrow))]
        ty: Option<Type<'file_name, 'source>>,
        /// The value of the variable
        #[cfg_attr(feature = "serde", serde(borrow))]
        value: Expression<'file_name, 'source>,
    },
    /// An if statement
    ///
    /// ```tantalum
    /// if x == 5 {
    ///    printf("x is 5\n");
    /// }
    /// ```
    If {
        /// The condition of the if statement
        #[cfg_attr(feature = "serde", serde(borrow))]
        condition: Expression<'file_name, 'source>,
        /// The body of the if statement
        #[cfg_attr(feature = "serde", serde(borrow))]
        body: Box<Statement<'file_name, 'source>>,
        /// The else branch of the if statement
        #[cfg_attr(feature = "serde", serde(borrow))]
        else_branch: Option<Box<Statement<'file_name, 'source>>>,
    },
    /// A while loop
    ///
    /// ```tantalum
    /// while x < 5 {
    ///   x = x + 1;
    /// }
    /// ```
    While {
        /// The condition of the while loop
        #[cfg_attr(feature = "serde", serde(borrow))]
        condition: Expression<'file_name, 'source>,
        /// The body of the while loop
        #[cfg_attr(feature = "serde", serde(borrow))]
        body: Box<Statement<'file_name, 'source>>,
    },
    /// A for loop with an initialization, condition, and update
    ///
    /// ```tantalum
    /// for (let i = 0; i < 5; i = i + 1) {
    ///  printf("i is %d\n", i);
    /// }
    /// ```
    ForInitCondUpdate {
        /// The initialization of the for loop
        #[cfg_attr(feature = "serde", serde(borrow))]
        initialization: Box<Statement<'file_name, 'source>>,
        /// The condition of the for loop
        #[cfg_attr(feature = "serde", serde(borrow))]
        condition: Box<Statement<'file_name, 'source>>,
        /// The update of the for loop
        #[cfg_attr(feature = "serde", serde(borrow))]
        update: Box<Statement<'file_name, 'source>>,
        /// The body of the for loop
        #[cfg_attr(feature = "serde", serde(borrow))]
        body: Box<Statement<'file_name, 'source>>,
    },
    /// A break statement
    ///
    /// ```tantalum
    /// break;
    /// ```
    Break,
    /// A continue statement
    ///
    /// ```tantalum
    /// continue;
    /// ```
    Continue,
    /// A return statement
    ///
    /// ```tantalum
    /// return 5;
    /// ```
    ///
    /// or
    ///
    /// ```tantalum
    /// return;
    /// ```
    Return {
        /// The value to return
        #[cfg_attr(feature = "serde", serde(borrow))]
        value: Option<Expression<'file_name, 'source>>,
    },
    /// An expression statement
    ///
    /// ```tantalum
    /// x = 5;
    /// ```
    Expression {
        /// The expression to evaluate
        #[cfg_attr(feature = "serde", serde(borrow))]
        expression: Expression<'file_name, 'source>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TopLevelExpression<'file_name, 'source> {
    /// The source code range that this top-level expression covers
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub span: Span<'file_name>,
    /// The type of top-level expression that was found
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub kind: TopLevelExpressionKind<'file_name, 'source>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TopLevelExpressionKind<'file_name, 'source> {
    ExternalFunction {
        /// The name of the function
        name: &'source str,
        /// The parameters of the function
        #[cfg_attr(feature = "serde", serde(borrow))]
        parameters: Vec<Parameter<'file_name, 'source>>,
        /// The return type of the function
        return_type: Type<'file_name, 'source>,
        /// Whether the function is variadic
        is_variadic: bool,
    },
    FunctionDeclaration {
        /// The name of the function
        name: &'source str,
        /// The parameters of the function
        #[cfg_attr(feature = "serde", serde(borrow))]
        parameters: Vec<Parameter<'file_name, 'source>>,
        /// The return type of the function
        return_type: Type<'file_name, 'source>,
        /// The body of the function
        #[cfg_attr(feature = "serde", serde(borrow))]
        body: Statement<'file_name, 'source>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter<'file_name, 'source> {
    /// The source code range that this parameter covers
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub span: Span<'file_name>,
    /// The kind of parameter that was found
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub kind: ParameterKind<'file_name, 'source>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParameterKind<'file_name, 'source> {
    /// A named parameter
    Named {
        /// The name of the parameter
        name: &'source str,
        /// The type of the parameter
        #[cfg_attr(feature = "serde", serde(borrow))]
        ty: Type<'file_name, 'source>,
    },
    /// An unnamed parameter, only allowed in external function declarations
    Unnamed {
        /// The type of the parameter
        #[cfg_attr(feature = "serde", serde(borrow))]
        ty: Type<'file_name, 'source>,
    },
}

/// A type in the Tantalum language
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Type<'file_name, 'source> {
    /// The source code range that this type covers
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub span: Span<'file_name>,
    /// The kind of type that was found
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub kind: TypeKind<'file_name, 'source>,
}

/// The kind of type that was found
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeKind<'file_name, 'source> {
    /// A function type
    ///
    /// ```tantalum
    /// fn(i32, i32): i32 // A function that takes two i32s and returns an i32
    /// ```
    Function {
        /// The parameters of the function
        #[cfg_attr(feature = "serde", serde(borrow))]
        parameters: Vec<Type<'file_name, 'source>>,
        /// The return type of the function
        #[cfg_attr(feature = "serde", serde(borrow))]
        return_type: Box<Type<'file_name, 'source>>,
        /// Whether the function is variadic
        is_variadic: bool,
    },
    /// An array of a fixed size
    ///
    /// ```tantalum
    /// [i32; 5] // An array of 5 i32s
    /// ```
    SizedArray {
        /// The type of the elements in the array
        #[cfg_attr(feature = "serde", serde(borrow))]
        element_type: Box<Type<'file_name, 'source>>,
        /// The size of the array
        size: usize,
    },
    /// An array of an unknown size
    ///
    /// ```tantalum
    /// [i32] // An array of i32s
    /// ```
    UnsizedArray(#[cfg_attr(feature = "serde", serde(borrow))] Box<Type<'file_name, 'source>>),
    /// A pointer to a type
    ///
    /// ```tantalum
    /// *i32 // A pointer to an i32
    /// ```
    Pointer(#[cfg_attr(feature = "serde", serde(borrow))] Box<Type<'file_name, 'source>>),
    /// A constant type modifier
    ///
    /// ```tantalum
    /// const i32 // A constant i32
    /// ```
    Const(#[cfg_attr(feature = "serde", serde(borrow))] Box<Type<'file_name, 'source>>),
    /// A named type
    ///
    /// ```tantalum
    /// i32 // A named type
    /// ```
    Named(&'source str),
}
