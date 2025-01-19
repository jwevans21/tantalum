#![feature(debug_closure_helpers)]

use crate::{
    functions::{FunctionId, Functions},
    inference::{InferenceId, TypeInferenceEnvironment},
    items::{FunctionBuilder, FunctionPrototype},
    literals::Literal,
    statements::Let,
    traits::Traits,
    types::PrimitiveType,
    types::{Type, Types},
    variables::Variables,
};
use std::collections::HashMap;
use std::rc::Rc;

use crate::expressions::{
    BinaryOperation, BinaryOperator, FunctionCall, TypeCast, UnaryOperation, UnaryOperator,
};
use crate::inference::TypeConstraint;
use crate::items::Function;
use crate::literals::LiteralValue;
use crate::statements::{Block, If, Return, While};
use crate::traits::TraitImpl;
use crate::types::TypeScopeId;
use crate::variables::VariableScopeBlockId;
pub use crate::{
    expressions::Expression,
    path::{Path, PathSegment},
    statements::Statement,
    types::TypeId,
    variables::VariableId,
};

mod expressions;
mod functions;
mod inference;
mod items;
mod literals;
mod path;
mod statements;
mod traits;
mod types;
mod variables;

/// A HLIR program package
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HLIRPackage {
    /// All the types found while parsing the package
    types: Types,
    /// All the traits and their implementations found while parsing the package
    traits: Traits,
    /// All the functions found while parsing the package
    functions: Functions,
    /// All the variables found while parsing the package
    variables: Variables,

    type_inference_environment: TypeInferenceEnvironment,

    current_function: Option<FunctionId>,
    building_functions: HashMap<FunctionId, FunctionBuilder>,
    current_blocks: Vec<(VariableScopeBlockId, TypeScopeId)>,
}

impl HLIRPackage {
    #[must_use]
    pub fn new() -> Self {
        let mut package = Self {
            types: Types::new(),
            traits: Traits::new(),
            functions: Functions::new(),
            variables: Variables::new(),

            type_inference_environment: TypeInferenceEnvironment::new(),

            current_function: None,
            building_functions: HashMap::new(),
            current_blocks: Vec::new(),
        };

        package.add_builtin_types();

        package
    }

    fn add_builtin_types(&mut self) {
        for (path, ty) in [
            (
                Path::new(vec![PathSegment::from("void")]),
                Type::Primitive(PrimitiveType::Void),
            ),
            (
                Path::new(vec![PathSegment::from("i8")]),
                Type::Primitive(PrimitiveType::I8),
            ),
            (
                Path::new(vec![PathSegment::from("i16")]),
                Type::Primitive(PrimitiveType::I16),
            ),
            (
                Path::new(vec![PathSegment::from("i32")]),
                Type::Primitive(PrimitiveType::I32),
            ),
            (
                Path::new(vec![PathSegment::from("i64")]),
                Type::Primitive(PrimitiveType::I64),
            ),
            (
                Path::new(vec![PathSegment::from("u8")]),
                Type::Primitive(PrimitiveType::U8),
            ),
            (
                Path::new(vec![PathSegment::from("u16")]),
                Type::Primitive(PrimitiveType::U16),
            ),
            (
                Path::new(vec![PathSegment::from("u32")]),
                Type::Primitive(PrimitiveType::U32),
            ),
            (
                Path::new(vec![PathSegment::from("u64")]),
                Type::Primitive(PrimitiveType::U64),
            ),
            (
                Path::new(vec![PathSegment::from("f32")]),
                Type::Primitive(PrimitiveType::F32),
            ),
            (
                Path::new(vec![PathSegment::from("f64")]),
                Type::Primitive(PrimitiveType::F64),
            ),
            (
                Path::new(vec![PathSegment::from("bool")]),
                Type::Primitive(PrimitiveType::Bool),
            ),
            (
                Path::new(vec![PathSegment::from("char")]),
                Type::Primitive(PrimitiveType::Char),
            ),
            (
                Path::new(vec![PathSegment::from("str")]),
                Type::Primitive(PrimitiveType::Str),
            ),
        ] {
            self.add_type(path, ty);
        }
    }

