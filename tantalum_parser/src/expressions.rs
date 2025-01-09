use tantalum_ast::{
    BinaryOperator, BinaryOperatorKind, Expression, ExpressionKind, UnaryOperator,
    UnaryOperatorKind,
};
use tantalum_lexer::{token::Token, token_kind::TokenKind};
use tantalum_span::{Span, Spanned};

use crate::{ParseError, Parser};

// This expression parsing code is based on the Pratt Parsing walkthrough
// here: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
impl<'file_name, 'source> Parser<'file_name, 'source> {
    pub const EXPRESSION_START: &'static [TokenKind] = &[
        // Primary Expression Start
        TokenKind::Identifier,
        TokenKind::BinaryIntegerLiteral,
        TokenKind::OctalIntegerLiteral,
        TokenKind::DecimalIntegerLiteral,
        TokenKind::HexadecimalIntegerLiteral,
        TokenKind::FloatLiteral,
        TokenKind::StringLiteral,
        TokenKind::CharacterLiteral,
        TokenKind::KeywordTrue,
        TokenKind::KeywordFalse,
        TokenKind::LeftParen,
        // Unary Expression Start
        TokenKind::Minus,
        TokenKind::Exclamation,
        TokenKind::Tilde,
    ];

    const PREFIX_START: &'static [TokenKind] =
        &[TokenKind::Minus, TokenKind::Exclamation, TokenKind::Tilde];

    const PRIMARY_START: &'static [TokenKind] = &[
        TokenKind::Identifier,
        TokenKind::BinaryIntegerLiteral,
        TokenKind::OctalIntegerLiteral,
        TokenKind::DecimalIntegerLiteral,
        TokenKind::HexadecimalIntegerLiteral,
        TokenKind::FloatLiteral,
        TokenKind::StringLiteral,
        TokenKind::CharacterLiteral,
        TokenKind::KeywordTrue,
        TokenKind::KeywordFalse,
        TokenKind::LeftParen,
    ];

    const POSTFIX_START: &'static [TokenKind] = &[
        TokenKind::LeftParen,
        TokenKind::LeftBracket,
        TokenKind::Dot,
        TokenKind::DotAmpersand,
        TokenKind::DotStar,
        TokenKind::Colon,
        TokenKind::ColonColon,
    ];

    const BINARY_OPERATOR: &'static [TokenKind] = &[
        TokenKind::AmpersandAmpersand,
        TokenKind::PipePipe,
        TokenKind::Ampersand,
        TokenKind::Pipe,
        TokenKind::Caret,
        TokenKind::EqualEqual,
        TokenKind::ExclamationEqual,
        TokenKind::LeftAngle,
        TokenKind::LeftAngleEqual,
        TokenKind::RightAngle,
        TokenKind::RightAngleEqual,
        TokenKind::LeftAngleLeftAngle,
        TokenKind::RightAngleRightAngle,
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Slash,
        TokenKind::Percent,
        TokenKind::Equal,
    ];

    fn prefix_binding_power(kind: TokenKind) -> Option<((), u8)> {
        match kind {
            _ if Self::PREFIX_START.contains(&kind) => Some(((), 100)),
            _ => None,
        }
    }

    fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
        match kind {
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some((80, 81)),
            TokenKind::Plus | TokenKind::Minus => Some((70, 71)),
            TokenKind::LeftAngleLeftAngle | TokenKind::RightAngleRightAngle => Some((60, 61)),
            TokenKind::LeftAngle
            | TokenKind::LeftAngleEqual
            | TokenKind::RightAngle
            | TokenKind::RightAngleEqual => Some((50, 51)),
            TokenKind::EqualEqual | TokenKind::ExclamationEqual => Some((40, 41)),
            TokenKind::Ampersand => Some((30, 31)),
            TokenKind::Caret => Some((20, 21)),
            TokenKind::Pipe => Some((10, 11)),
            TokenKind::AmpersandAmpersand => Some((5, 6)),
            TokenKind::PipePipe => Some((3, 4)),
            TokenKind::Equal => Some((1, 2)),
            _ if Self::BINARY_OPERATOR.contains(&kind) => {
                panic!("token contained in BINARY_OPERATOR, but does not have a binding power")
            }
            _ => None,
        }
    }

    fn postfix_binding_power(kind: TokenKind) -> Option<(u8, ())> {
        match kind {
            _ if Self::POSTFIX_START.contains(&kind) => Some((101, ())),
            _ => None,
        }
    }

    pub(crate) fn parse_expression(
        &mut self,
    ) -> Result<Expression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        self.parse_expression_binary(0)
    }

    fn parse_expression_primary(
        &mut self,
    ) -> Result<Expression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        match self.peek() {
            Some(token) => {
                if let Some(((), right_binding_power)) = Self::prefix_binding_power(token.kind()) {
                    self.next();

                    let operand = self.parse_expression_binary(right_binding_power)?;

                    Ok(Expression {
                        span: Span::new(token.span().start(), operand.span.end()),
                        kind: ExpressionKind::UnaryOperation {
                            operator: Self::unary_operator_from_token(token),
                            operand: Box::from(operand),
                        },
                    })
                } else {
                    self.parse_expression_primary_start()
                }
            }
            None => Err(ParseError::unexpected_eof(self.source, self.eof)),
        }
    }

    fn parse_expression_primary_start(
        &mut self,
    ) -> Result<Expression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let token = self.expect_any(
            [Self::PRIMARY_START, Self::TYPE_START_SET]
                .concat()
                .as_slice(),
        )?;
        let expr = match token.kind() {
            TokenKind::Identifier => Expression {
                span: token.span(),
                kind: ExpressionKind::Variable {
                    name: token.lexeme(),
                },
            },
            TokenKind::BinaryIntegerLiteral => Expression {
                span: token.span(),
                kind: ExpressionKind::IntegerLiteral {
                    value: token.lexeme(),
                    radix: 2,
                },
            },
            TokenKind::OctalIntegerLiteral => Expression {
                span: token.span(),
                kind: ExpressionKind::IntegerLiteral {
                    value: token.lexeme(),
                    radix: 8,
                },
            },
            TokenKind::DecimalIntegerLiteral => Expression {
                span: token.span(),
                kind: ExpressionKind::IntegerLiteral {
                    value: token.lexeme(),
                    radix: 10,
                },
            },
            TokenKind::HexadecimalIntegerLiteral => Expression {
                span: token.span(),
                kind: ExpressionKind::IntegerLiteral {
                    value: token.lexeme(),
                    radix: 16,
                },
            },
            TokenKind::FloatLiteral => Expression {
                span: token.span(),
                kind: ExpressionKind::FloatLiteral {
                    value: token.lexeme(),
                },
            },
            TokenKind::StringLiteral => Expression {
                span: token.span(),
                kind: ExpressionKind::StringLiteral {
                    value: token.lexeme(),
                },
            },
            TokenKind::CharacterLiteral => Expression {
                span: token.span(),
                kind: ExpressionKind::CharacterLiteral {
                    value: token.lexeme(),
                },
            },
            TokenKind::KeywordTrue => Expression {
                span: token.span(),
                kind: ExpressionKind::BooleanLiteral { value: true },
            },
            TokenKind::KeywordFalse => Expression {
                span: token.span(),
                kind: ExpressionKind::BooleanLiteral { value: false },
            },
            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RightParen)?;

                expr
            }
            kind if Self::TYPE_START_SET.contains(&kind) => {
                let ty = self.parse_type()?;
                self.expect(TokenKind::LeftParen)?;

                let expr = self.parse_expression()?;
                self.expect(TokenKind::RightParen)?;

                Expression {
                    span: Span::new(ty.span.start(), expr.span.end()),
                    kind: ExpressionKind::TypeCast {
                        ty,
                        expression: Box::from(expr),
                    },
                }
            }
            _ => unreachable!(
                "Already expected in set of primary expression starts ({:?})",
                Self::PRIMARY_START
            ),
        };

        Ok(expr)
    }

    #[allow(clippy::too_many_lines)]
    fn parse_expression_binary(
        &mut self,
        minimum_binding_power: u8,
    ) -> Result<Expression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let mut lhs = self.parse_expression_primary()?;

        while let Some(token) = self.peek() {
            let operator = if Self::infix_binding_power(token.kind()).is_some()
                || Self::postfix_binding_power(token.kind()).is_some()
            {
                token
            } else {
                break;
            };

            if let Some((left_binding_power, ())) = Self::postfix_binding_power(operator.kind()) {
                if left_binding_power < minimum_binding_power {
                    break;
                }
                self.next();

                match operator.kind() {
                    TokenKind::LeftParen => {
                        let mut arguments = Vec::new();

                        if self.is_at(TokenKind::RightParen).is_none() {
                            loop {
                                arguments.push(self.parse_expression()?);

                                if self.advance_if(TokenKind::Comma).is_none() {
                                    break;
                                }
                            }
                        }

                        let close = self.expect(TokenKind::RightParen)?;

                        lhs = Expression {
                            span: Span::new(lhs.span.start(), close.span().end()),
                            kind: ExpressionKind::FunctionCall {
                                source: Box::from(lhs),
                                arguments,
                            },
                        };
                    }
                    TokenKind::LeftBracket => {
                        let index = self.parse_expression()?;
                        let close = self.expect(TokenKind::RightBracket)?;

                        lhs = Expression {
                            span: Span::new(lhs.span.start(), close.span().end()),
                            kind: ExpressionKind::ArrayAccess {
                                source: Box::from(lhs),
                                index: Box::from(index),
                            },
                        };
                    }
                    TokenKind::Dot => {
                        let member = self.expect(TokenKind::Identifier)?;

                        lhs = Expression {
                            span: Span::new(lhs.span.start(), member.span().end()),
                            kind: ExpressionKind::MemberAccess {
                                source: Box::from(lhs),
                                member: member.lexeme(),
                            },
                        };
                    }
                    TokenKind::Colon => {
                        let ty = self.parse_type()?;

                        lhs = Expression {
                            span: Span::new(lhs.span.start(), ty.span.end()),
                            kind: ExpressionKind::TypeCast {
                                ty,
                                expression: Box::from(lhs),
                            },
                        };
                    }
                    TokenKind::ColonColon => todo!("Parse Path Expression"),
                    _ => {
                        lhs = Expression {
                            span: Span::new(lhs.span.start(), operator.span().end()),
                            kind: ExpressionKind::UnaryOperation {
                                operator: Self::unary_operator_from_token(operator),
                                operand: Box::from(lhs),
                            },
                        };
                    }
                }

                continue;
            }

            let (left_binding_power, right_binding_power) =
                Self::infix_binding_power(operator.kind()).expect("token is not an infix operator");

            if left_binding_power < minimum_binding_power {
                break;
            }

            self.next();

            let rhs = self.parse_expression_binary(right_binding_power)?;

            lhs = Expression {
                span: Span::new(lhs.span.start(), rhs.span.end()),
                kind: ExpressionKind::BinaryOperation {
                    left: Box::from(lhs),
                    operator: Self::binary_operator_from_token(operator),
                    right: Box::from(rhs),
                },
            };
        }
        dbg!(&lhs);

        Ok(lhs)
    }

    fn unary_operator_from_token(
        token: Spanned<'file_name, Token<'source>>,
    ) -> UnaryOperator<'file_name> {
        match token.kind() {
            TokenKind::Minus => UnaryOperator {
                span: token.span(),
                kind: UnaryOperatorKind::Negation,
            },
            TokenKind::Exclamation => UnaryOperator {
                span: token.span(),
                kind: UnaryOperatorKind::LogicalNegation,
            },
            TokenKind::Tilde => UnaryOperator {
                span: token.span(),
                kind: UnaryOperatorKind::BitwiseNegation,
            },
            TokenKind::DotAmpersand => UnaryOperator {
                span: token.span(),
                kind: UnaryOperatorKind::AddressOf,
            },
            TokenKind::DotStar => UnaryOperator {
                span: token.span(),
                kind: UnaryOperatorKind::Deref,
            },
            _ => unimplemented!("No Known Unary Operator for Token: {:?}", token),
        }
    }

    fn binary_operator_from_token(
        token: Spanned<'file_name, Token<'source>>,
    ) -> BinaryOperator<'file_name> {
        match token.kind() {
            TokenKind::Equal => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::Assignment,
            },

            TokenKind::AmpersandAmpersand => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::LogicalAnd,
            },
            TokenKind::PipePipe => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::LogicalOr,
            },

            TokenKind::Ampersand => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::BitwiseAnd,
            },
            TokenKind::Pipe => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::BitwiseOr,
            },
            TokenKind::Caret => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::BitwiseXor,
            },

            TokenKind::EqualEqual => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::Equality,
            },
            TokenKind::ExclamationEqual => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::NotEqual,
            },

            TokenKind::LeftAngle => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::LessThan,
            },
            TokenKind::LeftAngleEqual => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::LessThanOrEqual,
            },
            TokenKind::RightAngle => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::GreaterThan,
            },
            TokenKind::RightAngleEqual => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::GreaterThanOrEqual,
            },

            TokenKind::LeftAngleLeftAngle => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::ShiftLeft,
            },
            TokenKind::RightAngleRightAngle => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::ShiftRight,
            },

            TokenKind::Plus => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::Addition,
            },
            TokenKind::Minus => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::Subtraction,
            },

            TokenKind::Star => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::Multiplication,
            },
            TokenKind::Slash => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::Division,
            },
            TokenKind::Percent => BinaryOperator {
                span: token.span(),
                kind: BinaryOperatorKind::Modulus,
            },
            _ => unimplemented!("No Known Binary Operator for Token: {:?}", token),
        }
    }
}
