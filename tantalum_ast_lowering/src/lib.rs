use std::{collections::VecDeque, rc::Rc};

use tantalum_hlir::{
    HLIRBinaryOperation, HLIRBinaryOperator, HLIRBlock, HLIRExpression, HLIRForInitCondUpdate,
    HLIRFunction, HLIRFunctionCall, HLIRFunctionPrototypeAnonymous, HLIRFunctionType, HLIRIf,
    HLIRIndex, HLIRLiteral, HLIRLiteralInner, HLIRMemberAccess, HLIRPackage, HLIRPath, HLIRReturn,
    HLIRStatement, HLIRType, HLIRTypeCast, HLIRUnaryOperation, HLIRUnaryOperator,
    HLIRVariableDeclaration, HLIRWhile,
};

mod prototypes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASTLoweringContext {
    pub package: HLIRPackage,

    current_type: Option<Rc<HLIRType>>,
    current_statement: Option<HLIRStatement>,
    expression_stack: VecDeque<HLIRExpression>,
}

impl ASTLoweringContext {
    #[must_use]
    pub fn new() -> Self {
        Self {
            package: HLIRPackage::new(),

            current_type: None,
            current_statement: None,
            expression_stack: VecDeque::new(),
        }
    }

    pub fn lower(&mut self, ast: &tantalum_ast::AST) {
        prototypes::PrototypeExtractor::new(&mut self.package).lower(ast);

        <Self as tantalum_ast::ASTVisitor>::visit_ast(self, ast);
    }

    #[must_use]
    pub fn finish(self) -> HLIRPackage {
        self.package
    }

    fn get_or_insert_literal(&mut self, inner: HLIRLiteralInner) -> Rc<HLIRLiteralInner> {
        self.package.literals.get(&inner).cloned().map_or_else(
            || {
                let inner = Rc::new(inner);
                self.package.literals.insert(inner.clone());
                inner
            },
            |index| index,
        )
    }

    fn visit_binary_operator(operator: &tantalum_ast::BinaryOperator) -> HLIRBinaryOperator {
        match operator {
            tantalum_ast::BinaryOperator::Addition => HLIRBinaryOperator::Addition,
            tantalum_ast::BinaryOperator::Subtraction => HLIRBinaryOperator::Subtraction,
            tantalum_ast::BinaryOperator::Multiplication => HLIRBinaryOperator::Multiplication,
            tantalum_ast::BinaryOperator::Division => HLIRBinaryOperator::Division,
            tantalum_ast::BinaryOperator::Modulus => HLIRBinaryOperator::Modulus,

            tantalum_ast::BinaryOperator::BitwiseAnd => HLIRBinaryOperator::BitwiseAnd,
            tantalum_ast::BinaryOperator::BitwiseOr => HLIRBinaryOperator::BitwiseOr,
            tantalum_ast::BinaryOperator::BitwiseXor => HLIRBinaryOperator::BitwiseXor,
            tantalum_ast::BinaryOperator::LeftShift => HLIRBinaryOperator::LeftShift,
            tantalum_ast::BinaryOperator::RightShift => HLIRBinaryOperator::RightShift,

            tantalum_ast::BinaryOperator::LessThan => HLIRBinaryOperator::LessThan,
            tantalum_ast::BinaryOperator::LessThanOrEqual => HLIRBinaryOperator::LessThanOrEqual,
            tantalum_ast::BinaryOperator::GreaterThan => HLIRBinaryOperator::GreaterThan,
            tantalum_ast::BinaryOperator::GreaterThanOrEqual => {
                HLIRBinaryOperator::GreaterThanOrEqual
            }
            tantalum_ast::BinaryOperator::Equal => HLIRBinaryOperator::Equal,
            tantalum_ast::BinaryOperator::NotEqual => HLIRBinaryOperator::NotEqual,

            tantalum_ast::BinaryOperator::LogicalAnd => HLIRBinaryOperator::LogicalAnd,
            tantalum_ast::BinaryOperator::LogicalOr => HLIRBinaryOperator::LogicalOr,

            tantalum_ast::BinaryOperator::Assignment => HLIRBinaryOperator::Assignment,
        }
    }

