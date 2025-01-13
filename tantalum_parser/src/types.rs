use tantalum_ast::{ConstType, NamedType, PointerType, Type, UnsizedArrayType};
use tantalum_lexer::token_kind::TokenKind;
use tantalum_span::Spanned;

use crate::{error::ParseError, Parser};

impl<'file_name, 'source> Parser<'file_name, 'source> {
    pub const TYPE_START_SET: &'static [TokenKind] = &[
        TokenKind::Identifier,
        TokenKind::LeftBracket,
        TokenKind::Star,
        TokenKind::KeywordConst,
    ];

    pub(crate) fn parse_type(
        &mut self,
    ) -> Result<Spanned<'file_name, Type<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let token = self.expect_any(Self::TYPE_START_SET)?;

        match token.data().kind() {
            TokenKind::Identifier => Ok(token.map(|_| {
                Type::Named(NamedType {
                    name: token.map(|name| name.lexeme()),
                })
            })),
            TokenKind::LeftBracket => {
                let element_type = self.parse_type()?;

                let r_bracket = self.expect_any(&[TokenKind::RightBracket])?;

                Ok(Spanned::join_spans(
                    token.span(),
                    r_bracket.span(),
                    Type::UnsizedArray(UnsizedArrayType {
                        ty: Box::new(element_type),
                    }),
                ))
            }
            TokenKind::Star => {
                let element_type = self.parse_type()?;

                Ok(Spanned::join_spans(
                    token.span(),
                    element_type.span(),
                    Type::Pointer(PointerType {
                        ty: Box::new(element_type),
                    }),
                ))
            }
            TokenKind::KeywordConst => {
                let element_type = self.parse_type()?;

                Ok(Spanned::join_spans(
                    token.span(),
                    element_type.span(),
                    Type::Const(ConstType {
                        ty: Box::new(element_type),
                    }),
                ))
            }
            _ => todo!(),
        }
    }
}
