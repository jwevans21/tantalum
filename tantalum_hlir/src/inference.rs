use crate::types::{PrimitiveType, Type, Types};
use crate::{traits::TraitId, types::TypeId, Path, PathSegment};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct InferenceId(usize);

impl core::fmt::Debug for InferenceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "InferenceId({})", self.0)
    }
}

impl core::fmt::Display for InferenceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "?{}", self.0)
    }
}

/// A constraint on an unknown type variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeConstraint {
    /// The type is equal to the given type.
    Type(TypeId),
    ConvertedFrom(InferenceId),
    ConvertibleTo(InferenceId),
    /// The type implements the given trait.
    Implements(TraitId),

    DerefTo(InferenceId),
    RefTo(InferenceId),

    FromIntegerLiteral,
    FromFloatLiteral,
}

impl TypeConstraint {
    pub fn to_display(&self, types: &Types) -> String {
        match self {
            TypeConstraint::Type(ty) => format!("{}", types.to_display(*ty)),
            TypeConstraint::ConvertedFrom(id) => format!("ConvertedFrom({})", id),
            TypeConstraint::ConvertibleTo(id) => format!("ConvertibleTo({})", id),
            TypeConstraint::Implements(trait_id) => format!("Implements({:?})", trait_id),
            TypeConstraint::DerefTo(id) => format!("DerefTo({})", id),
            TypeConstraint::RefTo(id) => format!("RefTo({})", id),
            TypeConstraint::FromIntegerLiteral => "FromIntegerLiteral".to_string(),
            TypeConstraint::FromFloatLiteral => "FromFloatLiteral".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct TypeInferenceEnvironment {
    next_id: InferenceId,
    resolved: HashMap<InferenceId, TypeId>,
    constraints: HashMap<InferenceId, Vec<TypeConstraint>>,
}

impl TypeInferenceEnvironment {
    pub fn new() -> Self {
        Self {
            next_id: InferenceId(0),
            resolved: HashMap::new(),
            constraints: HashMap::new(),
        }
    }

    pub fn to_display(&self, id: InferenceId, types: &Types) -> String {
        match self.resolve(id) {
            Some(ty) => types.to_display(ty),
            None => format!("{id}"),
        }
    }

    fn next_id(&mut self) -> InferenceId {
        let id = self.next_id;
        self.next_id = InferenceId(id.0 + 1);
        id
    }

    pub fn create_unknown(&mut self) -> InferenceId {
        let id = self.next_id();
        self.constraints.insert(id, Vec::new());
        id
    }

    pub fn create_resolved(&mut self, ty: TypeId) -> InferenceId {
        let id = self.next_id();
        self.resolved.insert(id, ty);
        id
    }

    pub fn add_constraint(&mut self, id: InferenceId, constraint: TypeConstraint) {
        let entry = self.constraints.get_mut(&id).expect(
            "TypeInferenceId not found in constraints, either it was resolved or not created",
        );

        entry.push(constraint);
    }

    pub fn unify(&mut self, a: InferenceId, b: InferenceId, types: &Types) {
        match (self.resolve(a), self.resolve(b)) {
            (Some(a), Some(b)) => {
                assert_eq!(
                    a,
                    b,
                    "Cannot unify types {} and {}",
                    types.to_display(a),
                    types.to_display(b)
                );
            }
            (Some(type_id), None) => {
                self.resolved.insert(b, type_id);
                self.check_constraints(b, type_id, types);
            }
            (None, Some(type_id)) => {
                self.resolved.insert(a, type_id);
                self.check_constraints(a, type_id, types);
            }
            (None, None) => {
                self.add_constraint(a, TypeConstraint::ConvertedFrom(b));
                self.add_constraint(b, TypeConstraint::ConvertibleTo(a));
            }
        }
    }

    pub fn unify_with(&mut self, a: InferenceId, ty: TypeId, types: &Types) {
        if let Some(resolved) = self.resolve(a) {
            dbg!(&self, &types);
            assert_eq!(
                resolved,
                ty,
                "Cannot unify type {} with type {}",
                types.to_display(resolved),
                types.to_display(ty)
            );
        } else {
            self.resolved.insert(a, ty);
            self.check_constraints(a, ty, types);
        }
    }

    pub fn unify_final(&mut self, types: &Types) {
        let constraints = self.constraints.clone();

        for (id, constraints) in constraints {
            for constraint in constraints {
                match constraint {
                    TypeConstraint::Type(ty) => {
                        self.resolved.insert(id, ty);
                    }
                    TypeConstraint::ConvertedFrom(other) => {
                        self.unify(id, other, types);
                    }
                    TypeConstraint::ConvertibleTo(other) => {
                        self.unify(other, id, types);
                    }
                    TypeConstraint::FromIntegerLiteral => {
                        self.unify_with(
                            id,
                            types
                                .get(&Path::new(vec![PathSegment::new("i32".to_string())]))
                                .unwrap(),
                            types,
                        );
                    }
                    TypeConstraint::FromFloatLiteral => {
                        self.unify_with(
                            id,
                            types
                                .get(&Path::new(vec![PathSegment::new("f32".to_string())]))
                                .unwrap(),
                            types,
                        );
                    }
                    TypeConstraint::RefTo(other) => {
                        self.unify(other, id, types);
                    }
                    _ => {
                        dbg!(&self);
                        dbg!(&constraint);
                        panic!(
                            "Cannot resolve type inference id {} with constraint {}\n  --> {}",
                            self.to_display(id, types),
                            constraint.to_display(types),
                            self.constraints
                                .get(&id)
                                .iter()
                                .map(|c| c
                                    .iter()
                                    .map(|c| c.to_display(types))
                                    .collect::<Vec<_>>()
                                    .join(", "))
                                .collect::<Vec<_>>()
                                .join("\n")
                        );
                    }
                }
            }
        }
    }

    fn check_constraints(&mut self, id: InferenceId, ty: TypeId, types: &Types) {
        let constraints = self.constraints.remove(&id).unwrap();
        for constraint in constraints {
            match constraint {
                TypeConstraint::Type(expected) => {
                    assert_eq!(
                        ty,
                        expected,
                        "Type {} does not match expected type {}",
                        types.to_display(ty),
                        types.to_display(expected)
                    );
                }
                TypeConstraint::ConvertedFrom(other) => {
                    self.unify(id, other, types);
                }
                TypeConstraint::ConvertibleTo(other) => {
                    self.unify(other, id, types);
                }
                TypeConstraint::Implements(trait_id) => {
                    todo!()
                }
                TypeConstraint::DerefTo(other) => {
                    todo!()
                }
                TypeConstraint::RefTo(other) => match self.resolve(other) {
                    Some(other) => match self.resolve(id) {
                        Some(ty) => {
                            match (
                                types
                                    .get_by_id(ty)
                                    .expect("Type not found in types")
                                    .as_ref(),
                                types
                                    .get_by_id(other)
                                    .expect("Type not found in types")
                                    .as_ref(),
                            ) {
                                // Handle referencing a str type
                                (Type::Primitive(PrimitiveType::Str), Type::Ptr(id))
                                    if types.get_by_id(*id).is_some_and(|ty| {
                                        matches!(ty.as_ref(), Type::Primitive(PrimitiveType::U8))
                                    }) => {}
                                _ => {
                                    panic!(
                                        "Cannot reference type {} to type {}",
                                        types.to_display(ty),
                                        types.to_display(other)
                                    )
                                }
                            }
                        }
                        None => {
                            self.add_constraint(id, TypeConstraint::Type(other));
                        }
                    },
                    None => {
                        self.add_constraint(other, TypeConstraint::RefTo(id));
                    }
                },
                TypeConstraint::FromIntegerLiteral => match types.get_by_id(ty) {
                    None => panic!("Type {ty:?} not found in types"),
                    Some(ty) => match *ty {
                        Type::Primitive(primitive) => match primitive {
                            PrimitiveType::I8
                            | PrimitiveType::I16
                            | PrimitiveType::I32
                            | PrimitiveType::I64
                            | PrimitiveType::U8
                            | PrimitiveType::U16
                            | PrimitiveType::U32
                            | PrimitiveType::U64
                            | PrimitiveType::F32
                            | PrimitiveType::F64 => {}
                            PrimitiveType::Bool
                            | PrimitiveType::Char
                            | PrimitiveType::Str
                            | PrimitiveType::Void => {
                                panic!("Type {ty:?} cannot be converted to from an integer literal")
                            }
                        },
                        _ => panic!("Type {ty:?} is not a primitive type"),
                    },
                },
                TypeConstraint::FromFloatLiteral => {
                    // TODO: Check if the type is a float type
                    match types.get_by_id(ty) {
                        None => panic!("Type {ty:?} not found in types"),
                        Some(resolved_type) => match *resolved_type {
                            Type::Primitive(primitive) => match primitive {
                                PrimitiveType::F32 | PrimitiveType::F64 => {}
                                PrimitiveType::I8
                                | PrimitiveType::I16
                                | PrimitiveType::I32
                                | PrimitiveType::I64
                                | PrimitiveType::U8
                                | PrimitiveType::U16
                                | PrimitiveType::U32
                                | PrimitiveType::U64
                                | PrimitiveType::Bool
                                | PrimitiveType::Char
                                | PrimitiveType::Str
                                | PrimitiveType::Void => {
                                    panic!(
                                        "Type {} cannot be converted to from an float literal",
                                        types.to_display(ty)
                                    )
                                }
                            },
                            _ => panic!("Type {resolved_type:?} is not a primitive type"),
                        },
                    }
                }
            }
        }
    }

    pub fn is_resolved(&self, id: InferenceId) -> bool {
        self.resolved.contains_key(&id)
    }

    pub fn resolve(&self, id: InferenceId) -> Option<TypeId> {
        self.resolved.get(&id).copied()
    }
}

impl core::fmt::Debug for TypeInferenceEnvironment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TypeInferenceEnvironment")
            .field("next_id", &self.next_id)
            .field_with("resolved", |f| {
                let mut resolved: Vec<_> = self.resolved.iter().collect();
                resolved.sort_by_key(|(id, _)| **id);
                f.debug_map().entries(resolved).finish()
            })
            .field_with("constraints", |f| {
                let mut constraints: Vec<_> = self.constraints.iter().collect();
                constraints.sort_by_key(|(id, _)| **id);
                f.debug_map().entries(constraints).finish()
            })
            .finish()
    }
}
