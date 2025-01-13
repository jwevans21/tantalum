use tantalum_ast::{ExternalFunction, Function, Item, NamedParameter, Parameter};
use tantalum_lexer::{token::Token, token_kind::TokenKind};
use tantalum_span::Spanned;

use crate::{error::ParseError, Parser};

impl<'file_name, 'source> Parser<'file_name, 'source> {
    pub const ITEM_START: &'static [TokenKind] = &[TokenKind::KeywordFn, TokenKind::KeywordExtern];

    const EXTERN_START: &'static [TokenKind] = &[TokenKind::KeywordFn];

    pub(crate) fn parse_item(
        &mut self,
    ) -> Result<Spanned<'file_name, Item<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let token = self.expect_any(Self::ITEM_START)?;

        match token.data().kind() {
            TokenKind::KeywordFn => {
                let function = self.parse_top_level_function(token)?;
                Ok(function.map(Item::Function))
            }
            TokenKind::KeywordExtern => {
                let extern_function = self.parse_top_level_extern(token)?;
                Ok(extern_function.map(Item::ExternalFunction))
            }
            _ => unimplemented!(
                "Token {:?} is not in the set {:?}",
                token.data().kind(),
                Self::ITEM_START
            ),
        }
    }

    fn parse_top_level_function(
        &mut self,
        fn_token: Spanned<'file_name, Token<'source>>,
    ) -> Result<Spanned<'file_name, Function<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let name = self.expect(TokenKind::Identifier)?;

        let mut parameters = Vec::new();
        let l_paren = self.expect(TokenKind::LeftParen)?;
        while self.is_at(TokenKind::RightParen).is_none() {
            let parameter_name = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::Colon)?;
            let parameter_type = self.parse_type()?;

            parameters.push(Spanned::join_spans(
                parameter_name.span(),
                parameter_type.span(),
                Parameter::Named(NamedParameter {
                    name: parameter_name.map(|name| name.lexeme()),
                    ty: parameter_type,
                }),
            ));

            match self.nth(0) {
                Some(token) if token.data().kind() == TokenKind::Comma => {
                    self.expect(TokenKind::Comma)?;
                }
                Some(token) if token.data().kind() == TokenKind::RightParen => break,
                Some(token) => {
                    return Err(ParseError::unexpected_token(
                        self.source,
                        token.start(),
                        token.data().kind(),
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

        Ok(Spanned::join_spans(
            fn_token.span(),
            body.span(),
            Function {
                name: name.map(|name| name.lexeme()),
                parameters: Spanned::join_spans(l_paren.span(), r_paren.span(), parameters),
                return_type,
                body,
            },
        ))
    }

    fn parse_top_level_extern(
        &mut self,
        extern_token: Spanned<'file_name, Token<'source>>,
    ) -> Result<
        Spanned<'file_name, ExternalFunction<'file_name, 'source>>,
        ParseError<'file_name, 'source>,
    > {
        match self.is_at_any(Self::EXTERN_START) {
            None => {
                return Err(ParseError::unexpected_token(
                    self.source,
                    extern_token.start(),
                    extern_token.data().kind(),
                    TokenKind::KeywordFn,
                ));
            }
            Some(token) => match token.data().kind() {
                TokenKind::KeywordFn => self.parse_top_level_extern_function(extern_token),
                _ => unimplemented!(
                    "Token {:?} is not in the set {:?}",
                    token.data().kind(),
                    Self::EXTERN_START
                ),
            },
        }
    }

    fn parse_top_level_extern_function(
        &mut self,
        extern_token: Spanned<'file_name, Token<'source>>,
    ) -> Result<
        Spanned<'file_name, ExternalFunction<'file_name, 'source>>,
        ParseError<'file_name, 'source>,
    > {
        self.expect(TokenKind::KeywordFn)?;

        let name = self.expect(TokenKind::Identifier)?;

        let mut parameters = Vec::new();

        let l_paren = self.expect(TokenKind::LeftParen)?;

        while self.is_at(TokenKind::RightParen).is_none() {
            if self.is_at(TokenKind::DotDotDot).is_some() {
                let variadic = self.expect(TokenKind::DotDotDot)?;
                parameters.push(variadic.map(|_| Parameter::Variadic));

                break;
            }

            let parameter_name = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::Colon)?;
            let parameter_type = self.parse_type()?;

            parameters.push(Spanned::join_spans(
                parameter_name.span(),
                parameter_type.span(),
                Parameter::Named(NamedParameter {
                    name: parameter_name.map(|name| name.lexeme()),
                    ty: parameter_type,
                }),
            ));

            match self.nth(0) {
                Some(token) if token.data().kind() == TokenKind::Comma => {
                    self.expect(TokenKind::Comma)?;
                }
                Some(token) if token.data().kind() == TokenKind::RightParen => break,
                Some(token) => {
                    return Err(ParseError::unexpected_token(
                        self.source,
                        token.start(),
                        token.data().kind(),
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

        Ok(Spanned::join_spans(
            extern_token.span(),
            semicolon.span(),
            ExternalFunction {
                name: name.map(|name| name.lexeme()),
                parameters: Spanned::join_spans(l_paren.span(), r_paren.span(), parameters),
                return_type,
            },
        ))
    }
}