    // pub fn add_builtin_traits(&mut self) {
    //     let trait_id = self
    //         .traits
    //         .get_or_insert(Path::new(vec![PathSegment::from("Ref")]));
    //
    //     let t = self.traits.get_trait_mut(id).expect("trait not found");
    //
    //     t.add_type_parameter("T".to_string());
    //
    //     let id = t.add_method(
    //         "ref".to_string(),
    //         vec![t.get_type_parameter("Self").unwrap()],
    //         t.get_type_parameter("T").unwrap(),
    //     );
    //
    //     let trait_impl = TraitImpl {
    //         trait_id,
    //         type_id: self
    //             .get_type_id(&Path::new(vec![PathSegment::from("str")]))
    //             .unwrap(),
    //         type_constraints: {
    //             let mut map = HashMap::new();
    //             map.insert(
    //                 t.get_type_parameter("T").unwrap(),
    //                 vec![TypeConstraint::Type(self.types.get_or_insert(Type::Ptr(
    //                     self.types.get_or_insert(Type::Primitive(PrimitiveType::U8)),
    //                 )))],
    //             );
    //             map
    //         },
    //         method_impls: {
    //             let mut map = HashMap::new();
    //             // map.insert(
    //             //     id,
    //             //     Function {
    //             //         variable_scope: (),
    //             //         type_scope: (),
    //             //         prototype: Rc::new(FunctionPrototype {}),
    //             //         parameters: vec![],
    //             //         body: (),
    //             //     },
    //             // );
    //             map
    //         },
    //     };
    // }

    // TODO: Implement a building interface for types

    /// Used to define completely new types.
    ///
    /// With
    pub fn add_type(&mut self, path: Path, ty: Type) -> TypeId {
        self.types.create_type(path, ty)
    }

    /// Used to alias types (e.g. `type MyInt = i32;`)
    ///
    /// # Panics
    ///
    /// Panics if the current type is not found.
    pub fn add_type_alias(&mut self, new: Path, current: &Path) {
        let current_id = self.types.get(current).expect("Type not found");
        self.types.create_type_with_id(new, current_id);
    }

    /// Get the ID of a type by its path.
    ///
    /// This performs a lookup in the current scope.
    #[must_use]
    pub fn get_type_id(&self, path: &Path) -> Option<TypeId> {
        self.types.get(path)
    }

    // TODO: Implement a building interface for types

    pub fn build_type_pointer(&mut self, ty: TypeId) -> TypeId {
        self.types.get_or_insert(Type::Ptr(ty))
    }

    pub fn build_type_array(&mut self, ty: TypeId, size: usize) -> TypeId {
        self.types.get_or_insert(Type::SizedArray(ty, size))
    }

    pub fn build_type_unsized_array(&mut self, ty: TypeId) -> TypeId {
        self.types.get_or_insert(Type::UnsizedArray(ty))
    }

    // TODO: Implement a building interface for variable types

    pub fn create_type_inference_variable(&mut self) -> InferenceId {
        self.type_inference_environment.create_unknown()
    }

    pub fn create_type_inference_resolved(&mut self, ty: TypeId) -> InferenceId {
        self.type_inference_environment.create_resolved(ty)
    }

    // TODO: Implement a building interface for traits
    //          - Create a trait
    //          - Add trait generic type
    //          - Add trait method
    //              - Add method generic type
    //              - Add method parameter (name is irrelevant since it can be
    //                overridden by the implementor)
    //              - Add method return type
    //          - Add trait implementation
    //              - Add implementation generic type
    //              - Add implementation generic type restrictions
    //              - Add implementation method (ideally treated a normal function)

    // TODO: Implement a building interface for function prototypes

    #[must_use]
    pub fn get_function_id(&self, path: &Path) -> Option<FunctionId> {
        self.functions.get(path)
    }

    #[must_use]
    pub fn get_prototype(&self, id: FunctionId) -> Option<Rc<FunctionPrototype>> {
        self.functions.get_prototype(id)
    }

    pub fn build_function_prototype(
        &mut self,
        parameters: Vec<TypeId>,
        is_variadic: bool,
        return_type: TypeId,
    ) -> FunctionPrototype {
        FunctionPrototype {
            parameters,
            is_variadic,
            return_type,
        }
    }

    // TODO: Implement a building interface for functions

    pub fn create_function(&mut self, path: Path, prototype: FunctionPrototype) -> FunctionId {
        self.functions.create_function(path, prototype)
    }

    /// Start building a function.
    ///
    /// # Panics
    ///
    /// This function will panic if a function with the same ID is already being built.
    pub fn start_function_impl(&mut self, id: FunctionId) {
        assert!(
            !self.building_functions.contains_key(&id),
            "function already being built"
        );

        self.current_function = Some(id);

        let prototype = self
            .functions
            .get_prototype(id)
            .expect("function not found");

        let variable_scope = self.variables.push_scope();

        let type_scope = self.types.push_scope();

        self.building_functions.insert(
            id,
            FunctionBuilder::new(variable_scope, type_scope, prototype),
        );
    }