    fn visit_unary_operator(operator: &tantalum_ast::UnaryOperator) -> HLIRUnaryOperator {
        match operator {
            tantalum_ast::UnaryOperator::Negation => HLIRUnaryOperator::Negation,
            tantalum_ast::UnaryOperator::LogicalNegation => HLIRUnaryOperator::LogicalNegation,
            tantalum_ast::UnaryOperator::BitwiseNegation => HLIRUnaryOperator::BitwiseNegation,
            tantalum_ast::UnaryOperator::Deref => HLIRUnaryOperator::Deref,
            tantalum_ast::UnaryOperator::Ref => HLIRUnaryOperator::Ref,
        }
    }
}

impl Default for ASTLoweringContext {
    fn default() -> Self {
        Self::new()
    }
}

impl<'file_name, 'source> tantalum_ast::ASTVisitor<'file_name, 'source> for ASTLoweringContext {
    fn visit_function(&mut self, function: &tantalum_ast::Function<'file_name, 'source>) {
        // enter a new scope for the function
        self.package.scope.enter();

        let prototype = self
            .package
            .prototypes
            .get(*function.name.data())
            .expect("function prototype should have been extracted by the PrototypeExtractor")
            .clone();

        // declare the parameters in the function scope
        for (parameter, ty) in prototype
            .parameter_names
            .iter()
            .zip(&prototype.inner.parameters)
        {
            self.package.declare_variable(parameter, Some(ty.clone()));
        }

        self.visit_statement(function.body.data());

        let body = self
            .current_statement
            .take()
            .expect("function body should have been lowered");

        // exit the function scope
        let function_scope = self.package.scope.exit();

        let function = HLIRFunction {
            prototype: prototype.clone(),
            scope: function_scope,
            body,
        };

        self.package.functions.insert(prototype, function);
    }

    fn visit_external_function(&mut self, _: &tantalum_ast::ExternalFunction<'file_name, 'source>) {
        // do nothing, external functions are prototypes only which are handled by the PrototypeExtractor
    }

    fn visit_named_parameter(&mut self, _: &tantalum_ast::NamedParameter<'file_name, 'source>) {
        panic!("parameters are only handled by the PrototypeExtractor")
    }

    fn visit_variadic_parameter(&mut self) {
        panic!("parameters are only handled by the PrototypeExtractor")
    }

    fn visit_named_type(&mut self, named: &tantalum_ast::NamedType<'file_name, 'source>) {
        let path = HLIRPath::new(vec![(*named.name.data()).to_string()]);
        let ty = self
            .package
            .types
            .get(&path)
            .expect("named type should have been extracted by the PrototypeExtractor")
            .clone();

        self.current_type = Some(ty);
    }

    fn visit_function_type(&mut self, function: &tantalum_ast::FunctionType<'file_name, 'source>) {
        self.visit_type(function.return_type.data());
        let return_type = self
            .current_type
            .take()
            .expect("function type should have a return type");

        let parameters = function
            .parameters
            .iter()
            .map(|parameter| {
                self.visit_type(parameter.data());
                self.current_type.take().unwrap()
            })
            .collect();

        self.current_type = Some(Rc::new(HLIRType::Function(HLIRFunctionType {
            prototype: Rc::new(HLIRFunctionPrototypeAnonymous {
                parameters,
                is_variadic: function.is_variadic,
                return_type,
            }),
        })));
    }

    fn visit_pointer_type(&mut self, pointer: &tantalum_ast::PointerType<'file_name, 'source>) {
        self.visit_type(pointer.ty.data());

        let ty = self
            .current_type
            .take()
            .expect("pointer type should have a type");

        self.current_type = Some(Rc::new(HLIRType::Pointer(ty)));
    }

