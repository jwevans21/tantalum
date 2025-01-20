use crate::{path::Path, types::scope::TypeScope};
use std::{collections::HashMap, rc::Rc};

mod scope;

use crate::inference::InferenceId;
pub use scope::TypeScopeId;

/// A unique identifier for a type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TypeId(usize);

impl core::fmt::Debug for TypeId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TypeId({})", self.0)
    }
}

/// Represents a type in the HLIR.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    /// All types that are not yet known (e.g. variables with no explicit type).
    ///
    /// A list of constraints that the type must satisfy will be assembled during type inference.
    Unresolved(InferenceId),
    /// A primitive type.
    ///
    /// These are types that are built into the language.
    Primitive(PrimitiveType),
    /// A pointer to another type.
    ///
    /// Contains the type that the pointer points to.
    Ptr(TypeId),
    /// An array of a type with a fixed length.
    ///
    /// Contains the type of the elements in the array and the length of the array.
    SizedArray(TypeId, usize),
    /// An unsized array of a type.
    ///
    /// Contains the type of the elements in the array.
    UnsizedArray(TypeId),
}

impl core::fmt::Debug for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Type::Unresolved(id) => write!(f, "Unresolved({id:?})"),
            Type::Primitive(ty) => write!(f, "{ty:?}"),
            Type::Ptr(ty) => write!(f, "Ptr({ty:?})"),
            Type::SizedArray(ty, len) => write!(f, "Array({ty:?}; {len})"),
            Type::UnsizedArray(ty) => write!(f, "Array({ty:?})"),
        }
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Type::Unresolved(_) => write!(f, "unresolved"),
            Type::Primitive(ty) => write!(f, "{ty}"),
            Type::Ptr(ty) => write!(f, "*{ty:?}"),
            Type::SizedArray(ty, len) => write!(f, "[{ty:?}; {len}]"),
            Type::UnsizedArray(ty) => write!(f, "[{ty:?}]"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrimitiveType {
    /// A type that represents the absence of a value.
    Void,
    /// A signed 8-bit integer.
    I8,
    /// A signed 16-bit integer.
    I16,
    /// A signed 32-bit integer.
    I32,
    /// A signed 64-bit integer.
    I64,
    /// An unsigned 8-bit integer.
    U8,
    /// An unsigned 16-bit integer.
    U16,
    /// An unsigned 32-bit integer.
    U32,
    /// An unsigned 64-bit integer.
    U64,
    /// A 32-bit floating point number.
    F32,
    /// A 64-bit floating point number.
    F64,
    /// A boolean value. Can be either `true` or `false`.
    Bool,
    /// A single character.
    Char,
    /// An array of characters with a defined and fixed length.
    Str,
}

impl core::fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PrimitiveType::Void => write!(f, "void"),
            PrimitiveType::I8 => write!(f, "i8"),
            PrimitiveType::I16 => write!(f, "i16"),
            PrimitiveType::I32 => write!(f, "i32"),
            PrimitiveType::I64 => write!(f, "i64"),
            PrimitiveType::U8 => write!(f, "u8"),
            PrimitiveType::U16 => write!(f, "u16"),
            PrimitiveType::U32 => write!(f, "u32"),
            PrimitiveType::U64 => write!(f, "u64"),
            PrimitiveType::F32 => write!(f, "f32"),
            PrimitiveType::F64 => write!(f, "f64"),
            PrimitiveType::Bool => write!(f, "bool"),
            PrimitiveType::Char => write!(f, "char"),
            PrimitiveType::Str => write!(f, "str"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Types {
    /// The next ID to assign to a type.
    next_id: TypeId,
    /// A map of known type IDs to their corresponding types.
    known: HashMap<TypeId, Rc<Type>>,
    /// A map of types to their corresponding IDs.
    type_ids: HashMap<Rc<Type>, TypeId>,
    /// The current scope of types. This is used to resolve types by path as they are
    /// written in the source code (e.g. `u8` or `::u8`
    scope: TypeScope,
}

impl Types {
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_id: TypeId(0),
            known: HashMap::new(),
            type_ids: HashMap::new(),
            scope: TypeScope::new(),
        }
    }

    pub fn push_scope(&mut self) -> TypeScopeId {
        self.scope.push_block()
    }

    pub fn pop_scope(&mut self) {
        self.scope.pop_block();
    }

    fn next_id(&mut self) -> TypeId {
        let id = self.next_id;
        self.next_id = TypeId(id.0 + 1);
        id
    }

    pub fn iter(&self) -> impl Iterator<Item = (TypeId, &Type)> + '_ {
        self.known.iter().map(|(&id, ty)| (id, ty.as_ref()))
    }

    /// Get the ID of a type, inserting it if it does not already exist.
    ///
    /// Used to ensure deduplication of types.
    pub fn get_or_insert(&mut self, ty: Type) -> TypeId {
        if let Some(id) = self.type_ids.get(&ty) {
            return *id;
        }

        let id = self.next_id();

        let ty = Rc::new(ty);

        self.known.insert(id, ty.clone());
        self.type_ids.insert(ty, id);

        id
    }

    #[must_use]
    pub fn get(&self, path: &Path) -> Option<TypeId> {
        self.scope.get(path)
    }

    #[must_use]
    pub fn get_by_id(&self, id: TypeId) -> Option<Rc<Type>> {
        self.known.get(&id).cloned()
    }

    #[must_use]
    pub fn to_display(&self, id: TypeId) -> String {
        match self.get_by_id(id) {
            None => String::new(),
            Some(ty) => match *ty {
                Type::Unresolved(id) => format!("?{id}"),
                Type::Primitive(primitive) => primitive.to_string(),
                Type::Ptr(inner) => format!("*{}", self.to_display(inner)),
                Type::SizedArray(inner, len) => format!("[{}; {}]", self.to_display(inner), len),
                Type::UnsizedArray(inner) => format!("[{}]", self.to_display(inner)),
            },
        }
    }

    pub fn create_type(&mut self, path: Path, ty: Type) -> TypeId {
        let id = self.get_or_insert(ty);
        self.scope.insert(path, id);
        id
    }

    /// Inserts a new path reference to an existing type.
    ///
    /// # Panics
    ///
    /// Panics if the type ID does not exist (`debug_assert`).
    pub fn create_type_with_id(&mut self, path: Path, id: TypeId) {
        debug_assert!(self.known.contains_key(&id));
        self.scope.insert(path, id);
    }
}

impl Default for Types {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for Types {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut known: Vec<_> = self.known.iter().collect();
        known.sort_by_key(|(id, _)| **id);
        let mut type_ids: Vec<_> = self.type_ids.iter().collect();
        type_ids.sort_by_key(|(_, id)| **id);

        f.debug_struct("Types")
            .field("next_id", &self.next_id)
            .field_with("known", |f| f.debug_map().entries(known).finish())
            .field_with("type_ids", |f| f.debug_map().entries(type_ids).finish())
            .field("scope", &self.scope)
            .finish()
    }
}
