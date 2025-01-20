use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{
    FileType, InitializationConfig, Target, TargetMachine, TargetMachineOptions,
};
use inkwell::types::{AnyTypeEnum, BasicTypeEnum, StringRadix};
use inkwell::values::{AnyValue, AnyValueEnum, BasicValueEnum, FunctionValue};
use inkwell::AddressSpace;
use std::collections::HashMap;
use tantalum_hlir::{
    BinaryOperation, BinaryOperator, Expression, FunctionCall, FunctionId, HLIRPackage, Let,
    Literal, LiteralValue, PrimitiveType, Return, Statement, Type, TypeId, VariableId,
};

#[derive(Debug)]
pub struct LLVMCodegenContext<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    // package: HLIRPackage,
    types: HashMap<TypeId, AnyTypeEnum<'ctx>>,
    functions: HashMap<FunctionId, FunctionValue<'ctx>>,
    values: HashMap<VariableId, AnyValueEnum<'ctx>>,
}

impl<'ctx> LLVMCodegenContext<'ctx> {
    #[must_use]
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            module: context.create_module("main"),
            builder: context.create_builder(),

            // package,
            types: HashMap::new(),
            functions: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn dump(&self) {
        self.module.print_to_stderr();
    }

    pub fn dump_to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn emit_ll(&self) -> String {
        self.dump_to_string()
    }

    pub fn emit_bc(&self) -> Vec<u8> {
        self.module.write_bitcode_to_memory().as_slice().to_vec()
    }

    /// # Errors
    ///
    /// Returns an error if the target machine could not be initialized or
    /// if the target machine could not write the assembly to the output file.
    pub fn compile(&self, output: impl AsRef<std::path::Path>) -> Result<(), String> {
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|err| err.to_string())?;

        let triple = TargetMachine::get_default_triple();

        let target = Target::from_triple(&triple).map_err(|err| err.to_string())?;

        let options = TargetMachineOptions::default();

        let target_machine = target
            .create_target_machine_from_options(&triple, options)
            .ok_or_else(|| "failed to create target machine".to_string())?;

        target_machine
            .write_to_file(&self.module, FileType::Assembly, output.as_ref())
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if the module is invalid.
    pub fn verify(&self) -> Result<(), String> {
        self.module.verify().map_err(|err| err.to_string())
    }

    pub fn build(&mut self, package: &HLIRPackage) {
        self.build_types(package);

        self.build_prototypes(package);

        self.build_functions(package);
    }

    // fn build_types(&mut self) {
    //     for (id, ty) in self.package.types() {
    //         self.build_type(id, ty);
    //     }
    // }
    fn build_types(&mut self, package: &HLIRPackage) {
        for (id, ty) in package.types() {
            let ty = match ty {
                Type::Primitive(primitive) => match primitive {
                    PrimitiveType::Void => self.context.void_type().into(),
                    PrimitiveType::I8 | PrimitiveType::U8 | PrimitiveType::Char => {
                        self.context.i8_type().into()
                    }
                    PrimitiveType::I16 | PrimitiveType::U16 => self.context.i16_type().into(),
                    PrimitiveType::I32 | PrimitiveType::U32 => self.context.i32_type().into(),
                    PrimitiveType::I64 | PrimitiveType::U64 => self.context.i64_type().into(),
                    PrimitiveType::F32 => self.context.f32_type().into(),
                    PrimitiveType::F64 => self.context.f64_type().into(),
                    PrimitiveType::Bool => self.context.bool_type().into(),
                    PrimitiveType::Str => self.context.ptr_type(AddressSpace::default()).into(),
                },
                Type::Ptr(_) | Type::SizedArray(_, _) | Type::UnsizedArray(_) => {
                    self.context.ptr_type(AddressSpace::default()).into()
                }
                Type::Unresolved(_) => panic!("unsupported type {ty:?}"),
            };

            self.types.insert(id, ty);
        }
    }

    fn build_prototypes(&mut self, package: &HLIRPackage) {
        for (id, name, prototype) in package.prototypes() {
            let return_ty = self.types[&prototype.return_type];
            let param_tys = prototype
                .parameters
                .iter()
                .map(|id| {
                    self.types[id]
                        .try_into()
                        .expect("expected type to be a basic type")
                })
                .collect::<Vec<_>>();

            let function_type = match return_ty {
                AnyTypeEnum::VoidType(ty) => ty.fn_type(&param_tys, false),
                AnyTypeEnum::IntType(ty) => ty.fn_type(&param_tys, false),
                AnyTypeEnum::FloatType(ty) => ty.fn_type(&param_tys, false),
                AnyTypeEnum::PointerType(ty) => ty.fn_type(&param_tys, false),
                _ => panic!("unsupported return type {return_ty:?}"),
            };

            let function = self.module.add_function(&name, function_type, None);

            if name == "__main" {
                let main = self.module.add_function("main", function_type, None);
                let bb = self.context.append_basic_block(main, "entry");
                self.builder.position_at_end(bb);
                let ret = self
                    .builder
                    .build_call(function, &[], "")
                    .expect("failed to call function")
                    .try_as_basic_value()
                    .left()
                    .expect("expected basic value");
                self.builder
                    .build_return(Some(&ret))
                    .expect("failed to build return");
            }

            self.functions.insert(id, function);
        }
    }