    fn visit_sized_array_type(
        &mut self,
        array: &tantalum_ast::SizedArrayType<'file_name, 'source>,
    ) {
        self.visit_type(array.ty.data());

        let ty = self
            .current_type
            .take()
            .expect("sized array type should have a type");

        self.current_type = Some(Rc::new(HLIRType::SizedArray(ty, *array.size.data())));
    }

    fn visit_unsized_array_type(
        &mut self,
        array: &tantalum_ast::UnsizedArrayType<'file_name, 'source>,
    ) {
        self.visit_type(array.ty.data());

        let ty = self
            .current_type
            .take()
            .expect("unsized array type should have a type");

        self.current_type = Some(Rc::new(HLIRType::UnsizedArray(ty)));
    }

    fn visit_const_type(&mut self, constant: &tantalum_ast::ConstType<'file_name, 'source>) {
        self.visit_type(constant.ty.data());

        let ty = self
            .current_type
            .take()
            .expect("const type should have a type");

        self.current_type = Some(Rc::new(HLIRType::Const(ty)));
    }

    fn visit_block(&mut self, block: &tantalum_ast::Block<'file_name, 'source>) {
        // enter a new scope for the block
        self.package.scope.enter();

        let mut statements = Vec::new();

        for statement in &block.statements {
            self.visit_statement(statement.data());

            let statement = self
                .current_statement
                .take()
                .expect("each should set self.current_statement");

            statements.push(statement);
        }

        // exit the block scope
        let block_scope = self.package.scope.exit();

        self.current_statement = Some(HLIRStatement::Block(HLIRBlock {
            scope: block_scope,
            statements,
        }));
    }

    fn visit_variable_declaration(
        &mut self,
        declaration: &tantalum_ast::VariableDeclaration<'file_name, 'source>,
    ) {
        let ty = if let Some(ty) = &declaration.ty {
            self.visit_type(ty.data());
            Some(self.current_type.take().unwrap())
        } else {
            None
        };

        self.visit_expression(declaration.value.data());

        assert!(
            self.expression_stack.len() == 1,
            "variable declaration should leave one value on the stack"
        );

        let value = self.expression_stack.pop_front().unwrap();

        // must get the index last to allow resolution of same-name variables in the same scope
        if self.package.scope.get(declaration.name.data()).is_some() {
            eprintln!(
                "warning: redeclaration of variable '{}'",
                declaration.name.data()
            );
        }

        let index = self
            .package
            .declare_variable(declaration.name.data(), ty.clone());

        self.current_statement = Some(HLIRStatement::VariableDeclaration(
            HLIRVariableDeclaration { index, ty, value },
        ));
    }

    fn visit_if(&mut self, if_statement: &tantalum_ast::If<'file_name, 'source>) {
        self.visit_expression(if_statement.condition.data());

        let condition = self
            .expression_stack
            .pop_front()
            .expect("if statement should have a condition");

        self.visit_statement(if_statement.body.data());

        let body = self
            .current_statement
            .take()
            .expect("if statement should have a body");

        let else_branch = if let Some(else_branch) = &if_statement.else_branch {
            self.visit_statement(else_branch.data());

            Some(Box::new(
                self.current_statement
                    .take()
                    .expect("if statement should have an else branch"),
            ))
        } else {
            None
        };

        self.current_statement = Some(HLIRStatement::If(HLIRIf {
            condition,
            body: Box::new(body),
            else_branch,
        }));
    }

    fn visit_while(&mut self, while_statement: &tantalum_ast::While<'file_name, 'source>) {
        self.visit_expression(while_statement.condition.data());

        let condition = self
            .expression_stack
            .pop_front()
            .expect("while statement should have a condition");

        self.visit_statement(while_statement.body.data());

        let body = self
            .current_statement
            .take()
            .expect("while statement should have a body");

        self.current_statement = Some(HLIRStatement::While(HLIRWhile {
            condition,
            body: Box::new(body),
        }));
    }

