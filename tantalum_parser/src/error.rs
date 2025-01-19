use tantalum_lexer::token_kind::TokenKind;
use tantalum_span::Location;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ParseError<'file_name, 'source> {
    pub source: &'source str,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub location: Location<'file_name>,
    pub kind: ParseErrorKind,
}

impl<'file_name, 'source> ParseError<'file_name, 'source> {
    pub(crate) fn unexpected_eof(
        source: &'source str,
        location: Location<'file_name>,
    ) -> ParseError<'file_name, 'source> {
        ParseError {
            source,
            location,
            kind: ParseErrorKind::UnexpectedEof,
        }
    }

    pub(crate) fn unexpected_token(
        source: &'source str,
        location: Location<'file_name>,
        kind: TokenKind,
        token: TokenKind,
    ) -> ParseError<'file_name, 'source> {
        return Self {
            source,
            location,
            kind: ParseErrorKind::UnexpectedToken {
                kind,
                set: Box::from([token]),
            },
        };
    }

    pub(crate) fn unexpected_token_set(
        source: &'source str,
        location: Location<'file_name>,
        kind: TokenKind,
        set: &[TokenKind],
    ) -> ParseError<'file_name, 'source> {
        return Self {
            source,
            location,
            kind: ParseErrorKind::UnexpectedToken {
                kind,
                set: Box::from(set),
            },
        };
    }
}

impl core::fmt::Display for ParseError<'_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let lines = self.source.lines().collect::<Vec<_>>();

        writeln!(f, "error: {}", self.kind)?;
        writeln!(f, " --> {}:{}", self.location.file_name(), self.location.line())?;
        writeln!(f)?;

        for (i, line) in lines
            .iter()
            .enumerate()
            .skip(self.location.line() - 2)
            .take(3)
        {
            writeln!(f, "{:>4} | {}", i + 1, line)?;
            if i + 1 == self.location.line() {
                writeln!(f, "{:>4} | {:>1$}^", "", self.location.column())?;
            }
        }

        Ok(())
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

impl core::fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseErrorKind::UnexpectedEof => write!(f, "unexpected end of file"),
            ParseErrorKind::UnexpectedToken { kind, set } => {
                write!(f, "unexpected token {kind:?}, expected one of {set:?}")
            }
        }
    }
}
