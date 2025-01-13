use error::ParseError;
use tantalum_ast::AST;
use tantalum_lexer::{token::Token, token_kind::TokenKind, Lexer};
use tantalum_span::{Location, Spanned};

pub mod error;

mod expressions;
mod items;
mod literals;
mod statements;
mod types;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Parser<'file_name, 'source> {
    // lexer: Lexer<'file_name, 'source>,
    source: &'source str,
    file_name: &'file_name str,
    tokens: Vec<Spanned<'file_name, Token<'source>>>,
    eof: Location<'file_name>,
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
            eof: lexer.location(),
            position: 0,
        }
    }

    /// Parse the entire source file.
    ///
    /// # Errors
    ///
    /// Returns an error if the parser encounters an unexpected token or the end of the file.
    pub fn parse(&mut self) -> Result<AST, error::ParseError<'file_name, 'source>> {
        let mut items = Vec::new();

        while !self.is_eof() {
            items.push(self.parse_item()?);
        }

        Ok(AST(items))
    }

    fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn is_at(&self, kind: TokenKind) -> Option<Spanned<'file_name, Token<'source>>> {
        self.tokens
            .get(self.position)
            .filter(|token| token.data().kind() == kind)
            .copied()
    }

    fn advance_if(&mut self, kind: TokenKind) -> Option<Spanned<'file_name, Token<'source>>> {
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
    ) -> Result<Spanned<'file_name, Token<'source>>, error::ParseError<'file_name, 'source>> {
        let Some(token) = self.tokens.get(self.position) else {
            return Err(error::ParseError::unexpected_eof(self.source, self.eof));
        };

        if token.data().kind() == kind {
            self.position += 1;
            Ok(*token)
        } else {
            Err(error::ParseError::unexpected_token_set(
                self.source,
                token.span().start(),
                token.data().kind(),
                &[kind],
            ))
        }
    }

    fn peek(&self) -> Option<Spanned<'file_name, Token<'source>>> {
        self.tokens.get(self.position).copied()
    }

    fn next(&mut self) -> Option<Spanned<'file_name, Token<'source>>> {
        let token = self.tokens.get(self.position).copied();
        self.position += 1;
        token
    }

    fn is_at_any<'a>(&self, set: &'a [TokenKind]) -> Option<Spanned<'file_name, Token<'source>>> {
        self.tokens
            .get(self.position)
            .filter(|token| set.contains(&token.data().kind()))
            .copied()
    }

    fn expect_any<'a>(
        &mut self,
        set: &'a [TokenKind],
    ) -> Result<Spanned<'file_name, Token<'source>>, error::ParseError<'file_name, 'source>> {
        let Some(token) = self.tokens.get(self.position) else {
            return Err(error::ParseError::unexpected_eof(self.source, self.eof));
        };

        if set.contains(&token.data().kind()) {
            self.position += 1;
            Ok(*token)
        } else {
            Err(error::ParseError::unexpected_token_set(
                self.source,
                token.span().start(),
                token.data().kind(),
                set,
            ))
        }
    }

    fn nth(&self, n: usize) -> Option<Spanned<'file_name, Token<'source>>> {
        self.tokens.get(self.position + n).copied()
    }
}
