use crate::{types::TypeId, variables::scope::VariableScope};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::rc::Rc;

mod scope;

use crate::functions::FunctionId;
use crate::inference::InferenceId;
pub use scope::VariableScopeBlockId;

/// A unique identifier for a variable.
///
/// Used to reduce the size of variable references in the HLIR and to clearly distinguish between
/// different variables with the same name.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VariableId(usize);

impl core::fmt::Debug for VariableId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VariableId({})", self.0)
    }
}

/// A variable in the HLIR.
///
/// Variables have a unique identifier, a name, and a type.
///
/// The unique identifier is not stored in the variable itself, but is used to reference the variable
/// through the `known` map in the [`Variables`] struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub ty: InferenceId,
}

/// The type of a variable in the HLIR.
///
/// Variables can be either a normal variable or a function prototype. This allows for functions
/// to be resolved in the same way as variables.
#[derive(Clone, PartialEq, Eq)]
pub enum VariableType {
    /// This [`VariableId`] refers to a normal variable.
    Variable(Variable),
    /// This [`VariableId`] refers to a [`FunctionId`].
    ///
    /// [`FunctionId`]: crate::functions::FunctionId
    Function(FunctionId),
}

impl core::fmt::Debug for VariableType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VariableType::Variable(variable) => write!(f, "{variable:?}"),
            VariableType::Function(function_id) => write!(f, "{function_id:?}"),
        }
    }
}

/// The collection of variables in the HLIR.
#[derive(Clone, PartialEq, Eq)]
pub struct Variables {
    /// The next ID to assign to a variable.
    next_id: VariableId,
    /// The known variables in the HLIR.
    ///
    /// Variables are stored in a map with their unique identifier as the key.
    ///
    /// The unique identifier is found within the [`VariableScope`] struct.
    known: HashMap<VariableId, VariableType>,
    /// The current scope of all variables.
    scope: VariableScope,
}

impl Variables {
    pub fn new() -> Self {
        Self {
            next_id: VariableId(0),
            known: HashMap::new(),
            scope: VariableScope::new(),
        }
    }

    pub fn push_scope(&mut self) -> VariableScopeBlockId {
        self.scope.push_block()
    }

    pub fn pop_scope(&mut self) {
        self.scope.pop_block();
    }

    fn next_id(&mut self) -> VariableId {
        let id = self.next_id;
        self.next_id = VariableId(id.0 + 1);
        id
    }

    pub fn get_or_insert(&mut self, name: &str, ty: InferenceId) -> VariableId {
        if let Some(id) = self.get(name) {
            return id;
        }

        let id = self.next_id();
        self.known.insert(
            id,
            VariableType::Variable(Variable {
                name: name.to_string(),
                ty,
            }),
        );

        self.scope.insert(name, id);

        id
    }

    pub fn create_variable(&mut self, name: &str, ty: InferenceId) -> VariableId {
        let id = self.next_id();
        self.known.insert(
            id,
            VariableType::Variable(Variable {
                name: name.to_string(),
                ty,
            }),
        );

        self.scope.insert(name, id);

        id
    }

    pub fn get(&self, name: &str) -> Option<VariableId> {
        self.scope.get(name)
    }

    pub fn get_type(&self, id: VariableId) -> Option<InferenceId> {
        match self.known.get(&id) {
            Some(VariableType::Variable(variable)) => Some(variable.ty),
            _ => None,
        }
    }

    pub fn get_or_insert_function(&mut self, name: &str, function_id: FunctionId) -> VariableId {
        if let Some(id) = self.get(name) {
            return id;
        }

        let id = self.next_id();
        self.known.insert(id, VariableType::Function(function_id));

        self.scope.insert(name, id);

        id
    }
}

impl core::fmt::Debug for Variables {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Variables")
            .field("next_id", &self.next_id)
            .field_with("known", |f| {
                let mut known: Vec<_> = self.known.iter().collect();
                known.sort_by_key(|(id, _)| **id);

                f.debug_map().entries(known).finish()
            })
            .field("scope", &self.scope)
            .finish()
    }
}