    /// Add a parameter to a function.
    ///
    /// # Panics
    ///
    /// This function will panic if the function is not being built.
    pub fn add_function_parameter(&mut self, id: FunctionId, parameter: VariableId) {
        let builder = self
            .building_functions
            .get_mut(&id)
            .expect("function not being built");

        builder.add_parameter(parameter);
    }

    /// Set the body of a function.
    ///
    /// # Panics
    ///
    /// This function will panic if the function is not being built.
    pub fn set_function_body(&mut self, id: FunctionId, body: Statement) {
        let builder = self
            .building_functions
            .get_mut(&id)
            .expect("function not being built");

        builder.set_body(body);
    }

    /// Finish building a function.
    ///
    /// # Panics
    ///
    /// This function will panic if the function is not being built or if the
    /// function body has not been set.
    pub fn finish_function_impl(&mut self, id: FunctionId) {
        let builder = self
            .building_functions
            .remove(&id)
            .expect("function not being built");
        self.current_function.take();

        self.type_inference_environment.unify_final(&self.types);

        let function = builder.build().expect("function body not set");

        self.variables.pop_scope();
        self.types.pop_scope();

        self.functions.insert(id, function);
    }

    // TODO: Implement a building interface for variables

    /// Add a variable to the HLIR package and allow it to be accessed by its
    /// name within the current scope.
    pub fn create_variable(&mut self, name: &str, ty: InferenceId) -> VariableId {
        self.variables.create_variable(name, ty)
    }

    /// Get the ID of a variable by its name.
    #[must_use]
    pub fn get_variable_id(&self, name: &str) -> Option<VariableId> {
        self.variables.get(name)
    }

    // TODO: Implement a building interface for statements

    pub fn build_block_start(&mut self) {
        let variable_scope = self.variables.push_scope();
        let type_scope = self.types.push_scope();

        self.current_blocks.push((variable_scope, type_scope));
    }

    /// Finish building a block and get the statement.
    ///
    /// # Panics
    ///
    /// This function will panic if a block is not being built.
    pub fn build_block_end(&mut self, statements: Vec<Statement>) -> Statement {
        self.variables.pop_scope();
        self.types.pop_scope();

        let block = self.current_blocks.pop().expect("block not found");
        let block = Block {
            variable_scope: block.0,
            type_scope: block.1,
            statements,
        };
        block.into()
    }

    pub fn build_statement_let(
        &mut self,
        name: &str,
        ty: InferenceId,
        value: Expression,
    ) -> Statement {
        let expression_ty = value.ty(self);

        self.type_inference_environment
            .unify(ty, expression_ty, &self.types);

        Let {
            variable: self.create_variable(name, ty),
            value,
        }
        .into()
    }

    /// Build an if statement.
    ///
    /// # Panics
    ///
    /// This function will panic if there is no bool type in the package.
    pub fn build_statement_if(
        &mut self,
        condition: Expression,
        then_block: Statement,
        else_block: Option<Statement>,
    ) -> Statement {
        let ty = condition.ty(self);

        self.type_inference_environment.unify_with(
            ty,
            self.types
                .get(&Path::new(vec![PathSegment::from("bool".to_string())]))
                .expect("expected bool type to exist in package"),
            &self.types,
        );

        If {
            condition,
            then_branch: Box::new(then_block),
            else_branch: else_block.map(Box::new),
        }
        .into()
    }

    pub fn build_statement_while(&mut self, condition: Expression, block: Statement) -> Statement {
        While {
            condition,
            body: Box::new(block),
        }
        .into()
    }

    pub fn build_statement_return_void(&mut self) -> Statement {
        Return::void().into()
    }

    pub fn build_statement_return(&mut self, value: Expression) -> Statement {
        let function_return_ty = self
            .building_functions
            .get(
                &self
                    .current_function
                    .expect("returns cannot be outside of a function"),
            )
            .expect("function not found")
            .prototype
            .return_type;

        let ty = value.ty(self);

        self.type_inference_environment
            .unify_with(ty, function_return_ty, &self.types);

        Return { value: Some(value) }.into()
    }

    pub fn build_statement_expression(&mut self, expression: Expression) -> Statement {
        Statement::Expression(expression)
    }

    // TODO: Implement a building interface for expressions

    /// Builds an expression that references a variable.
    ///
    /// # Panics
    ///
    /// This function will panic if the variable is not found.
    pub fn build_expression_variable(&mut self, name: &str) -> Expression {
        Expression::Variable(self.get_variable_id(name).expect("variable not found"))
    }

