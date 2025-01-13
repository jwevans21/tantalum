use tantalum_span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type<'file_name, 'source> {
    Named(#[cfg_attr(feature = "serde", serde(borrow))] NamedType<'file_name, 'source>),
    Function(FunctionType<'file_name, 'source>),
    Pointer(PointerType<'file_name, 'source>),
    SizedArray(SizedArrayType<'file_name, 'source>),
    UnsizedArray(UnsizedArrayType<'file_name, 'source>),

    Const(ConstType<'file_name, 'source>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamedType<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Spanned<'file_name, &'source str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionType<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parameters: Vec<Spanned<'file_name, Type<'file_name, 'source>>>,
    pub return_type: Box<Spanned<'file_name, Type<'file_name, 'source>>>,
    pub is_variadic: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerType<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ty: Box<Spanned<'file_name, Type<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SizedArrayType<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ty: Box<Spanned<'file_name, Type<'file_name, 'source>>>,
    pub size: Spanned<'file_name, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnsizedArrayType<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ty: Box<Spanned<'file_name, Type<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstType<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ty: Box<Spanned<'file_name, Type<'file_name, 'source>>>,
}
