use error::ParseError;
use tantalum_ast::TopLevelExpression;
use tantalum_lexer::{token::Token, token_kind::TokenKind, Lexer};
use tantalum_span::Span;

pub mod error;

mod expressions;
mod statements;
mod types;
mod top_level;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Parser<'file_name, 'source> {
    // lexer: Lexer<'file_name, 'source>,
    source: &'source str,
    file_name: &'file_name str,
    tokens: Vec<Token<'file_name, 'source>>,
    eof: Span<'file_name>,
    position: usize,
}

impl<'file_name, 'source> Parser<'file_name, 'source> {
    #[must_use]
    #[inline]
    pub fn new(mut lexer: Lexer<'file_name, 'source>) -> Self {
        Self {
            source: lexer.source(),
            file_name: lexer.file_name(),
            tokens: lexer.by_ref().collect(),
            eof: lexer.span(),
            position: 0,
        }
    }

    /// Parse the entire source file.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the parser encounters an unexpected token or the end of the file.
    pub fn parse(
        &mut self,
    ) -> Result<Vec<TopLevelExpression<'file_name, 'source>>, error::ParseError<'file_name, 'source>>
    {
        let mut top_levels = Vec::new();

        while !self.is_eof() {
            top_levels.push(self.parse_top_level()?);
        }

        Ok(top_levels)
    }

    fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn is_at(&self, kind: TokenKind) -> Option<Token<'file_name, 'source>> {
        self.tokens
            .get(self.position)
            .filter(|token| token.kind() == kind)
            .copied()
    }

    fn advance_if(&mut self, kind: TokenKind) -> Option<Token<'file_name, 'source>> {
        if let Some(token) = self.is_at(kind) {
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(
        &mut self,
        kind: TokenKind,
    ) -> Result<Token<'file_name, 'source>, error::ParseError<'file_name, 'source>> {
        let Some(token) = self.tokens.get(self.position) else {
            return Err(error::ParseError::unexpected_eof(self.source, self.eof));
        };

        if token.kind() == kind {
            self.position += 1;
            Ok(*token)
        } else {
            Err(error::ParseError::unexpected_token_set(
                self.source,
                token.span(),
                token.kind(),
                &[kind],
            ))
        }
    }

    #[allow(unused)]
    fn next_is(&self, kind: TokenKind) -> Option<Token<'file_name, 'source>> {
        self.tokens
            .get(self.position + 1)
            .filter(|token| token.kind() == kind)
            .copied()
    }

    fn is_at_any<'a>(&self, set: &'a [TokenKind]) -> Option<Token<'file_name, 'source>> {
        self.tokens
            .get(self.position)
            .filter(|token| set.contains(&token.kind()))
            .copied()
    }

    fn advance_if_any<'a>(&mut self, set: &'a [TokenKind]) -> Option<Token<'file_name, 'source>> {
        if let Some(token) = self.is_at_any(set) {
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect_any<'a>(
        &mut self,
        set: &'a [TokenKind],
    ) -> Result<Token<'file_name, 'source>, error::ParseError<'file_name, 'source>> {
        let Some(token) = self.tokens.get(self.position) else {
            return Err(error::ParseError::unexpected_eof(self.source, self.eof));
        };

        if set.contains(&token.kind()) {
            self.position += 1;
            Ok(*token)
        } else {
            Err(error::ParseError::unexpected_token_set(
                self.source,
                token.span(),
                token.kind(),
                set,
            ))
        }
    }

    #[allow(unused)]
    fn next_is_any<'a>(&self, set: &'a [TokenKind]) -> Option<Token<'file_name, 'source>> {
        self.tokens
            .get(self.position + 1)
            .filter(|token| set.contains(&token.kind()))
            .copied()
    }

    fn nth(&self, n: usize) -> Option<Token<'file_name, 'source>> {
        self.tokens.get(self.position + n).copied()
    }

    fn put_back(&mut self, n: usize) {
        self.position -= n;
    }
}