    pub fn build_expression_literal(&mut self, literal: Literal) -> Expression {
        Expression::Literal(literal)
    }

    pub fn build_function_call(
        &mut self,
        function: FunctionId,
        arguments: Vec<Expression>,
    ) -> Expression {
        let function_prototype = self.get_prototype(function).expect("function not found");
        let return_ty = function_prototype.return_type;
        let result = self.create_type_inference_resolved(return_ty);

        dbg!(
            &function_prototype,
            &arguments,
            &function_prototype.parameters.len()
        );
        for (i, argument) in arguments.iter().enumerate() {
            let argument_ty = argument.ty(self);

            if i < function_prototype.parameters.len() {
                let parameter_ty = function_prototype.parameters[i];
                self.type_inference_environment
                    .unify_with(argument_ty, parameter_ty, &self.types);
            } else if function_prototype.is_variadic {
                //
            } else {
                dbg!(i, &function_prototype.parameters.len(), &argument);
                panic!("too many arguments for function");
            }
        }

        Expression::FunctionCall(FunctionCall {
            function,
            arguments,
            result,
        })
    }

    pub fn build_unary_operator_negation(&mut self) -> UnaryOperator {
        UnaryOperator::Negation
    }

    pub fn build_unary_operator_bitwise_not(&mut self) -> UnaryOperator {
        UnaryOperator::BitwiseNot
    }

    pub fn build_unary_operator_logical_not(&mut self) -> UnaryOperator {
        UnaryOperator::LogicalNot
    }

    pub fn build_unary_operator_deref(&mut self) -> UnaryOperator {
        UnaryOperator::Deref
    }

    pub fn build_unary_operator_ref(&mut self) -> UnaryOperator {
        UnaryOperator::Ref
    }

    pub fn build_expression_unary(
        &mut self,
        operator: UnaryOperator,
        operand: Expression,
    ) -> Expression {
        let operand_ty = operand.ty(self);

        let result_ty = match operator {
            UnaryOperator::Negation | UnaryOperator::BitwiseNot | UnaryOperator::LogicalNot => {
                operand_ty
            }
            UnaryOperator::Deref => {
                let ty = self.create_type_inference_variable();

                self.type_inference_environment
                    .add_constraint(operand_ty, TypeConstraint::DerefTo(ty));

                ty
            }
            UnaryOperator::Ref => {
                let ty = self.create_type_inference_variable();

                self.type_inference_environment
                    .add_constraint(ty, TypeConstraint::RefTo(operand_ty));

                ty
            }
        };

        Expression::UnaryOperation(UnaryOperation {
            operator,
            operand: Box::new(operand),
            result: result_ty,
        })
    }

    pub fn build_binary_operator_addition(&mut self) -> BinaryOperator {
        BinaryOperator::Addition
    }

    pub fn build_binary_operator_subtraction(&mut self) -> BinaryOperator {
        BinaryOperator::Subtraction
    }

    pub fn build_binary_operator_multiplication(&mut self) -> BinaryOperator {
        BinaryOperator::Multiplication
    }

    pub fn build_binary_operator_division(&mut self) -> BinaryOperator {
        BinaryOperator::Division
    }

    pub fn build_binary_operator_remainder(&mut self) -> BinaryOperator {
        BinaryOperator::Remainder
    }

    pub fn build_binary_operator_bitwise_and(&mut self) -> BinaryOperator {
        BinaryOperator::BitwiseAnd
    }

    pub fn build_binary_operator_bitwise_or(&mut self) -> BinaryOperator {
        BinaryOperator::BitwiseOr
    }

    pub fn build_binary_operator_bitwise_xor(&mut self) -> BinaryOperator {
        BinaryOperator::BitwiseXor
    }

    pub fn build_binary_operator_bitwise_shift_left(&mut self) -> BinaryOperator {
        BinaryOperator::BitwiseShiftLeft
    }

    pub fn build_binary_operator_bitwise_shift_right(&mut self) -> BinaryOperator {
        BinaryOperator::BitwiseShiftRight
    }

    pub fn build_binary_operator_logical_and(&mut self) -> BinaryOperator {
        BinaryOperator::LogicalAnd
    }

    pub fn build_binary_operator_logical_or(&mut self) -> BinaryOperator {
        BinaryOperator::LogicalOr
    }

    pub fn build_binary_operator_equal(&mut self) -> BinaryOperator {
        BinaryOperator::Equals
    }

    pub fn build_binary_operator_not_equal(&mut self) -> BinaryOperator {
        BinaryOperator::NotEquals
    }

