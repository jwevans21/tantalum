use tantalum_span::Spanned;

use crate::{Statement, Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item<'file_name, 'source> {
    Function(#[cfg_attr(feature = "serde", serde(borrow))] Function<'file_name, 'source>),
    ExternalFunction(
        #[cfg_attr(feature = "serde", serde(borrow))] ExternalFunction<'file_name, 'source>,
    ),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Spanned<'file_name, &'source str>,
    pub parameters: Spanned<'file_name, Vec<Spanned<'file_name, Parameter<'file_name, 'source>>>>,
    pub return_type: Option<Spanned<'file_name, Type<'file_name, 'source>>>,
    pub body: Spanned<'file_name, Statement<'file_name, 'source>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExternalFunction<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Spanned<'file_name, &'source str>,
    pub parameters: Spanned<'file_name, Vec<Spanned<'file_name, Parameter<'file_name, 'source>>>>,
    pub return_type: Option<Spanned<'file_name, Type<'file_name, 'source>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Parameter<'file_name, 'source> {
    Named(#[cfg_attr(feature = "serde", serde(borrow))] NamedParameter<'file_name, 'source>),
    Variadic,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamedParameter<'file_name, 'source> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Spanned<'file_name, &'source str>,
    pub ty: Spanned<'file_name, Type<'file_name, 'source>>,
}
