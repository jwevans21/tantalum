use tantalum_ast::{
    Parameter, ParameterKind, TopLevelExpression, TopLevelExpressionKind, Type, TypeKind,
};
use tantalum_lexer::{token::Token, token_kind::TokenKind};
use tantalum_span::{Span, Spanned};

use crate::{error::ParseError, Parser};

impl<'file_name, 'source> Parser<'file_name, 'source> {
    pub const TOP_LEVEL_START: &'static [TokenKind] =
        &[TokenKind::KeywordFn, TokenKind::KeywordExtern];

    const EXTERN_START: &'static [TokenKind] = &[TokenKind::KeywordFn];

    pub(crate) fn parse_top_level(
        &mut self,
    ) -> Result<TopLevelExpression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let token = self.expect_any(Self::TOP_LEVEL_START)?;

        match token.kind() {
            TokenKind::KeywordFn => self.parse_top_level_function(token),
            TokenKind::KeywordExtern => self.parse_top_level_extern(token),
            _ => unimplemented!(
                "Token {:?} is not in the set {:?}",
                token.kind(),
                Self::TOP_LEVEL_START
            ),
        }
    }

    fn parse_top_level_function(
        &mut self,
        fn_token: Spanned<'file_name, Token<'source>>,
    ) -> Result<TopLevelExpression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let name = self.expect(TokenKind::Identifier)?;

        let mut parameters = Vec::new();
        self.expect(TokenKind::LeftParen)?;
        while self.is_at(TokenKind::RightParen).is_none() {
            let parameter_name = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::Colon)?;
            let parameter_type = self.parse_type()?;

            parameters.push(Parameter {
                span: Span::new(parameter_name.start(), parameter_type.span.end()),
                kind: ParameterKind::Named {
                    name: parameter_name.lexeme(),
                    ty: parameter_type,
                },
            });

            match self.nth(0) {
                Some(token) if token.kind() == TokenKind::Comma => {
                    self.expect(TokenKind::Comma)?;
                }
                Some(token) if token.kind() == TokenKind::RightParen => break,
                Some(token) => {
                    return Err(ParseError::unexpected_token(
                        self.source,
                        token.start(),
                        token.kind(),
                        TokenKind::Comma,
                    ));
                }
                None => {
                    return Err(ParseError::unexpected_eof(self.source, self.eof));
                }
            }
        }
        let r_paren = self.expect(TokenKind::RightParen)?;

        let return_type = if self.is_at(TokenKind::Colon).is_some() {
            self.expect(TokenKind::Colon)?;
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_statement()?;

        Ok(TopLevelExpression {
            span: Span::new(fn_token.start(), body.span.end()),
            kind: TopLevelExpressionKind::FunctionDeclaration {
                name: name.lexeme(),
                parameters,
                return_type: return_type.map_or_else(
                    || Type {
                        span: r_paren.span(),
                        kind: TypeKind::Named("void"),
                    },
                    |ty| ty,
                ),
                body,
            },
        })
    }

    fn parse_top_level_extern(
        &mut self,
        extern_token: Spanned<'file_name, Token<'source>>,
    ) -> Result<TopLevelExpression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        match self.is_at_any(Self::EXTERN_START) {
            None => {
                return Err(ParseError::unexpected_token(
                    self.source,
                    extern_token.start(),
                    extern_token.kind(),
                    TokenKind::KeywordFn,
                ));
            }
            Some(token) => match token.kind() {
                TokenKind::KeywordFn => self.parse_top_level_extern_function(extern_token),
                _ => unimplemented!(
                    "Token {:?} is not in the set {:?}",
                    token.kind(),
                    Self::EXTERN_START
                ),
            },
        }
    }

    fn parse_top_level_extern_function(
        &mut self,
        extern_token: Spanned<'file_name, Token<'source>>,
    ) -> Result<TopLevelExpression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        self.expect(TokenKind::KeywordFn)?;

        let name = self.expect(TokenKind::Identifier)?;

        let mut parameters = Vec::new();
        let mut is_variadic = false;

        self.expect(TokenKind::LeftParen)?;

        while self.is_at(TokenKind::RightParen).is_none() {
            if self.is_at(TokenKind::DotDotDot).is_some() {
                self.expect(TokenKind::DotDotDot)?;
                is_variadic = true;
                break;
            }

            let parameter_name = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::Colon)?;
            let parameter_type = self.parse_type()?;

            parameters.push(Parameter {
                span: Span::new(parameter_name.start(), parameter_type.span.end()),
                kind: ParameterKind::Named {
                    name: parameter_name.lexeme(),
                    ty: parameter_type,
                },
            });

            match self.nth(0) {
                Some(token) if token.kind() == TokenKind::Comma => {
                    self.expect(TokenKind::Comma)?;
                }
                Some(token) if token.kind() == TokenKind::RightParen => break,
                Some(token) => {
                    return Err(ParseError::unexpected_token(
                        self.source,
                        token.start(),
                        token.kind(),
                        TokenKind::Comma,
                    ));
                }
                None => {
                    return Err(ParseError::unexpected_eof(self.source, self.eof));
                }
            }
        }

        let r_paren = self.expect(TokenKind::RightParen)?;

        let return_type = if self.is_at(TokenKind::Colon).is_some() {
            self.expect(TokenKind::Colon)?;
            Some(self.parse_type()?)
        } else {
            None
        };

        let semicolon = self.expect(TokenKind::Semicolon)?;

        Ok(TopLevelExpression {
            span: Span::new(extern_token.start(), semicolon.span().end()),
            kind: TopLevelExpressionKind::ExternalFunction {
                name: name.lexeme(),
                parameters,
                return_type: return_type.map_or_else(
                    || Type {
                        span: r_paren.span(),
                        kind: TypeKind::Named("void"),
                    },
                    |ty| ty,
                ),
                is_variadic,
            },
        })
    }
}