    pub fn build_binary_operator_less_than(&mut self) -> BinaryOperator {
        BinaryOperator::LessThan
    }

    pub fn build_binary_operator_less_than_or_equal(&mut self) -> BinaryOperator {
        BinaryOperator::LessThanOrEqual
    }

    pub fn build_binary_operator_greater_than(&mut self) -> BinaryOperator {
        BinaryOperator::GreaterThan
    }

    pub fn build_binary_operator_greater_than_or_equal(&mut self) -> BinaryOperator {
        BinaryOperator::GreaterThanOrEqual
    }

    pub fn build_expression_binary(
        &mut self,
        operator: BinaryOperator,
        left: Expression,
        right: Expression,
    ) -> Expression {
        let left_ty = left.ty(self);
        let right_ty = right.ty(self);
        let result_ty = match operator {
            BinaryOperator::Addition
            | BinaryOperator::Subtraction
            | BinaryOperator::Multiplication
            | BinaryOperator::Division
            | BinaryOperator::Remainder
            | BinaryOperator::BitwiseAnd
            | BinaryOperator::BitwiseOr
            | BinaryOperator::BitwiseXor
            | BinaryOperator::BitwiseShiftLeft
            | BinaryOperator::BitwiseShiftRight => left_ty,
            BinaryOperator::LogicalAnd
            | BinaryOperator::LogicalOr
            | BinaryOperator::Equals
            | BinaryOperator::NotEquals
            | BinaryOperator::LessThan
            | BinaryOperator::LessThanOrEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanOrEqual => self.create_type_inference_resolved(
                self.types
                    .get(&Path::new(vec![PathSegment::from("bool".to_string())]))
                    .expect("expected bool type to exist in package"),
            ),
        };

        self.type_inference_environment
            .unify(left_ty, right_ty, &self.types);

        Expression::BinaryOperation(BinaryOperation {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            result: result_ty,
        })
    }

    pub fn build_expression_type_cast(&mut self, ty: TypeId, expression: Expression) -> Expression {
        let target_type = self.create_type_inference_resolved(ty);
        let expression_ty = expression.ty(self);

        self.type_inference_environment
            .unify_with(expression_ty, ty, &self.types);

        Expression::TypeCast(TypeCast {
            target_type,
            expression: Box::new(expression),
        })
    }

    // TODO: Implement a building interface for literals

    pub fn build_integer_literal(&mut self, value: String, radix: u32) -> Literal {
        let type_inference_id = self.create_type_inference_variable();

        self.type_inference_environment
            .add_constraint(type_inference_id, TypeConstraint::FromIntegerLiteral);

        Literal::new(LiteralValue::Integer { value, radix }, type_inference_id)
    }

    pub fn build_float_literal(&mut self, value: String) -> Literal {
        let type_inference_id = self.create_type_inference_variable();

        self.type_inference_environment
            .add_constraint(type_inference_id, TypeConstraint::FromFloatLiteral);

        Literal::new(LiteralValue::Float { value }, type_inference_id)
    }

    pub fn build_boolean_literal(&mut self, value: bool) -> Literal {
        let type_inference_id = self.create_type_inference_variable();

        self.type_inference_environment.add_constraint(
            type_inference_id,
            TypeConstraint::Type(
                self.types
                    .get(&Path::new(vec![PathSegment::from("bool".to_string())]))
                    .expect("expected bool type to exist in package"),
            ),
        );

        Literal::new(LiteralValue::Boolean { value }, type_inference_id)
    }

    pub fn build_character_literal(&mut self, value: String) -> Literal {
        let type_inference_id = self.create_type_inference_variable();

        self.type_inference_environment.add_constraint(
            type_inference_id,
            TypeConstraint::Type(
                self.types
                    .get(&Path::new(vec![PathSegment::from("char".to_string())]))
                    .expect("expected char type to exist in package"),
            ),
        );

        Literal::new(LiteralValue::Character { value }, type_inference_id)
    }

    pub fn build_string_literal(&mut self, value: String) -> Literal {
        let type_inference_id = self.create_type_inference_variable();

        self.type_inference_environment.add_constraint(
            type_inference_id,
            TypeConstraint::Type(
                self.types
                    .get(&Path::new(vec![PathSegment::from("str".to_string())]))
                    .expect("expected str type to exist in package"),
            ),
        );

        Literal::new(LiteralValue::String { value }, type_inference_id)
    }
}

impl Default for HLIRPackage {
    fn default() -> Self {
        Self::new()
    }
}
