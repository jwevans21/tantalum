use crate::inference::InferenceId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    value: LiteralValue,
    ty: InferenceId,
}

impl Literal {
    pub fn new(value: LiteralValue, ty: InferenceId) -> Self {
        Self { value, ty }
    }
    pub fn ty(&self) -> InferenceId {
        self.ty
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralValue {
    Integer { value: String, radix: u32 },
    Float { value: String },
    Boolean { value: bool },
    Character { value: String },
    String { value: String },
}
