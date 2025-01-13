use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

mod expressions;
mod items;
mod literals;
mod scope;
mod statements;
mod types;

pub use expressions::*;
pub use items::*;
pub use literals::*;
pub use scope::*;
pub use statements::*;
pub use types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRPackage {
    pub types: BTreeMap<HLIRPath, Rc<HLIRType>>,

    pub scope: HLIRScope,

    pub prototypes: BTreeMap<String, Rc<HLIRFunctionPrototype>>,
    pub functions: BTreeMap<Rc<HLIRFunctionPrototype>, HLIRFunction>,
    pub literals: BTreeSet<Rc<HLIRLiteralInner>>,
}

impl HLIRPackage {
    fn default_types() -> BTreeMap<HLIRPath, Rc<HLIRType>> {
        let mut types = BTreeMap::new();

        types.insert(
            HLIRPath::new(vec!["i8".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::I8)),
        );
        types.insert(
            HLIRPath::new(vec!["i16".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::I16)),
        );
        types.insert(
            HLIRPath::new(vec!["i32".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::I32)),
        );
        types.insert(
            HLIRPath::new(vec!["i64".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::I64)),
        );
        types.insert(
            HLIRPath::new(vec!["u8".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::U8)),
        );
        types.insert(
            HLIRPath::new(vec!["u16".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::U16)),
        );
        types.insert(
            HLIRPath::new(vec!["u32".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::U32)),
        );
        types.insert(
            HLIRPath::new(vec!["u64".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::U64)),
        );
        types.insert(
            HLIRPath::new(vec!["f32".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::F32)),
        );
        types.insert(
            HLIRPath::new(vec!["f64".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::F64)),
        );
        types.insert(
            HLIRPath::new(vec!["bool".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::Bool)),
        );
        types.insert(
            HLIRPath::new(vec!["char".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::Char)),
        );
        types.insert(
            HLIRPath::new(vec!["str".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::Str)),
        );
        types.insert(
            HLIRPath::new(vec!["void".to_string()]),
            Rc::new(HLIRType::Builtin(HLIRBuiltinType::Void)),
        );

        types
    }

    #[must_use]
    pub fn new() -> Self {
        Self {
            types: Self::default_types(),
            scope: HLIRScope::new(),

            prototypes: BTreeMap::new(),
            functions: BTreeMap::new(),
            literals: BTreeSet::new(),
        }
    }

    pub fn declare_function(&mut self, prototype: HLIRFunctionPrototype) -> ScopedValueIndex {
        let prototype = Rc::new(prototype);

        let index = self.scope.declare_function(prototype.clone());
        self.prototypes.insert(prototype.name.clone(), prototype);

        index
    }

    pub fn declare_variable(&mut self, name: &str, ty: Option<Rc<HLIRType>>) -> ScopedValueIndex {
        self.scope.declare_variable(name, ty)
    }

    pub fn declare_literal(&mut self, literal: HLIRLiteralInner) -> Rc<HLIRLiteralInner> {
        if let Some(existing) = self.literals.get(&literal) {
            return existing.clone();
        }

        let literal = Rc::new(literal);
        self.literals.insert(literal.clone());
        return literal;
    }
}

impl Default for HLIRPackage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRPath {
    pub components: Vec<String>,
}

impl HLIRPath {
    #[must_use]
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn push(&mut self, component: String) {
        self.components.push(component);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.components.pop()
    }

    #[must_use]
    pub fn last(&self) -> Option<&String> {
        self.components.last()
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.components.iter()
    }
}