    fn visit_for_init_cond_update(
        &mut self,
        for_statement: &tantalum_ast::ForInitCondUpdate<'file_name, 'source>,
    ) {
        self.visit_statement(for_statement.init.data());
        let init = self
            .current_statement
            .take()
            .expect("for statement should have an init statement");

        self.visit_statement(for_statement.condition.data());
        let condition = self
            .current_statement
            .take()
            .expect("for statement should have a condition statement");

        self.visit_statement(for_statement.update.data());
        let update = self
            .current_statement
            .take()
            .expect("for statement should have an update statement");

        self.visit_statement(for_statement.body.data());
        let body = self
            .current_statement
            .take()
            .expect("for statement should have a body statement");

        self.current_statement = Some(HLIRStatement::ForInitCondUpdate(HLIRForInitCondUpdate {
            init: Box::new(init),
            condition: Box::new(condition),
            update: Box::new(update),
            body: Box::new(body),
        }));
    }

    fn visit_break(&mut self) {
        self.current_statement = Some(HLIRStatement::Break);
    }

    fn visit_continue(&mut self) {
        self.current_statement = Some(HLIRStatement::Continue);
    }

    fn visit_return(&mut self, return_statement: &tantalum_ast::Return<'file_name, 'source>) {
        let value = if let Some(value) = &return_statement.value {
            self.visit_expression(value.data());

            assert!(
                self.expression_stack.len() == 1,
                "return statement should leave one value on the stack"
            );

            Some(self.expression_stack.pop_front().unwrap())
        } else {
            None
        };

        self.current_statement = Some(HLIRStatement::Return(HLIRReturn { value }));
    }

    fn visit_expression_statement(
        &mut self,
        expression_statement: &tantalum_ast::Expression<'file_name, 'source>,
    ) {
        self.visit_expression(expression_statement);

        assert!(
            self.expression_stack.len() == 1,
            "expression statement should leave one value on the stack"
        );

        let expression = self.expression_stack.pop_front().unwrap();

        self.current_statement = Some(HLIRStatement::Expression(expression));
    }

    fn visit_variable(&mut self, variable: &tantalum_ast::Variable<'file_name, 'source>) {
        let index = self
            .package
            .scope
            .get(variable.name.data())
            .expect("variable should be in scope");

        self.expression_stack
            .push_front(HLIRExpression::Variable(index));
    }

    fn visit_function_call(&mut self, call: &tantalum_ast::FunctionCall<'file_name, 'source>) {
        self.visit_expression(call.function.data());
        let function_expression = self
            .expression_stack
            .pop_front()
            .expect("function call should have a function expression");

        let arguments = call
            .arguments
            .iter()
            .map(|argument| {
                self.visit_expression(argument.data());
                self.expression_stack.pop_front().unwrap()
            })
            .collect();

        let function = HLIRFunctionCall {
            function: Box::new(function_expression),
            arguments,
        };

        self.expression_stack
            .push_front(HLIRExpression::FunctionCall(function));
    }

    fn visit_member_access(&mut self, access: &tantalum_ast::MemberAccess<'file_name, 'source>) {
        self.visit_expression(access.object.data());
        let object = self
            .expression_stack
            .pop_front()
            .expect("member access should have an object");

        self.expression_stack
            .push_front(HLIRExpression::MemberAccess(HLIRMemberAccess {
                object: Box::new(object),
                member: (*access.member.data()).to_string(),
            }));
    }

    fn visit_array_access(&mut self, index: &tantalum_ast::Index<'file_name, 'source>) {
        self.visit_expression(index.object.data());
        let object = self
            .expression_stack
            .pop_front()
            .expect("array access should have an object");

        self.visit_expression(index.index.data());
        let index = self
            .expression_stack
            .pop_front()
            .expect("array access should have an index");

        self.expression_stack
            .push_front(HLIRExpression::Index(HLIRIndex {
                object: Box::new(object),
                index: Box::new(index),
            }));
    }

