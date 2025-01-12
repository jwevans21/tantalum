use tantalum_ast::{
    BinaryOperation, BinaryOperator, Expression, FunctionCall, Index, MemberAccess, TypeCast,
    UnaryOperation, UnaryOperator, Variable,
};
use tantalum_lexer::{token::Token, token_kind::TokenKind};
use tantalum_span::Spanned;

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
    ) -> Result<Spanned<'file_name, Expression<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        self.parse_expression_binary(0)
    }

    fn parse_expression_primary(
        &mut self,
    ) -> Result<Spanned<'file_name, Expression<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        match self.peek() {
            Some(token) => {
                if let Some(((), right_binding_power)) = Self::prefix_binding_power(token.kind()) {
                    self.next();

                    let operand = self.parse_expression_binary(right_binding_power)?;

                    Ok(Spanned::join_spans(
                        token.span(),
                        operand.span(),
                        Expression::UnaryOperation(UnaryOperation {
                            operator: Self::unary_operator_from_token(token),
                            operand: Box::from(operand),
                        }),
                    ))
                } else {
                    self.parse_expression_primary_start()
                }
            }
            None => Err(ParseError::unexpected_eof(self.source, self.eof)),
        }
    }

    fn parse_expression_primary_start(
        &mut self,
    ) -> Result<Spanned<'file_name, Expression<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let token = self.expect_any(Self::PRIMARY_START)?;

        let expr = match token.kind() {
            TokenKind::Identifier => token.map(|_| {
                Expression::Variable(Variable {
                    name: token.map(|name| name.lexeme()),
                })
            }),
            TokenKind::BinaryIntegerLiteral
            | TokenKind::OctalIntegerLiteral
            | TokenKind::DecimalIntegerLiteral
            | TokenKind::HexadecimalIntegerLiteral
            | TokenKind::FloatLiteral
            | TokenKind::KeywordTrue
            | TokenKind::KeywordFalse
            | TokenKind::CharacterLiteral
            | TokenKind::StringLiteral => self.parse_literal(token)?.map(Expression::Literal),
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

        Ok(expr)
    }

    #[allow(clippy::too_many_lines)]
    fn parse_expression_binary(
        &mut self,
        minimum_binding_power: u8,
    ) -> Result<Spanned<'file_name, Expression<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
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

                        lhs = Spanned::join_spans(
                            lhs.span(),
                            close.span(),
                            Expression::FunctionCall(FunctionCall {
                                function: Box::new(lhs),
                                arguments,
                            }),
                        );
                    }
                    TokenKind::LeftBracket => {
                        let index = self.parse_expression()?;
                        let close = self.expect(TokenKind::RightBracket)?;

                        lhs = Spanned::join_spans(
                            lhs.span(),
                            close.span(),
                            Expression::Index(Index {
                                object: Box::new(lhs),
                                index: Box::new(index),
                            }),
                        );
                    }
                    TokenKind::Dot => {
                        let member = self.expect(TokenKind::Identifier)?;

                        lhs = Spanned::join_spans(
                            lhs.span(),
                            member.span(),
                            Expression::MemberAccess(MemberAccess {
                                object: Box::new(lhs),
                                member: member.map(|member| member.lexeme()),
                            }),
                        );
                    }
                    TokenKind::Colon => {
                        let ty = self.parse_type()?;

                        lhs = Spanned::join_spans(
                            lhs.span(),
                            ty.span(),
                            Expression::TypeCast(TypeCast {
                                value: Box::new(lhs),
                                ty,
                            }),
                        );
                    }
                    TokenKind::ColonColon => todo!("Parse Path Expression"),
                    _ => {
                        lhs = Spanned::join_spans(
                            lhs.span(),
                            operator.span(),
                            Expression::UnaryOperation(UnaryOperation {
                                operator: Self::unary_operator_from_token(operator),
                                operand: Box::from(lhs),
                            }),
                        );
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

            lhs = Spanned::join_spans(
                lhs.span(),
                rhs.span(),
                Expression::BinaryOperation(BinaryOperation {
                    left: Box::from(lhs),
                    operator: Self::binary_operator_from_token(operator),
                    right: Box::from(rhs),
                }),
            );
        }
        dbg!(&lhs);

        Ok(lhs)
    }

    fn unary_operator_from_token(
        token: Spanned<'file_name, Token<'source>>,
    ) -> Spanned<'file_name, UnaryOperator> {
        token.map(|token| match token.kind() {
            TokenKind::Minus => UnaryOperator::Negation,
            TokenKind::Exclamation => UnaryOperator::LogicalNegation,
            TokenKind::Tilde => UnaryOperator::BitwiseNegation,
            TokenKind::DotStar => UnaryOperator::Deref,
            TokenKind::DotAmpersand => UnaryOperator::Ref,
            _ => unimplemented!("No Known Unary Operator for Token: {:?}", token),
        })
    }

    fn binary_operator_from_token(
        token: Spanned<'file_name, Token<'source>>,
    ) -> Spanned<'file_name, BinaryOperator> {
        token.map(|token| match token.kind() {
            TokenKind::Plus => BinaryOperator::Addition,
            TokenKind::Minus => BinaryOperator::Subtraction,
            TokenKind::Star => BinaryOperator::Multiplication,
            TokenKind::Slash => BinaryOperator::Division,
            TokenKind::Percent => BinaryOperator::Modulus,

            TokenKind::Ampersand => BinaryOperator::BitwiseAnd,
            TokenKind::Pipe => BinaryOperator::BitwiseOr,
            TokenKind::Caret => BinaryOperator::BitwiseXor,
            TokenKind::LeftAngleLeftAngle => BinaryOperator::LeftShift,
            TokenKind::RightAngleRightAngle => BinaryOperator::RightShift,

            TokenKind::LeftAngle => BinaryOperator::LessThan,
            TokenKind::LeftAngleEqual => BinaryOperator::LessThanOrEqual,
            TokenKind::RightAngle => BinaryOperator::GreaterThan,
            TokenKind::RightAngleEqual => BinaryOperator::GreaterThanOrEqual,

            TokenKind::EqualEqual => BinaryOperator::Equal,
            TokenKind::ExclamationEqual => BinaryOperator::NotEqual,

            TokenKind::AmpersandAmpersand => BinaryOperator::LogicalAnd,
            TokenKind::PipePipe => BinaryOperator::LogicalOr,

            TokenKind::Equal => BinaryOperator::Assignment,

            _ => unimplemented!("No Known Binary Operator for Token: {:?}", token),
        })
    }
}
