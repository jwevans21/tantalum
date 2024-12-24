use tantalum_ast::{
    BinaryOperator, BinaryOperatorKind, Expression, ExpressionKind, UnaryOperator,
    UnaryOperatorKind,
};
use tantalum_lexer::{token::Token, token_kind::TokenKind};

use crate::{ParseError, Parser};

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

    const UNARY_START: &'static [TokenKind] =
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

    pub(crate) fn parse_expression(
        &mut self,
    ) -> Result<Expression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let lhs = self.parse_expression_unary()?;

        self.parse_expression_binary(0, lhs)
    }

    fn parse_expression_unary(
        &mut self,
    ) -> Result<Expression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let Some(token) = self.advance_if_any(Self::UNARY_START) else {
            return self.parse_expression_primary();
        };

        let operand = self.parse_expression_unary()?;

        Ok(Expression {
            span: token.span().extend(&operand.span),
            kind: ExpressionKind::UnaryOperation {
                operator: Self::unary_operator_from_token(token),
                operand: Box::from(operand),
            },
        })
    }

    fn parse_expression_primary(
        &mut self,
    ) -> Result<Expression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let Some(token) = self.advance_if_any(Self::PRIMARY_START) else {
            return Err(ParseError::unexpected_eof(self.source, self.eof));
        };

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
            _ => unreachable!(
                "Already expected in set of primary expression starts ({:?})",
                Self::PRIMARY_START
            ),
        };

        self.parse_expression_postfix(expr)
    }

    fn parse_expression_postfix(
        &mut self,
        mut lhs: Expression<'file_name, 'source>,
    ) -> Result<Expression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        while let Some(token) = self.advance_if_any(Self::POSTFIX_START) {
            match token.kind() {
                TokenKind::DotAmpersand | TokenKind::DotStar => {
                    lhs = Expression {
                        span: lhs.span.extend(&token.span()),
                        kind: ExpressionKind::UnaryOperation {
                            operator: Self::unary_operator_from_token(token),
                            operand: Box::from(lhs),
                        },
                    };
                }
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
                        span: lhs.span.extend(&close.span()),
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
                        span: lhs.span.extend(&close.span()),
                        kind: ExpressionKind::ArrayAccess {
                            source: Box::from(lhs),
                            index: Box::from(index),
                        },
                    };
                }
                TokenKind::Dot => {
                    let field = self.expect(TokenKind::Identifier)?;

                    lhs = Expression {
                        span: lhs.span.extend(&field.span()),
                        kind: ExpressionKind::MemberAccess {
                            source: Box::from(lhs),
                            member: field.lexeme(),
                        },
                    };
                }
                TokenKind::Colon => {
                    let ty = self.parse_type()?;

                    lhs = Expression {
                        span: lhs.span.extend(&ty.span),
                        kind: ExpressionKind::TypeCast {
                            ty,
                            expression: Box::from(lhs),
                        },
                    };
                }
                TokenKind::ColonColon => todo!("Parse Path Expression"),
                _ => unimplemented!(
                    "The Postfix Token: {:?} Is Not In The Set {:?}",
                    token,
                    Self::POSTFIX_START
                ),
            }
        }

        Ok(lhs)
    }

    fn parse_expression_binary(
        &mut self,
        current_precedence: u8,
        mut lhs: Expression<'file_name, 'source>,
    ) -> Result<Expression<'file_name, 'source>, ParseError<'file_name, 'source>> {
        loop {
            let Some(token) = self.advance_if_any(Self::BINARY_OPERATOR) else {
                return Ok(lhs);
            };

            let operator = Self::binary_operator_from_token(token);
            let precedence = Self::precedence_of_operator(&operator.kind);

            if precedence < current_precedence {
                self.put_back(1);
                return Ok(lhs);
            }

            let mut rhs = self.parse_expression_unary()?;

            if let Some(next_token) = self.is_at_any(Self::BINARY_OPERATOR) {
                let next_operator = Self::binary_operator_from_token(next_token);
                let next_precedence = Self::precedence_of_operator(&next_operator.kind);

                if precedence < next_precedence {
                    rhs = self.parse_expression_binary(precedence + 1, rhs)?;
                }
            }

            lhs = Expression {
                span: lhs.span.extend(&rhs.span),
                kind: ExpressionKind::BinaryOperation {
                    left: Box::from(lhs),
                    operator,
                    right: Box::from(rhs),
                },
            };
        }
    }

    fn unary_operator_from_token(token: Token<'file_name, 'source>) -> UnaryOperator<'file_name> {
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

    fn binary_operator_from_token(token: Token<'file_name, 'source>) -> BinaryOperator<'file_name> {
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

    fn precedence_of_operator(operator: &BinaryOperatorKind) -> u8 {
        [
            // Assignment Operators
            [BinaryOperatorKind::Assignment].as_slice(),
            // Logical Operators
            [
                BinaryOperatorKind::LogicalAnd,
                BinaryOperatorKind::LogicalOr,
            ]
            .as_slice(),
            // Bitwise Operators
            [
                BinaryOperatorKind::BitwiseAnd,
                BinaryOperatorKind::BitwiseOr,
                BinaryOperatorKind::BitwiseXor,
            ]
            .as_slice(),
            // Equality Operators
            [BinaryOperatorKind::Equality, BinaryOperatorKind::NotEqual].as_slice(),
            // Relational Operators
            [
                BinaryOperatorKind::LessThan,
                BinaryOperatorKind::LessThanOrEqual,
                BinaryOperatorKind::GreaterThan,
                BinaryOperatorKind::GreaterThanOrEqual,
            ]
            .as_slice(),
            // Shift Operators
            [
                BinaryOperatorKind::ShiftLeft,
                BinaryOperatorKind::ShiftRight,
            ]
            .as_slice(),
            // Additive Operators
            [
                BinaryOperatorKind::Addition,
                BinaryOperatorKind::Subtraction,
            ]
            .as_slice(),
            // Multiplicative Operators
            [
                BinaryOperatorKind::Multiplication,
                BinaryOperatorKind::Division,
                BinaryOperatorKind::Modulus,
            ]
            .as_slice(),
        ]
        .iter()
        .position(|set| set.contains(operator))
        .map_or(0, |index| (u8::try_from(index).unwrap_or(u8::MAX) + 1) * 10)
    }
}
