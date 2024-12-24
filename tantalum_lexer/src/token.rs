use core::ops::Range;

use tantalum_span::Span;

use crate::token_kind::TokenKind;

/// A token that was found in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Token<'file_name, 'source> {
    /// The source code range that this token covers
    #[cfg_attr(feature = "serde", serde(borrow))]
    span: Span<'file_name>,
    /// The portion of the source code that this token represents
    lexeme: &'source str,
    /// The kind of token that was found
    kind: TokenKind,
}

impl<'file_name, 'source> Token<'file_name, 'source> {
    #[must_use]
    #[inline]
    pub fn new(span: Span<'file_name>, lexeme: &'source str, kind: TokenKind) -> Self {
        return Self { span, lexeme, kind };
    }

    #[must_use]
    #[inline]
    pub fn file_name(&self) -> &'file_name str {
        return self.span.file_name();
    }

    #[must_use]
    #[inline]
    pub fn lexeme(&self) -> &'source str {
        return self.lexeme;
    }

    #[must_use]
    #[inline]
    pub fn kind(&self) -> TokenKind {
        return self.kind;
    }

    #[must_use]
    #[inline]
    pub fn span(&self) -> Span<'file_name> {
        return self.span;
    }

    #[must_use] pub fn range(&self) -> Range<usize> {
        return self.span.range();
    }

    #[must_use]
    #[inline]
    pub fn lines(&self) -> usize {
        return self.span.line();
    }

    #[must_use]
    #[inline]
    pub fn columns(&self) -> usize {
        return self.span.column();
    }
}
