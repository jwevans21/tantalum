use core::cell::RefCell;
use std::{
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};

use crate::{HLIRFunctionPrototype, HLIRFunctionType, HLIRType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct ScopedValueIndex(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRScopeBlock {
    variables: BTreeMap<String, ScopedValueIndex>,
    parent: Option<Rc<RefCell<HLIRScopeBlock>>>,
}

impl HLIRScopeBlock {
    #[must_use]
    pub fn get(&self, name: &str) -> Option<ScopedValueIndex> {
        if let Some(index) = self.variables.get(name) {
            return Some(*index);
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }

        return None;
    }
}

impl Default for HLIRScopeBlock {
    fn default() -> Self {
        return Self {
            variables: BTreeMap::new(),
            parent: None,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HLIRScopeItemKind {
    Function(Rc<HLIRFunctionPrototype>),
    Variable(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HLIRScope {
    next_index: ScopedValueIndex,
    names: BTreeMap<ScopedValueIndex, (HLIRScopeItemKind, Option<Rc<HLIRType>>)>,
    global: Rc<RefCell<HLIRScopeBlock>>,
    frames: VecDeque<Rc<RefCell<HLIRScopeBlock>>>,
}

impl HLIRScope {
    #[must_use]
    pub fn new() -> Self {
        return Self {
            next_index: ScopedValueIndex(0),
            names: BTreeMap::new(),
            global: Rc::new(RefCell::new(HLIRScopeBlock::default())),
            frames: VecDeque::new(),
        };
    }

    fn get_current_frame(&self) -> &Rc<RefCell<HLIRScopeBlock>> {
        return self.frames.front().unwrap_or(&self.global);
    }

    pub fn enter(&mut self) {
        let new_block = Rc::new(RefCell::new(HLIRScopeBlock {
            variables: BTreeMap::new(),
            parent: Some(self.get_current_frame().clone()),
        }));

        self.frames.push_front(new_block);
    }

    /// # Panics
    ///
    /// Panics if there is no scope to exit (the global scope is always present
    /// and cannot be exited)
    pub fn exit(&mut self) -> Rc<RefCell<HLIRScopeBlock>> {
        let frame = self.frames.pop_front();

        return frame.expect("cannot exit global scope");
    }

    pub fn declare_function(&mut self, prototype: Rc<HLIRFunctionPrototype>) -> ScopedValueIndex {
        let index = self.next_index;

        self.next_index = ScopedValueIndex(index.0 + 1);

        self.get_current_frame()
            .borrow_mut()
            .variables
            .insert(prototype.name.clone(), index);

        let function_type = HLIRFunctionType {
            prototype: prototype.inner.clone(),
        };

        self.names.insert(
            index,
            (
                HLIRScopeItemKind::Function(prototype),
                Some(Rc::new(HLIRType::Function(function_type))),
            ),
        );

        return index;
    }

    pub fn declare_variable(&mut self, name: &str, ty: Option<Rc<HLIRType>>) -> ScopedValueIndex {
        let index = self.next_index;

        self.next_index = ScopedValueIndex(index.0 + 1);

        self.get_current_frame()
            .borrow_mut()
            .variables
            .insert(name.to_string(), index);

        self.names
            .insert(index, (HLIRScopeItemKind::Variable(name.to_string()), ty));

        return index;
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<ScopedValueIndex> {
        for frame in &self.frames {
            if let Some(index) = frame.borrow().variables.get(name) {
                return Some(*index);
            }
        }

        if let Some(index) = self.global.borrow().variables.get(name) {
            return Some(*index);
        }

        return None;
    }
}

impl Default for HLIRScope {
    fn default() -> Self {
        return Self::new();
    }
}
