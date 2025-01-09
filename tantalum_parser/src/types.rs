use tantalum_ast::{Type, TypeKind};
use tantalum_lexer::token_kind::TokenKind;
use tantalum_span::Span;

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
    ) -> Result<Type<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let token = self.expect_any(Self::TYPE_START_SET)?;

        match token.kind() {
            TokenKind::Identifier => Ok(Type {
                span: token.span(),
                kind: TypeKind::Named(token.lexeme()),
            }),
            TokenKind::LeftBracket => {
                let element_type = self.parse_type()?;

                let r_bracket = self.expect_any(&[TokenKind::RightBracket])?;

                Ok(Type {
                    span: Span::new(token.span().start(), r_bracket.span().end()),
                    kind: TypeKind::UnsizedArray(Box::new(element_type)),
                })
            }
            TokenKind::Star => {
                let element_type = self.parse_type()?;

                Ok(Type {
                    span: Span::new(token.span().start(), element_type.span.end()),
                    kind: TypeKind::Pointer(Box::new(element_type)),
                })
            }
            TokenKind::KeywordConst => {
                let element_type = self.parse_type()?;

                Ok(Type {
                    span: Span::new(token.span().start(), element_type.span.end()),
                    kind: TypeKind::Const(Box::new(element_type)),
                })
            }
            _ => todo!(),
        }
    }
}