    fn build_functions(&mut self, package: &HLIRPackage) {
        for (id, body) in package.impls() {
            let function = self.functions[&id];
            let entry = self.context.append_basic_block(function, "entry");

            self.builder.position_at_end(entry);

            for (i, arg) in function.get_param_iter().enumerate() {
                let variable = body.parameters[i];
                let name = package.get_variable_name(&variable).to_string();
                arg.set_name(&name);
                self.values.insert(variable, arg.into());
            }

            self.build_statement(/* function, */ &body.body, package)
                .expect("failed to build statement");
        }
    }

    fn build_statement(
        &mut self,
        /* func: FunctionValue<'ctx>, */
        statement: &Statement,
        package: &HLIRPackage,
    ) -> Result<AnyValueEnum<'ctx>, BuilderError> {
        Ok(match statement {
            Statement::Block(block) => {
                let mut last = None;
                for statement in &block.statements {
                    last = Some(self.build_statement(/* func, */ statement, package)?);
                }
                last.unwrap()
            }
            Statement::Let(Let { variable, value }) => {
                let value = self.build_expression(value, package)?;
                let value: BasicValueEnum<'ctx> = value.try_into().expect("expected value");
                let name = package.get_variable_name(variable).to_string();
                let alloca = self.builder.build_alloca(value.get_type(), &name)?;
                self.builder.build_store(alloca, value)?;
                self.values.insert(*variable, alloca.into());
                alloca.into()
            }
            Statement::Return(Return { value: None }) => self.builder.build_return(None)?.into(),
            Statement::Return(Return { value: Some(value) }) => {
                let value: BasicValueEnum<'ctx> = self
                    .build_expression(value, package)?
                    .try_into()
                    .expect("expected value");
                self.builder.build_return(Some(&value))?.into()
            }
            Statement::Expression(expr) => self.build_expression(expr, package)?,
            _ => todo!(),
        })
    }

    fn build_expression(
        &mut self,
        expression: &Expression,
        package: &HLIRPackage,
    ) -> Result<AnyValueEnum<'ctx>, BuilderError> {
        match expression {
            Expression::Variable(variable) => {
                let value = self.values[variable];
                let expected_ty = package
                    .get_resolved_type(expression.ty(package))
                    .expect("unresolved type");

                let expected_ty = self.types[&expected_ty];

                Ok(match (value, expected_ty) {
                    (AnyValueEnum::PointerValue(ptr), AnyTypeEnum::PointerType(_)) => {
                        ptr.as_any_value_enum()
                    }
                    (AnyValueEnum::PointerValue(ptr), ty) => {
                        let ty: BasicTypeEnum = ty.try_into().expect("expected basic type");
                        self.builder.build_load(ty, ptr, "")?.into()
                    }
                    (value, _) => value,
                })
            }
            Expression::Literal(literal) => self.build_literal(literal, package),
            Expression::FunctionCall(FunctionCall {
                function,
                arguments,
                result: _,
            }) => {
                let function = self.functions[function];
                let arguments = arguments
                    .iter()
                    .map(|arg| self.build_expression(arg, package))
                    .filter_map(Result::ok)
                    .map(|arg| arg.try_into().expect("expected value"))
                    .collect::<Vec<_>>();

                let result = self
                    .builder
                    .build_call(function, &arguments, "")?
                    .as_any_value_enum();

                Ok(result)
            }
            Expression::BinaryOperation(BinaryOperation {
                left,
                operator,
                right,
                result,
            }) => {
                let left = self.build_expression(left, package)?;
                let right = self.build_expression(right, package)?;

                let left: BasicValueEnum<'ctx> = left.try_into().expect("expected value");
                let right: BasicValueEnum<'ctx> = right.try_into().expect("expected value");

                let result_ty = package.get_resolved_type(*result).expect("unresolved type");
                let result_ty = self.types[&result_ty];

                macro_rules! create_operation {
                    ($($op:ident => { $int:ident, $float:ident }),*) => {
                        match operator {
                            $(
                                BinaryOperator::$op => match result_ty {
                                    AnyTypeEnum::IntType(_) => self.builder.$int(left.into_int_value(), right.into_int_value(), "")?.as_any_value_enum(),
                                    AnyTypeEnum::FloatType(_) => self.builder.$float(left.into_float_value(), right.into_float_value(), "")?.as_any_value_enum(),
                                    _ => panic!("unsupported type {result_ty:?}"),
                                },
                            )*
                            _ => todo!(),
                        }
                    };
                }

                let result = create_operation!(
                    Addition => { build_int_add, build_float_add },
                    Subtraction => { build_int_sub, build_float_sub },
                    Multiplication => { build_int_mul, build_float_mul },
                    Division => { build_int_unsigned_div, build_float_div },
                    Remainder => { build_int_signed_rem, build_float_rem }
                );

                Ok(result)
            }
            _ => todo!(),
        }
    }

    fn build_literal(
        &self,
        literal: &Literal,
        package: &HLIRPackage,
    ) -> Result<AnyValueEnum<'ctx>, BuilderError> {
        match &literal.value {
            LiteralValue::Integer { value, radix } => {
                let literal_ty = package
                    .get_resolved_type(literal.ty)
                    .expect("unresolved type");
                let ty = self.types[&literal_ty];

                assert!(ty.is_int_type());

                let ty = ty.into_int_type();
                Ok(ty
                    .const_int_from_string(
                        value,
                        match radix {
                            2 => StringRadix::Binary,
                            8 => StringRadix::Octal,
                            10 => StringRadix::Decimal,
                            16 => StringRadix::Hexadecimal,
                            _ => panic!("unsupported radix {radix}"),
                        },
                    )
                    .expect("failed to parse integer")
                    .into())
            }
            _ => todo!(),
        }
    }
}
