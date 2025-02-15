use tantalum_ast::{
    BinaryOperator, Boolean, Character, ConstType, Float, FunctionCall, FunctionType, Index,
    Integer, MemberAccess, NamedType, PointerType, Return, SizedArrayType, UnaryOperation,
    UnsizedArrayType, Variable,
};
use tantalum_hlir::{Expression, HLIRPackage, Path, PathSegment, Statement, TypeId};

pub struct FunctionLoweringContext<'a> {
    package: &'a mut HLIRPackage,
    type_stack: Vec<TypeId>,
    statement_stack: Vec<Statement>,
    expression_stack: Vec<Expression>,
}

impl<'a> FunctionLoweringContext<'a> {
    #[must_use]
    pub fn new(package: &'a mut HLIRPackage) -> Self {
        Self {
            package,
            type_stack: Vec::new(),
            statement_stack: Vec::new(),
            expression_stack: Vec::new(),
        }
    }

    pub fn lower(mut self, ast: &tantalum_ast::AST) {
        <Self as tantalum_ast::ASTVisitor>::visit_ast(&mut self, ast);
    }
}

impl tantalum_ast::ASTVisitor<'_, '_> for FunctionLoweringContext<'_> {
    fn visit_function(&mut self, function: &tantalum_ast::Function<'_, '_>) {
        let path = Path::new(vec![PathSegment::from(*(function.name.data()))]);

        let function_id = self
            .package
            .get_function_id(&path)
            .expect("expected function to exist in package");
        let function_prototype = self
            .package
            .get_prototype(function_id)
            .expect("expected prototype to exist in package");

        self.package.start_function_impl(function_id);

        // Add parameters to variable scope
        for (parameter, ty) in function
            .parameters
            .data()
            .iter()
            .zip(function_prototype.parameters.iter())
        {
            let parameter = parameter.data();
            match parameter {
                tantalum_ast::Parameter::Named(named) => {
                    let inference_id = self.package.create_type_inference_resolved(*ty);
                    let variable_id = self
                        .package
                        .create_variable(named.name.data(), inference_id);
                    self.package
                        .add_function_parameter(function_id, variable_id);
                }
                tantalum_ast::Parameter::Variadic => {}
            }
        }

        self.visit_statement(function.body.data());

        let statement = self
            .statement_stack
            .pop()
            .expect("expected statement to exist in stack");

        self.package.set_function_body(function_id, statement);

        self.package.finish_function_impl(function_id);
    }

    fn visit_named_type(&mut self, named: &NamedType<'_, '_>) {
        let path = Path::from(*(named.name.data()));
        let type_id = self
            .package
            .get_type_id(&path)
            .expect("expected type to exist in package");

        self.type_stack.push(type_id);
    }

    fn visit_function_type(&mut self, function: &FunctionType<'_, '_>) {
        todo!("function types not yet implemented, did not lower {function:?}");
    }

    fn visit_pointer_type(&mut self, pointer: &PointerType<'_, '_>) {
        self.visit_type(pointer.ty.data());
        let ty = self
            .type_stack
            .pop()
            .expect("expected type to exist in stack");

        let type_id = self.package.build_type_pointer(ty);

        self.type_stack.push(type_id);
    }

    fn visit_sized_array_type(&mut self, array: &SizedArrayType<'_, '_>) {
        todo!("sized arrays not yet implemented, did not lower {array:?}");
    }

    fn visit_unsized_array_type(&mut self, array: &UnsizedArrayType<'_, '_>) {
        self.visit_type(array.ty.data());

        let ty = self
            .type_stack
            .pop()
            .expect("expected type to exist in stack");

        let type_id = self.package.build_type_unsized_array(ty);

        self.type_stack.push(type_id);
    }

    fn visit_const_type(&mut self, constant: &ConstType<'_, '_>) {
        todo!("const types not yet implemented, did not lower {constant:?}");
    }

    fn visit_block(&mut self, block: &tantalum_ast::Block<'_, '_>) {
        self.package.build_block_start();

        let mut statements = Vec::new();

        for statement in &block.statements {
            self.visit_statement(statement.data());
            statements.push(
                self.statement_stack
                    .pop()
                    .expect("expected statement to exist in stack"),
            );
        }

        let statement = self.package.build_block_end(statements);

        self.statement_stack.push(statement);
    }

    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &tantalum_ast::VariableDeclaration<'_, '_>,
    ) {
        let name = *(variable_declaration.name.data());
        let value = variable_declaration.value.data();

        self.visit_expression(value);

        let inference_id = if let Some(ty) = &variable_declaration.ty {
            let ty = ty.data();
            self.visit_type(ty);

            let type_id = self
                .type_stack
                .pop()
                .expect("expected type to exist in stack");
            self.package.create_type_inference_resolved(type_id)
        } else {
            self.package.create_type_inference_variable()
        };

        let value = self
            .expression_stack
            .pop()
            .expect("expected statement to exist in stack");

        let statement = self.package.build_statement_let(name, inference_id, value);

        self.statement_stack.push(statement);
    }

    fn visit_if(&mut self, if_statement: &tantalum_ast::If<'_, '_>) {
        self.visit_expression(if_statement.condition.data());
        let condition = self
            .expression_stack
            .pop()
            .expect("expected statement to exist in stack");

        self.visit_statement(if_statement.body.data());
        let then_branch = self
            .statement_stack
            .pop()
            .expect("expected statement to exist in stack");

        let else_branch = if let Some(else_branch) = &if_statement.else_branch {
            self.visit_statement(else_branch.data());
            Some(
                self.statement_stack
                    .pop()
                    .expect("expected statement to exist in stack"),
            )
        } else {
            None
        };

        let statement = self
            .package
            .build_statement_if(condition, then_branch, else_branch);

        self.statement_stack.push(statement);
    }

    fn visit_return(&mut self, return_statement: &Return<'_, '_>) {
        if let Some(value) = &return_statement.value {
            self.visit_expression(value.data());
            let value = self
                .expression_stack
                .pop()
                .expect("expected statement to exist in stack");

            let statement = self.package.build_statement_return(value);

            self.statement_stack.push(statement);
        } else {
            let statement = self.package.build_statement_return_void();

            self.statement_stack.push(statement);
        }
    }

    fn visit_expression_statement(&mut self, expression: &tantalum_ast::Expression<'_, '_>) {
        self.visit_expression(expression);

        let expression = self
            .expression_stack
            .pop()
            .expect("expected statement to exist in stack");

        let statement = self.package.build_statement_expression(expression);

        self.statement_stack.push(statement);
    }

    fn visit_variable(&mut self, variable: &Variable<'_, '_>) {
        let name = *(variable.name.data());

        let expression = self.package.build_expression_variable(name);

        self.expression_stack.push(expression);
    }

    fn visit_function_call(&mut self, call: &FunctionCall<'_, '_>) {
        let function = match call.function.data() {
            tantalum_ast::Expression::Variable(variable) => {
                let name = *(variable.name.data());
                self.package
                    .get_function_id(&Path::from(name))
                    .expect("expected function to exist")
            }
            _ => panic!("expected function to be a variable"),
        };

        let mut arguments = Vec::new();
        for argument in &call.arguments {
            self.visit_expression(argument.data());
            arguments.push(
                self.expression_stack
                    .pop()
                    .expect("expected statement to exist in stack"),
            );
        }

        let expression = self.package.build_function_call(function, arguments);

        self.expression_stack.push(expression);
    }

    fn visit_member_access(&mut self, access: &MemberAccess<'_, '_>) {
        todo!("member access not yet implemented, did not lower {access:?}");
    }

    fn visit_array_access(&mut self, index: &Index<'_, '_>) {
        todo!("array access not yet implemented, did not lower {index:?}");
    }

    fn visit_unary_operation(&mut self, unary: &UnaryOperation<'_, '_>) {
        self.visit_expression(unary.operand.data());
        let operand = self
            .expression_stack
            .pop()
            .expect("expected statement to exist in stack");

        let operator = match unary.operator.data() {
            tantalum_ast::UnaryOperator::Negation => self.package.build_unary_operator_negation(),
            tantalum_ast::UnaryOperator::BitwiseNegation => {
                self.package.build_unary_operator_bitwise_not()
            }
            tantalum_ast::UnaryOperator::LogicalNegation => {
                self.package.build_unary_operator_logical_not()
            }
            tantalum_ast::UnaryOperator::Deref => self.package.build_unary_operator_deref(),
            tantalum_ast::UnaryOperator::Ref => self.package.build_unary_operator_ref(),
        };

        let expression = self.package.build_expression_unary(operator, operand);

        self.expression_stack.push(expression);
    }

    fn visit_binary_operation(&mut self, binary: &tantalum_ast::BinaryOperation<'_, '_>) {
        self.visit_expression(binary.left.data());
        let left = self
            .expression_stack
            .pop()
            .expect("expected statement to exist in stack");

        self.visit_expression(binary.right.data());
        let right = self
            .expression_stack
            .pop()
            .expect("expected statement to exist in stack");

        let operator = match binary.operator.data() {
            BinaryOperator::Addition => self.package.build_binary_operator_addition(),
            BinaryOperator::Subtraction => self.package.build_binary_operator_subtraction(),
            BinaryOperator::Multiplication => self.package.build_binary_operator_multiplication(),
            BinaryOperator::Division => self.package.build_binary_operator_division(),
            BinaryOperator::Modulus => self.package.build_binary_operator_remainder(),
            BinaryOperator::BitwiseAnd => self.package.build_binary_operator_bitwise_and(),
            BinaryOperator::BitwiseOr => self.package.build_binary_operator_bitwise_or(),
            BinaryOperator::BitwiseXor => self.package.build_binary_operator_bitwise_xor(),
            BinaryOperator::LeftShift => self.package.build_binary_operator_bitwise_shift_left(),
            BinaryOperator::RightShift => self.package.build_binary_operator_bitwise_shift_right(),
            BinaryOperator::LessThan => self.package.build_binary_operator_less_than(),
            BinaryOperator::LessThanOrEqual => {
                self.package.build_binary_operator_less_than_or_equal()
            }
            BinaryOperator::GreaterThan => self.package.build_binary_operator_greater_than(),
            BinaryOperator::GreaterThanOrEqual => {
                self.package.build_binary_operator_greater_than_or_equal()
            }
            BinaryOperator::Equal => self.package.build_binary_operator_equal(),
            BinaryOperator::NotEqual => self.package.build_binary_operator_not_equal(),
            BinaryOperator::LogicalAnd => self.package.build_binary_operator_logical_and(),
            BinaryOperator::LogicalOr => self.package.build_binary_operator_logical_or(),
            BinaryOperator::Assignment => todo!("assignment operator not yet implemented"),
        };

        let expression = self.package.build_expression_binary(operator, left, right);

        self.expression_stack.push(expression);
    }

    fn visit_type_cast(&mut self, cast: &tantalum_ast::TypeCast<'_, '_>) {
        self.visit_type(cast.ty.data());
        let ty = self
            .type_stack
            .pop()
            .expect("expected type to exist in stack");

        self.visit_expression(cast.value.data());
        let expression = self
            .expression_stack
            .pop()
            .expect("expected statement to exist in stack");

        let expression = self.package.build_expression_type_cast(ty, expression);

        self.expression_stack.push(expression);
    }

    fn visit_integer_literal(&mut self, integer: &Integer<'_, '_>) {
        let literal = self
            .package
            .build_integer_literal((*integer.value.data()).to_string(), integer.radix);

        let expression = self.package.build_expression_literal(literal);

        self.expression_stack.push(expression);
    }

    fn visit_float_literal(&mut self, float: &Float<'_, '_>) {
        let literal = self
            .package
            .build_float_literal((*float.value.data()).to_string());

        let expression = self.package.build_expression_literal(literal);

        self.expression_stack.push(expression);
    }

    fn visit_boolean_literal(&mut self, boolean: &Boolean<'_, '_>) {
        let literal = self.package.build_boolean_literal(
            (*boolean.value.data())
                .parse::<bool>()
                .expect("expected boolean value to be valid and verified by parser"),
        );

        let expression = self.package.build_expression_literal(literal);

        self.expression_stack.push(expression);
    }

    fn visit_character_literal(&mut self, character: &Character<'_, '_>) {
        todo!("character literals not yet implemented, did not lower {character:?}");
    }

    fn visit_string_literal(&mut self, string: &tantalum_ast::String<'_, '_>) {
        let literal = self
            .package
            .build_string_literal((*string.value.data()).to_string());

        let expression = self.package.build_expression_literal(literal);

        self.expression_stack.push(expression);
    }
}
