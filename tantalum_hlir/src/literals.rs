use crate::inference::InferenceId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub value: LiteralValue,
    pub ty: InferenceId,
}

impl Literal {
    #[must_use]
    pub fn new(value: LiteralValue, ty: InferenceId) -> Self {
        Self { value, ty }
    }

    #[must_use]
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
