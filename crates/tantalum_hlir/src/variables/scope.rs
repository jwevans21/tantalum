use crate::variables::VariableId;
use std::collections::HashMap;

/// A unique identifier for a block of variable scope.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VariableScopeBlockId(usize);

impl core::fmt::Debug for VariableScopeBlockId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VariableScopeBlockId({})", self.0)
    }
}

/// A scoped block of mappings from variable names to variable IDs.
///
/// Used to resolve variables in a hierarchical scope. Allows for shadowing of variable names.
///
/// Lookups are performed by traversing the scope hierarchy from the current block to
/// the root block with the `parent` scope block identifier.
#[derive(Clone, PartialEq, Eq)]
pub struct VariableScopeBlock {
    /// The parent block of this block. If this block is the root block, this will be `None`.
    parent: Option<VariableScopeBlockId>,
    /// The variables defined in this block.
    ///
    /// Used to replace variable names with their unique identifiers.
    names: HashMap<String, VariableId>,
}

impl VariableScopeBlock {
    pub fn new() -> Self {
        Self {
            parent: None,
            names: HashMap::new(),
        }
    }

    pub fn with_parent(parent: VariableScopeBlockId) -> Self {
        Self {
            parent: Some(parent),
            names: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, id: VariableId) {
        self.names.insert(name.to_string(), id);
    }

    pub fn get(&self, name: &str) -> Option<VariableId> {
        self.names.get(name).copied()
    }
}

impl Default for VariableScopeBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for VariableScopeBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VariableScopeBlock")
            .field_with("parent", |f| write!(f, "{:?}", self.parent))
            .field("names", &self.names)
            .finish()
    }
}

/// A hierarchical scope of variable mappings.
#[derive(Clone, PartialEq, Eq)]
pub struct VariableScope {
    /// The next ID to assign to a variable scope block.
    next_id: VariableScopeBlockId,
    /// The identifier of the current block.
    ///
    /// This is the block that new variables will be inserted into.
    ///
    /// The block is accessed through the `blocks` map.
    current_block: VariableScopeBlockId,
    /// A map of variable scope blocks.
    blocks: HashMap<VariableScopeBlockId, VariableScopeBlock>,
}

impl VariableScope {
    pub fn new() -> Self {
        Self {
            next_id: VariableScopeBlockId(1),
            current_block: VariableScopeBlockId(0),
            blocks: {
                let mut blocks = HashMap::new();
                blocks.insert(
                    VariableScopeBlockId(0),
                    VariableScopeBlock {
                        parent: None,
                        names: HashMap::new(),
                    },
                );
                blocks
            },
        }
    }

    fn next_id(&mut self) -> VariableScopeBlockId {
        let id = self.next_id;
        self.next_id = VariableScopeBlockId(id.0 + 1);
        id
    }

    pub fn push_block(&mut self) -> VariableScopeBlockId {
        let id = self.next_id();
        assert!(
            self.blocks
                .insert(id, VariableScopeBlock::with_parent(self.current_block))
                .is_none(),
            "block already exists"
        );
        self.current_block = id;
        id
    }

    pub fn pop_block(&mut self) -> Option<VariableScopeBlockId> {
        let current_block = self.current_block;
        self.current_block = self.blocks.get(&current_block)?.parent?;
        Some(current_block)
    }

    pub fn insert(&mut self, name: &str, id: VariableId) {
        self.blocks
            .get_mut(&self.current_block)
            .unwrap()
            .insert(name, id);
    }

    pub fn get(&self, name: &str) -> Option<VariableId> {
        let mut current_block = self.current_block;

        while let Some(block) = self.blocks.get(&current_block) {
            if let Some(id) = block.get(name) {
                return Some(id);
            }

            current_block = block.parent?;
        }
        None
    }
}

impl core::fmt::Debug for VariableScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VariableScope")
            .field("next_id", &self.next_id)
            .field("current_block", &self.current_block)
            .field_with("blocks", |f| {
                let mut blocks: Vec<_> = self.blocks.iter().collect();
                blocks.sort_by_key(|(id, _)| **id);

                f.debug_map()
                    .entries(blocks.iter().map(|(id, block)| (id, block)))
                    .finish()
            })
            .finish()
    }
}
