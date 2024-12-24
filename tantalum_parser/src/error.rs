use tantalum_lexer::token_kind::TokenKind;
use tantalum_span::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ParseError<'file_name, 'source> {
    pub source: &'source str,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub location: Span<'file_name>,
    pub kind: ParseErrorKind,
}

impl<'file_name, 'source> ParseError<'file_name, 'source> {
    pub(crate) fn unexpected_eof(
        source: &'source str,
        span: Span<'file_name>,
    ) -> ParseError<'file_name, 'source> {
        ParseError {
            source,
            location: span,
            kind: ParseErrorKind::UnexpectedEof,
        }
    }

    pub(crate) fn unexpected_token(
        source: &'source str,
        span: Span<'file_name>,
        kind: TokenKind,
        token: TokenKind,
    ) -> ParseError<'file_name, 'source> {
        return Self {
            source,
            location: span,
            kind: ParseErrorKind::UnexpectedToken {
                kind,
                set: Box::from([token]),
            },
        };
    }

    pub(crate) fn unexpected_token_set(
        source: &'source str,
        span: Span<'file_name>,
        kind: TokenKind,
        set: &[TokenKind],
    ) -> ParseError<'file_name, 'source> {
        return Self {
            source,
            location: span,
            kind: ParseErrorKind::UnexpectedToken {
                kind,
                set: Box::from(set),
            },
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ParseErrorKind {
    UnexpectedEof,
    UnexpectedToken {
        kind: TokenKind,
        set: Box<[TokenKind]>,
    },
}
