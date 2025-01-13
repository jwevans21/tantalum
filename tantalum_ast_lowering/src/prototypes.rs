use std::rc::Rc;

use tantalum_hlir::{
    HLIRFunctionPrototype, HLIRFunctionPrototypeAnonymous, HLIRFunctionType, HLIRPackage, HLIRPath,
    HLIRType,
};

struct CurrentPrototype {
    name: String,
    parameters: Vec<(String, Rc<HLIRType>)>,
    return_type: Option<Rc<HLIRType>>,
    is_variadic: bool,
}

pub struct PrototypeExtractor<'a> {
    package: &'a mut HLIRPackage,
    current_prototype: Option<CurrentPrototype>,
    current_type: Option<Rc<HLIRType>>,
}

impl<'a> PrototypeExtractor<'a> {
    pub fn new(package: &'a mut HLIRPackage) -> Self {
        Self {
            package,
            current_prototype: None,
            current_type: None,
        }
    }

    pub fn lower(mut self, ast: &tantalum_ast::AST) {
        <Self as tantalum_ast::ASTVisitor>::visit_ast(&mut self, ast);
    }
}

impl<'file_name, 'source> tantalum_ast::ASTVisitor<'file_name, 'source> for PrototypeExtractor<'_> {
    fn visit_function(&mut self, function: &tantalum_ast::Function<'file_name, 'source>) {
        self.current_prototype = Some(CurrentPrototype {
            name: (*function.name.data()).to_string(),
            parameters: Vec::new(),
            return_type: None,
            is_variadic: false,
        });

        for parameter in function.parameters.data() {
            self.visit_parameter(parameter.data());
        }

        match &function.return_type {
            Some(return_type) => {
                self.visit_type(return_type.data());
                let return_type = self
                    .current_type
                    .take()
                    .expect("current_type should be set after visiting type");

                self.current_prototype
                    .as_mut()
                    .expect("current_prototype should be set before visiting return type")
                    .return_type = Some(return_type);
            }
            None => {
                self.current_prototype
                    .as_mut()
                    .expect("current_prototype should be set before visiting return type")
                    .return_type = Some(
                    self.package
                        .types
                        .get(&HLIRPath::new(vec!["void".to_string()]))
                        .expect("void type should be in package")
                        .clone(),
                );
            }
        }

        let current_prototype = self
            .current_prototype
            .take()
            .expect("current_prototype should be set after visiting parameters and return type");

        let prototype = HLIRFunctionPrototype {
            name: current_prototype.name,
            inner: Rc::new(HLIRFunctionPrototypeAnonymous {
                parameters: current_prototype
                    .parameters
                    .iter()
                    .map(|(_, ty)| ty.clone())
                    .collect(),
                is_variadic: current_prototype.is_variadic,
                return_type: current_prototype
                    .return_type
                    .expect("return type should be set"),
            }),
            parameter_names: current_prototype
                .parameters
                .into_iter()
                .map(|(name, _)| name)
                .collect(),
        };

        self.package.declare_function(prototype);
    }

    fn visit_external_function(
        &mut self,
        external_function: &tantalum_ast::ExternalFunction<'file_name, 'source>,
    ) {
        self.current_prototype = Some(CurrentPrototype {
            name: (*external_function.name.data()).to_string(),
            parameters: Vec::new(),
            return_type: None,
            is_variadic: false,
        });

        for parameter in external_function.parameters.data() {
            self.visit_parameter(parameter.data());
        }

        match &external_function.return_type {
            Some(return_type) => {
                self.visit_type(return_type.data());
                let return_type = self
                    .current_type
                    .take()
                    .expect("current_type should be set after visiting type");

                self.current_prototype
                    .as_mut()
                    .expect("current_prototype should be set before visiting return type")
                    .return_type = Some(return_type);
            }
            None => {
                self.current_prototype
                    .as_mut()
                    .expect("current_prototype should be set before visiting return type")
                    .return_type = Some(
                    self.package
                        .types
                        .get(&HLIRPath::new(vec!["void".to_string()]))
                        .expect("void type should be in package")
                        .clone(),
                );
            }
        }

        let current_prototype = self
            .current_prototype
            .take()
            .expect("current_prototype should be set after visiting parameters and return type");

        let prototype = HLIRFunctionPrototype {
            name: current_prototype.name,
            inner: Rc::new(HLIRFunctionPrototypeAnonymous {
                parameters: current_prototype
                    .parameters
                    .iter()
                    .map(|(_, ty)| ty.clone())
                    .collect(),
                is_variadic: current_prototype.is_variadic,
                return_type: current_prototype
                    .return_type
                    .expect("return type should be set"),
            }),
            parameter_names: current_prototype
                .parameters
                .into_iter()
                .map(|(name, _)| name)
                .collect(),
        };

        self.package.declare_function(prototype);
    }

    fn visit_named_parameter(&mut self, named: &tantalum_ast::NamedParameter<'file_name, 'source>) {
        let name = (*named.name.data()).to_string();

        self.visit_type(named.ty.data());

        let ty = self
            .current_type
            .take()
            .expect("current_type should be set after visiting type");

        self.current_prototype
            .as_mut()
            .expect("current_prototype should be set before visiting parameters")
            .parameters
            .push((name, ty));
    }

    fn visit_variadic_parameter(&mut self) {
        self.current_prototype
            .as_mut()
            .expect("current_prototype should be set before visiting parameters")
            .is_variadic = true;
    }

    fn visit_named_type(&mut self, named: &tantalum_ast::NamedType<'file_name, 'source>) {
        let path = HLIRPath::new(vec![(*named.name.data()).to_string()]);

        let ty = self
            .package
            .types
            .get(&path)
            .expect("type should be in package")
            .clone();

        self.current_type = Some(ty);
    }

    fn visit_function_type(&mut self, function: &tantalum_ast::FunctionType<'file_name, 'source>) {
        let prototype = HLIRFunctionPrototypeAnonymous {
            parameters: function
                .parameters
                .iter()
                .map(|ty| {
                    self.visit_type(ty.data());
                    self.current_type
                        .take()
                        .expect("current_type should be set after visiting type")
                })
                .collect(),
            return_type: {
                self.visit_type(function.return_type.data());
                self.current_type
                    .take()
                    .expect("current_type should be set after visiting type")
            },
            is_variadic: function.is_variadic,
        };

        self.current_type = Some(Rc::new(HLIRType::Function(HLIRFunctionType {
            prototype: Rc::new(prototype),
        })));
    }

    fn visit_pointer_type(&mut self, pointer: &tantalum_ast::PointerType<'file_name, 'source>) {
        self.visit_type(pointer.ty.data());

        let ty = self
            .current_type
            .take()
            .expect("current_type should be set after visiting type");
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
            .expect("current_type should be set after visiting type");
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
            .expect("current_type should be set after visiting type");
        self.current_type = Some(Rc::new(HLIRType::UnsizedArray(ty)));
    }

    fn visit_const_type(&mut self, constant: &tantalum_ast::ConstType<'file_name, 'source>) {
        self.visit_type(constant.ty.data());

        let ty = self
            .current_type
            .take()
            .expect("current_type should be set after visiting type");
        self.current_type = Some(Rc::new(HLIRType::Const(ty)));
    }

    fn visit_block(&mut self, _: &tantalum_ast::Block<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_variable_declaration(
        &mut self,
        _: &tantalum_ast::VariableDeclaration<'file_name, 'source>,
    ) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_if(&mut self, _: &tantalum_ast::If<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_while(&mut self, _: &tantalum_ast::While<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_for_init_cond_update(
        &mut self,
        _: &tantalum_ast::ForInitCondUpdate<'file_name, 'source>,
    ) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_break(&mut self) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_continue(&mut self) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_return(&mut self, _: &tantalum_ast::Return<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_variable(&mut self, _: &tantalum_ast::Variable<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_function_call(&mut self, _: &tantalum_ast::FunctionCall<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_member_access(&mut self, _: &tantalum_ast::MemberAccess<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_array_access(&mut self, _: &tantalum_ast::Index<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_unary_operation(&mut self, _: &tantalum_ast::UnaryOperation<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_binary_operation(&mut self, _: &tantalum_ast::BinaryOperation<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_type_cast(&mut self, _: &tantalum_ast::TypeCast<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_integer_literal(&mut self, _: &tantalum_ast::Integer<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_float_literal(&mut self, _: &tantalum_ast::Float<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_boolean_literal(&mut self, _: &tantalum_ast::Boolean<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_character_literal(&mut self, _: &tantalum_ast::Character<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }

    fn visit_string_literal(&mut self, _: &tantalum_ast::String<'file_name, 'source>) {
        panic!("PrototypeExtractor should only visit function items (functions, external functions, etc.)");
    }
}
