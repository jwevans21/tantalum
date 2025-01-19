use tantalum_hlir::HLIRPackage;

mod functions;
mod prototypes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASTLoweringContext {
    package: HLIRPackage,
    errors: Vec<String>,
}

impl ASTLoweringContext {
    #[must_use]
    pub fn new() -> Self {
        Self {
            package: HLIRPackage::new(),
            errors: Vec::new(),
        }
    }

    pub fn lower(&mut self, ast: &tantalum_ast::AST) {
        // TODO: Process types in AST to produce types

        // TODO: Process traits in AST to produce traits

        // TODO: Process functions in AST to produce prototypes

        prototypes::PrototypeLoweringContext::new(&mut self.package).lower(ast);

        // TODO: Process functions in AST to produce implementations

        functions::FunctionLoweringContext::new(&mut self.package).lower(ast);

        eprintln!("{:#?}", self.package);
    }

    #[must_use]
    pub fn finish(self) -> HLIRPackage {
        self.package
    }
}

impl Default for ASTLoweringContext {
    fn default() -> Self {
        Self::new()
    }
}
