use tantalum_span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal<'file_name, 'source> {
    Integer(#[cfg_attr(feature = "serde", serde(borrow))] Integer<'file_name, 'source>),
    Float(Float<'file_name, 'source>),
    Boolean(Boolean<'file_name, 'source>),
    Character(Character<'file_name, 'source>),
    String(String<'file_name, 'source>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Integer<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Spanned<'file_name, &'source str>,
    pub radix: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Float<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Spanned<'file_name, &'source str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Boolean<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Spanned<'file_name, &'source str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Character<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Spanned<'file_name, &'source str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct String<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Spanned<'file_name, &'source str>,
}
