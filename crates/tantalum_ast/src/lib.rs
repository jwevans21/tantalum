//! # Tantalum Abstract Syntax Tree
//!
//! Provides the abstract syntax tree for the Tantalum language.

mod expressions;
mod items;
mod literals;
mod statements;
mod types;

pub use expressions::*;
pub use items::*;
pub use literals::*;
pub use statements::*;
use tantalum_span::Spanned;
pub use types::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AST<'file_name, 'source>(
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub  Vec<Spanned<'file_name, Item<'file_name, 'source>>>,
);

#[allow(unused_variables)]
pub trait ASTVisitor<'file_name, 'source> {
    fn visit_ast(&mut self, ast: &AST<'file_name, 'source>) {
        for item in &ast.0 {
            self.visit_item(item.data());
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Items
    ////////////////////////////////////////////////////////////////////////////

    fn visit_item(&mut self, item: &Item<'file_name, 'source>) {
        match item {
            Item::Function(function) => self.visit_function(function),
            Item::ExternalFunction(external_function) => {
                self.visit_external_function(external_function);
            }
        }
    }

    fn visit_function(&mut self, function: &Function<'file_name, 'source>) {}
    fn visit_external_function(
        &mut self,
        external_function: &ExternalFunction<'file_name, 'source>,
    ) {
    }

    ////////////////////////////////////////////////////////////////////////////
    // Parameters
    ////////////////////////////////////////////////////////////////////////////

    fn visit_parameter(&mut self, parameter: &Parameter<'file_name, 'source>) {
        match parameter {
            Parameter::Named(named) => self.visit_named_parameter(named),
            Parameter::Variadic => self.visit_variadic_parameter(),
        }
    }

    fn visit_named_parameter(&mut self, named: &NamedParameter<'file_name, 'source>) {}
    fn visit_variadic_parameter(&mut self) {}

    ////////////////////////////////////////////////////////////////////////////
    // Types
    ////////////////////////////////////////////////////////////////////////////

    fn visit_type(&mut self, ty: &Type<'file_name, 'source>) {
        match ty {
            Type::Named(named) => self.visit_named_type(named),
            Type::Function(function) => self.visit_function_type(function),
            Type::Pointer(pointer) => self.visit_pointer_type(pointer),
            Type::SizedArray(array) => self.visit_sized_array_type(array),
            Type::UnsizedArray(array) => self.visit_unsized_array_type(array),
            Type::Const(constant) => self.visit_const_type(constant),
        }
    }

    fn visit_named_type(&mut self, named: &NamedType<'file_name, 'source>) {}
    fn visit_function_type(&mut self, function: &FunctionType<'file_name, 'source>) {}
    fn visit_pointer_type(&mut self, pointer: &PointerType<'file_name, 'source>) {}
    fn visit_sized_array_type(&mut self, array: &SizedArrayType<'file_name, 'source>) {}
    fn visit_unsized_array_type(&mut self, array: &UnsizedArrayType<'file_name, 'source>) {}
    fn visit_const_type(&mut self, constant: &ConstType<'file_name, 'source>) {}

    ////////////////////////////////////////////////////////////////////////////
    // Statements
    ////////////////////////////////////////////////////////////////////////////

    fn visit_statement(&mut self, statement: &Statement<'file_name, 'source>) {
        match statement {
            Statement::Block(block) => self.visit_block(block),
            Statement::VariableDeclaration(declaration) => {
                self.visit_variable_declaration(declaration);
            }
            Statement::If(if_statement) => self.visit_if(if_statement),
            Statement::While(while_statement) => self.visit_while(while_statement),
            Statement::ForInitCondUpdate(for_statement) => {
                self.visit_for_init_cond_update(for_statement);
            }
            Statement::Break => self.visit_break(),
            Statement::Continue => self.visit_continue(),
            Statement::Return(return_statement) => self.visit_return(return_statement),
            Statement::Expression(expression) => self.visit_expression_statement(expression),
        }
    }

    fn visit_block(&mut self, block: &Block<'file_name, 'source>) {}
    fn visit_variable_declaration(
        &mut self,
        declaration: &VariableDeclaration<'file_name, 'source>,
    ) {
    }
    fn visit_if(&mut self, if_statement: &If<'file_name, 'source>) {}
    fn visit_while(&mut self, while_statement: &While<'file_name, 'source>) {}
    fn visit_for_init_cond_update(
        &mut self,
        for_statement: &ForInitCondUpdate<'file_name, 'source>,
    ) {
    }
    fn visit_break(&mut self) {}
    fn visit_continue(&mut self) {}
    fn visit_return(&mut self, return_statement: &Return<'file_name, 'source>) {}
    fn visit_expression_statement(&mut self, expression: &Expression<'file_name, 'source>) {
        self.visit_expression(expression);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Expressions
    ////////////////////////////////////////////////////////////////////////////

    fn visit_expression(&mut self, expression: &Expression<'file_name, 'source>) {
        match expression {
            Expression::Variable(variable) => self.visit_variable(variable),
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::FunctionCall(call) => self.visit_function_call(call),
            Expression::MemberAccess(access) => self.visit_member_access(access),
            Expression::Index(index) => self.visit_array_access(index),
            Expression::UnaryOperation(unary) => self.visit_unary_operation(unary),
            Expression::BinaryOperation(binary) => self.visit_binary_operation(binary),
            Expression::TypeCast(cast) => self.visit_type_cast(cast),
        }
    }

    fn visit_variable(&mut self, variable: &Variable<'file_name, 'source>) {}
    fn visit_function_call(&mut self, call: &FunctionCall<'file_name, 'source>) {}
    fn visit_member_access(&mut self, access: &MemberAccess<'file_name, 'source>) {}
    fn visit_array_access(&mut self, index: &Index<'file_name, 'source>) {}
    fn visit_unary_operation(&mut self, unary: &UnaryOperation<'file_name, 'source>) {}
    fn visit_binary_operation(&mut self, binary: &BinaryOperation<'file_name, 'source>) {}
    fn visit_type_cast(&mut self, cast: &TypeCast<'file_name, 'source>) {}

    ////////////////////////////////////////////////////////////////////////////
    // Literals
    ////////////////////////////////////////////////////////////////////////////

    fn visit_literal(&mut self, literal: &Literal<'file_name, 'source>) {
        match literal {
            Literal::Integer(integer) => self.visit_integer_literal(integer),
            Literal::Float(float) => self.visit_float_literal(float),
            Literal::Boolean(boolean) => self.visit_boolean_literal(boolean),
            Literal::Character(character) => self.visit_character_literal(character),
            Literal::String(string) => self.visit_string_literal(string),
        }
    }

    fn visit_integer_literal(&mut self, integer: &Integer<'file_name, 'source>) {}
    fn visit_float_literal(&mut self, float: &Float<'file_name, 'source>) {}
    fn visit_boolean_literal(&mut self, boolean: &Boolean<'file_name, 'source>) {}
    fn visit_character_literal(&mut self, character: &Character<'file_name, 'source>) {}
    fn visit_string_literal(&mut self, string: &String<'file_name, 'source>) {}
}

/*
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
*/
