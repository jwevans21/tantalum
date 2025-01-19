use crate::inference::TypeConstraint;
use crate::items::Function;
use crate::path::Path;
use crate::types::TypeId;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TraitId(usize);

impl core::fmt::Debug for TraitId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TraitId({})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TraitTypeParameterId(usize);

impl core::fmt::Debug for TraitTypeParameterId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TraitTypeParameterId({})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TraitMethodId(usize);

impl core::fmt::Debug for TraitMethodId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TraitMethodId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trait {
    // pub name: Path,
    next_parameter_id: TraitTypeParameterId,
    pub type_parameters: HashMap<String, TraitTypeParameterId>,
    next_method_id: TraitMethodId,
    pub methods: HashMap<String, TraitMethodId>,
    pub method_definitions: HashMap<TraitMethodId, TraitMethod>,
}

impl Trait {
    pub fn new() -> Self {
        Self {
            next_parameter_id: TraitTypeParameterId(0),
            type_parameters: HashMap::new(),
            next_method_id: TraitMethodId(0),
            methods: HashMap::new(),
            method_definitions: HashMap::new(),
        }
    }

    fn next_parameter_id(&mut self) -> TraitTypeParameterId {
        let id = self.next_parameter_id;
        self.next_parameter_id = TraitTypeParameterId(id.0 + 1);
        id
    }

    fn next_method_id(&mut self) -> TraitMethodId {
        let id = self.next_method_id;
        self.next_method_id = TraitMethodId(id.0 + 1);
        id
    }
    
    pub fn get_type_parameter(&self, name: &str) -> Option<TraitTypeParameterId> {
        self.type_parameters.get(name).copied()
    }

    pub fn add_type_parameter(&mut self, name: String) -> TraitTypeParameterId {
        let id = self.next_parameter_id();
        self.type_parameters.insert(name, id);
        id
    }

    pub fn add_type_parameter_constraint(
        &mut self,
        id: TraitTypeParameterId,
        constraint: TypeConstraint,
    ) {
        debug_assert!(
            self.type_parameters.values().any(|&x| x == id),
            "Type parameter ID not found for current trait"
        );
    }

    pub fn add_method(
        &mut self,
        name: String,
        parameters: Vec<TraitTypeParameterId>,
        return_type: TraitTypeParameterId,
    ) -> TraitMethodId {
        let id = self.next_method_id();
        self.methods.insert(name.clone(), id);
        self.method_definitions.insert(
            id,
            TraitMethod {
                name,
                parameters,
                return_type,
            },
        );
        id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitImpl {
    pub trait_id: TraitId,
    pub type_id: TypeId,
    pub type_constraints: HashMap<TraitTypeParameterId, Vec<TypeConstraint>>,
    pub method_impls: HashMap<TraitMethodId, Function>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitMethod {
    pub name: String,
    // TODO: add other fields for a trait method
    pub parameters: Vec<TraitTypeParameterId>,
    pub return_type: TraitTypeParameterId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Traits {
    next_id: TraitId,
    known: HashMap<Path, TraitId>,
    trait_defs: HashMap<TraitId, Trait>,
    trait_impls: HashMap<TraitId, HashMap<TypeId, TraitImpl>>,
}

impl Traits {
    pub fn new() -> Self {
        Self {
            next_id: TraitId(0),
            known: HashMap::new(),
            trait_defs: HashMap::new(),
            trait_impls: HashMap::new(),
        }
    }

    fn next_id(&mut self) -> TraitId {
        let id = self.next_id;
        self.next_id = TraitId(id.0 + 1);
        id
    }

    pub fn get_or_insert(&mut self, name: Path) -> TraitId {
        if let Some(id) = self.known.get(&name) {
            return *id;
        }

        let id = self.next_id();
        self.known.insert(name, id);
        self.trait_defs.insert(id, Trait::new());
        self.trait_impls.insert(id, HashMap::new());
        id
    }

    pub fn get_trait(&self, id: TraitId) -> Option<&Trait> {
        self.trait_defs.get(&id)
    }

    pub fn get_trait_mut(&mut self, id: TraitId) -> Option<&mut Trait> {
        self.trait_defs.get_mut(&id)
    }

    pub fn get_trait_impls(&self, id: TraitId) -> Option<&HashMap<TypeId, TraitImpl>> {
        self.trait_impls.get(&id)
    }

    pub fn get_trait_impl(&self, trait_id: TraitId, type_id: TypeId) -> Option<&TraitImpl> {
        self.trait_impls.get(&trait_id)?.get(&type_id)
    }

    pub fn insert_trait_impl(&mut self, impl_: TraitImpl) {
        self.trait_impls
            .entry(impl_.trait_id)
            .or_default()
            .insert(impl_.type_id, impl_);
    }
}
