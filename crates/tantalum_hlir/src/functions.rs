use crate::items::{Function, FunctionPrototype};
use crate::path::Path;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FunctionId(usize);

impl core::fmt::Debug for FunctionId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "FunctionId({})", self.0)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Functions {
    next_id: FunctionId,
    known: HashMap<Path, FunctionId>,
    function_prototypes: HashMap<FunctionId, Rc<FunctionPrototype>>,
    function_impls: HashMap<FunctionId, Function>,
}

impl Functions {
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_id: FunctionId(0),
            known: HashMap::new(),
            function_prototypes: HashMap::new(),
            function_impls: HashMap::new(),
        }
    }

    pub fn prototypes(&self) -> impl Iterator<Item = (FunctionId, &FunctionPrototype)> {
        self.function_prototypes
            .iter()
            .map(|(id, prototype)| (*id, prototype.as_ref()))
    }

    pub fn impls(&self) -> impl Iterator<Item = (FunctionId, &Function)> {
        self.function_impls
            .iter()
            .map(|(id, function)| (*id, function))
    }

    pub fn next_id(&mut self) -> FunctionId {
        let id = self.next_id;
        self.next_id = FunctionId(id.0 + 1);
        id
    }

    #[must_use]
    pub fn get(&self, path: &Path) -> Option<FunctionId> {
        self.known.get(path).copied()
    }

    #[must_use]
    pub fn get_prototype(&self, id: FunctionId) -> Option<Rc<FunctionPrototype>> {
        self.function_prototypes.get(&id).cloned()
    }

    #[must_use]
    pub fn get_path(&self, id: &FunctionId) -> Option<&Path> {
        self.known.iter().find_map(|(path, function_id)| {
            if *function_id == *id {
                Some(path)
            } else {
                None
            }
        })
    }

    pub fn create_function(&mut self, path: Path, prototype: FunctionPrototype) -> FunctionId {
        let id = self.next_id();
        self.known.insert(path, id);
        self.function_prototypes.insert(id, Rc::new(prototype));
        id
    }

    pub fn insert(&mut self, id: FunctionId, function: Function) {
        debug_assert!(self.function_prototypes.contains_key(&id));
        debug_assert!(!self.function_impls.contains_key(&id));

        self.function_impls.insert(id, function);
    }
}

impl Default for Functions {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for Functions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Functions")
            .field("next_id", &self.next_id)
            .field_with("known", |f| {
                let mut known: Vec<_> = self.known.iter().collect();
                known.sort_by_key(|(_, id)| **id);
                f.debug_map().entries(known).finish()
            })
            .field_with("function_prototypes", |f| {
                let mut function_prototypes: Vec<_> = self.function_prototypes.iter().collect();
                function_prototypes.sort_by_key(|(id, _)| **id);
                f.debug_map().entries(function_prototypes).finish()
            })
            .field_with("function_impls", |f| {
                let mut function_impls: Vec<_> = self.function_impls.iter().collect();
                function_impls.sort_by_key(|(id, _)| **id);
                f.debug_map().entries(function_impls).finish()
            })
            .finish()
    }
}