    fn visit_unary_operation(&mut self, unary: &tantalum_ast::UnaryOperation<'file_name, 'source>) {
        self.visit_expression(unary.operand.data());
        let operand = self
            .expression_stack
            .pop_front()
            .expect("unary operation should have an operand");

        self.expression_stack
            .push_front(HLIRExpression::UnaryOperation(HLIRUnaryOperation {
                operator: Self::visit_unary_operator(unary.operator.data()),
                operand: Box::new(operand),
            }));
    }

    fn visit_binary_operation(
        &mut self,
        binary: &tantalum_ast::BinaryOperation<'file_name, 'source>,
    ) {
        self.visit_expression(binary.left.data());
        self.visit_expression(binary.right.data());

        let right = self
            .expression_stack
            .pop_front()
            .expect("binary operation should have right operand");
        let left = self
            .expression_stack
            .pop_front()
            .expect("binary operation should have left operand");

        let expression = HLIRBinaryOperation {
            left: Box::new(left),
            operator: Self::visit_binary_operator(binary.operator.data()),
            right: Box::new(right),
        };

        self.expression_stack
            .push_front(HLIRExpression::BinaryOperation(expression));
    }

    fn visit_type_cast(&mut self, cast: &tantalum_ast::TypeCast<'file_name, 'source>) {
        self.visit_expression(cast.value.data());
        let value = self
            .expression_stack
            .pop_front()
            .expect("type cast should have a value");

        self.visit_type(cast.ty.data());
        let ty = self
            .current_type
            .take()
            .expect("type cast should have a type");

        self.expression_stack
            .push_front(HLIRExpression::TypeCast(HLIRTypeCast {
                expression: Box::new(value),
                ty,
            }));
    }

    fn visit_integer_literal(&mut self, integer: &tantalum_ast::Integer<'file_name, 'source>) {
        let inner = HLIRLiteralInner::Integer {
            value: (*integer.value.data()).to_string(),
            radix: integer.radix,
        };

        let literal = self.get_or_insert_literal(inner);

        self.expression_stack
            .push_front(HLIRExpression::Literal(HLIRLiteral(literal)));
    }

    fn visit_float_literal(&mut self, float: &tantalum_ast::Float<'file_name, 'source>) {
        let inner = HLIRLiteralInner::Float {
            value: (*float.value.data()).to_string(),
        };

        let literal = self.get_or_insert_literal(inner);

        self.expression_stack
            .push_front(HLIRExpression::Literal(HLIRLiteral(literal)));
    }

    fn visit_boolean_literal(&mut self, boolean: &tantalum_ast::Boolean<'file_name, 'source>) {
        let inner = HLIRLiteralInner::Boolean {
            value: boolean
                .value
                .data()
                .parse()
                .expect("parser should have validated boolean value"),
        };

        let literal = self.get_or_insert_literal(inner);

        self.expression_stack
            .push_front(HLIRExpression::Literal(HLIRLiteral(literal)));
    }

    fn visit_character_literal(
        &mut self,
        character: &tantalum_ast::Character<'file_name, 'source>,
    ) {
        let inner = HLIRLiteralInner::Character {
            value: character.value.data().chars().next().unwrap(),
        };

        let literal = self.get_or_insert_literal(inner);

        self.expression_stack
            .push_front(HLIRExpression::Literal(HLIRLiteral(literal)));
    }

    fn visit_string_literal(&mut self, string: &tantalum_ast::String<'file_name, 'source>) {
        let inner = HLIRLiteralInner::String {
            value: string
                .value
                .data()
                .strip_prefix('"')
                .expect("strings should start with '\"'")
                .strip_suffix('"')
                .expect("strings should end with '\"'")
                .to_string(),
        };

        let literal = self.get_or_insert_literal(inner);

        self.expression_stack
            .push_front(HLIRExpression::Literal(HLIRLiteral(literal)));
    }
}
