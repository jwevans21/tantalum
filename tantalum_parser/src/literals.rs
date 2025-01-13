use tantalum_ast::{Boolean, Character, Float, Integer, Literal, String};
use tantalum_lexer::{token::Token, token_kind::TokenKind};
use tantalum_span::Spanned;

use crate::{error::ParseError, Parser};

impl<'file_name, 'source> Parser<'file_name, 'source> {
    pub const LITERAL_START: &'static [TokenKind] = &[
        TokenKind::BinaryIntegerLiteral,
        TokenKind::OctalIntegerLiteral,
        TokenKind::DecimalIntegerLiteral,
        TokenKind::HexadecimalIntegerLiteral,
        TokenKind::FloatLiteral,
        TokenKind::KeywordTrue,
        TokenKind::KeywordFalse,
        TokenKind::CharacterLiteral,
        TokenKind::StringLiteral,
    ];

    pub(crate) fn parse_literal(
        &mut self,
        token: Spanned<'file_name, Token<'source>>,
    ) -> Result<Spanned<'file_name, Literal<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let literal = match token.data().kind() {
            TokenKind::BinaryIntegerLiteral => token
                .map(|_| Integer {
                    value: token.map(|token| token.lexeme()),
                    radix: 2,
                })
                .map(Literal::Integer),
            TokenKind::OctalIntegerLiteral => token
                .map(|_| Integer {
                    value: token.map(|token| token.lexeme()),
                    radix: 8,
                })
                .map(Literal::Integer),
            TokenKind::DecimalIntegerLiteral => token
                .map(|_| Integer {
                    value: token.map(|token| token.lexeme()),
                    radix: 10,
                })
                .map(Literal::Integer),
            TokenKind::HexadecimalIntegerLiteral => token
                .map(|_| Integer {
                    value: token.map(|token| token.lexeme()),
                    radix: 16,
                })
                .map(Literal::Integer),
            TokenKind::FloatLiteral => token
                .map(|_| Float {
                    value: token.map(|token| token.lexeme()),
                })
                .map(Literal::Float),
            TokenKind::KeywordTrue => token
                .map(|_| Boolean {
                    value: token.map(|token| token.lexeme()),
                })
                .map(Literal::Boolean),
            TokenKind::KeywordFalse => token
                .map(|_| Boolean {
                    value: token.map(|token| token.lexeme()),
                })
                .map(Literal::Boolean),
            TokenKind::CharacterLiteral => token
                .map(|_| Character {
                    value: token.map(|token| token.lexeme()),
                })
                .map(Literal::Character),
            TokenKind::StringLiteral => token
                .map(|_| String {
                    value: token.map(|token| token.lexeme()),
                })
                .map(Literal::String),
            _ => {
                return Err(ParseError::unexpected_token_set(
                    self.source,
                    token.start(),
                    token.data().kind(),
                    Self::LITERAL_START,
                ));
            }
        };

        Ok(literal)
    }
}
