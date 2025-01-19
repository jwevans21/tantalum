use crate::{path::Path, types::TypeId};
use std::collections::HashMap;
use std::fmt::Formatter;

/// A unique identifier for a type scope block.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TypeScopeId(usize);

impl core::fmt::Debug for TypeScopeId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TypeScopeId({})", self.0)
    }
}

/// A scoped block of mappings from paths to type IDs.
///
/// Used to resolve types in a hierarchical scope. Allows for shadowing of type names.
///
/// Lookups are performed by traversing the scope hierarchy from the current block to
/// the root block with the `parent` scope block identifier.
#[derive(Clone, PartialEq, Eq)]
pub struct TypeScopeBlock {
    /// The parent block of this block. If this block is the root block, this will be `None`.
    parent: Option<TypeScopeId>,
    /// The types defined in this block.
    types: HashMap<Path, TypeId>,
}

impl TypeScopeBlock {
    pub fn new() -> Self {
        Self {
            parent: None,
            types: HashMap::new(),
        }
    }

    pub fn with_parent(parent: TypeScopeId) -> Self {
        Self {
            parent: Some(parent),
            types: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: Path, id: TypeId) {
        self.types.insert(path, id);
    }

    pub fn get(&self, path: &Path) -> Option<TypeId> {
        self.types.get(path).copied()
    }
}

impl core::fmt::Debug for TypeScopeBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TypeScopeBlock")
            .field("parent", &self.parent)
            .field_with("types", |f| {
                let mut types: Vec<_> = self.types.iter().collect();
                types.sort_by_key(|(_, id)| **id);
                f.debug_map().entries(types).finish()
            })
            .finish()
    }
}

/// A hierarchical scope of type mappings.
#[derive(Clone, PartialEq, Eq)]
pub struct TypeScope {
    /// The next ID to assign to a type scope block.
    next_id: TypeScopeId,
    /// The identifier of the current block.
    ///
    /// This is the block that new types will be inserted into.
    ///
    /// The block is accessed through the `blocks` map.
    current_block: TypeScopeId,
    /// A map of type scope blocks.
    blocks: HashMap<TypeScopeId, TypeScopeBlock>,
}

impl TypeScope {
    pub fn new() -> Self {
        Self {
            next_id: TypeScopeId(1),
            current_block: TypeScopeId(0),
            blocks: {
                let mut blocks = HashMap::new();
                blocks.insert(TypeScopeId(0), TypeScopeBlock::new());
                blocks
            },
        }
    }

    fn next_id(&mut self) -> TypeScopeId {
        let id = self.next_id;
        self.next_id = TypeScopeId(id.0 + 1);
        id
    }

    pub fn push_block(&mut self) -> TypeScopeId {
        let id = self.next_id();
        self.blocks
            .insert(id, TypeScopeBlock::with_parent(self.current_block));
        self.current_block = id;
        id
    }

    pub fn pop_block(&mut self) -> Option<TypeScopeId> {
        let current_block = self.current_block;
        self.current_block = self.blocks.get(&current_block)?.parent?;
        Some(current_block)
    }

    pub fn insert(&mut self, path: Path, id: TypeId) {
        self.blocks
            .get_mut(&self.current_block)
            .unwrap()
            .insert(path, id);
    }

    pub fn get(&self, path: &Path) -> Option<TypeId> {
        let mut current_block = self.current_block;

        while let Some(block) = self.blocks.get(&current_block) {
            if let Some(id) = block.get(path) {
                return Some(id);
            }

            current_block = block.parent?;
        }

        None
    }
}

impl core::fmt::Debug for TypeScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeScope")
            .field("next_id", &self.next_id)
            .field("current_block", &self.current_block)
            .field_with("blocks", |f| {
                let mut blocks: Vec<_> = self.blocks.iter().collect();
                blocks.sort_by_key(|(id, _)| **id);
                f.debug_map().entries(blocks).finish()
            })
            .finish()
    }
}
