use crate::statements::Statement;
use crate::types::{TypeId, TypeScopeId};
use crate::variables::{VariableId, VariableScopeBlockId};
use std::rc::Rc;

/// A prototype for a function in the HLIR.
///
/// Contains only the information required to call the function (ignoring a reference to the
/// function in the compiled code itself).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionPrototype {
    /// The type of the parameters to the function.
    ///
    /// These can be resolved from [`TypeId`]'s to [`Type`]'s using the [`Types`] struct.
    ///
    /// [`TypeId`]: crate::types::TypeId
    /// [`Type`]: crate::types::Type
    /// [`Types`]: crate::types::Types
    pub parameters: Vec<TypeId>,
    /// Whether the function is variadic.
    ///
    /// If this is `true`, the function can take `n` arguments where `n` is greater than or equal to
    /// the length of the `parameters` field.
    pub is_variadic: bool,
    /// The return type of the function.
    ///
    /// Can be resolved the same way as the `parameters` field.
    pub return_type: TypeId,
}

/// A function in the HLIR.
///
/// References a [`FunctionPrototype`] and contains the parameters and body of the function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub variable_scope: VariableScopeBlockId,
    pub type_scope: TypeScopeId,
    /// The prototype of the function.
    ///
    /// Defines the structural layout of the function.
    pub prototype: Rc<FunctionPrototype>,
    /// The parameter variables of the function.
    pub parameters: Vec<VariableId>,
    /// The body of the function. Can be either a single statement or a block of statements.
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionBuilder {
    variable_scope: VariableScopeBlockId,
    type_scope: TypeScopeId,
    pub prototype: Rc<FunctionPrototype>,
    parameters: Vec<VariableId>,
    body: Option<Statement>,
}

impl FunctionBuilder {
    pub fn new(
        variable_scope: VariableScopeBlockId,
        type_scope: TypeScopeId,
        prototype: Rc<FunctionPrototype>,
    ) -> Self {
        Self {
            variable_scope,
            type_scope,
            prototype,
            parameters: Vec::new(),
            body: None,
        }
    }

    pub fn add_parameter(&mut self, parameter: VariableId) {
        self.parameters.push(parameter);
    }

    pub fn set_body(&mut self, body: Statement) {
        self.body = Some(body);
    }

    /// Build the function based on the current state of the builder.
    ///
    /// If the body of the function is not set, this will return `None`.
    pub fn build(self) -> Option<Function> {
        Some(Function {
            variable_scope: self.variable_scope,
            type_scope: self.type_scope,
            prototype: self.prototype,
            parameters: self.parameters,
            body: self.body?,
        })
    }
}
