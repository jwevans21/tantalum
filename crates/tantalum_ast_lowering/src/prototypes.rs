use tantalum_hlir::{HLIRPackage, Path, PathSegment, TypeId};

pub struct PrototypeLoweringContext<'a> {
    package: &'a mut HLIRPackage,

    types: Vec<TypeId>,
}

impl<'a> PrototypeLoweringContext<'a> {
    pub fn new(package: &'a mut HLIRPackage) -> Self {
        Self {
            package,
            types: Vec::new(),
        }
    }

    fn void_type(&self) -> TypeId {
        self.package
            .get_type_id(&Path::new(vec![PathSegment::new("void".to_string())]))
            .expect("expected void type to exist in package")
    }

    pub fn lower(mut self, ast: &tantalum_ast::AST) {
        <Self as tantalum_ast::ASTVisitor>::visit_ast(&mut self, ast);
    }
}

impl tantalum_ast::ASTVisitor<'_, '_> for PrototypeLoweringContext<'_> {
    fn visit_function(&mut self, function: &tantalum_ast::Function<'_, '_>) {
        let mut parameters = Vec::new();
        let mut variadic = false;

        for parameter in function.parameters.data() {
            let parameter = parameter.data();
            match parameter {
                tantalum_ast::Parameter::Named(named) => {
                    self.visit_type(named.ty.data());

                    let parameter_type = self
                        .types
                        .pop()
                        .expect("expected type to have been visited");

                    parameters.push(parameter_type);
                }
                tantalum_ast::Parameter::Variadic => {
                    variadic = true;
                }
            }
        }

        let return_type = if let Some(return_type) = &function.return_type {
            self.visit_type(return_type.data());
            self.types
                .pop()
                .expect("expected type to have been visited")
        } else {
            self.void_type()
        };

        let prototype = self
            .package
            .build_function_prototype(parameters, variadic, return_type);

        self.package
            .create_function(Path::from(*(function.name.data())), prototype);
    }

    fn visit_external_function(
        &mut self,
        external_function: &tantalum_ast::ExternalFunction<'_, '_>,
    ) {
        let mut is_variadic = false;
        let mut parameters = Vec::new();

        for parameter in external_function.parameters.data() {
            match parameter.data() {
                tantalum_ast::Parameter::Named(named) => {
                    self.visit_type(named.ty.data());

                    let parameter_type = self
                        .types
                        .pop()
                        .expect("expected type to have been visited");

                    parameters.push(parameter_type);
                }
                tantalum_ast::Parameter::Variadic => {
                    is_variadic = true;
                }
            }
        }

        let return_type = if let Some(return_type) = &external_function.return_type {
            self.visit_type(return_type.data());
            self.types
                .pop()
                .expect("expected type to have been visited")
        } else {
            self.void_type()
        };

        let prototype = self
            .package
            .build_function_prototype(parameters, is_variadic, return_type);

        self.package
            .create_function(Path::from(*(external_function.name.data())), prototype);
    }

    fn visit_named_type(&mut self, named: &tantalum_ast::NamedType<'_, '_>) {
        let path = Path::from(*(named.name.data()));
        let type_id = self
            .package
            .get_type_id(&path)
            .expect("expected type to exist in package");
        self.types.push(type_id);
    }

    fn visit_pointer_type(&mut self, pointer: &tantalum_ast::PointerType<'_, '_>) {
        self.visit_type(pointer.ty.data());
        let ty = self
            .types
            .pop()
            .expect("expected type to have been visited");
        let pointer_type = self.package.build_type_pointer(ty);
        self.types.push(pointer_type);
    }

    fn visit_sized_array_type(&mut self, array: &tantalum_ast::SizedArrayType<'_, '_>) {
        self.visit_type(array.ty.data());
        let ty = self
            .types
            .pop()
            .expect("expected type to have been visited");
        let array_type = self.package.build_type_array(ty, *(array.size.data()));
        self.types.push(array_type);
    }

    fn visit_unsized_array_type(&mut self, array: &tantalum_ast::UnsizedArrayType<'_, '_>) {
        self.visit_type(array.ty.data());
        let ty = self
            .types
            .pop()
            .expect("expected type to have been visited");
        let array_type = self.package.build_type_unsized_array(ty);
        self.types.push(array_type);
    }
}
