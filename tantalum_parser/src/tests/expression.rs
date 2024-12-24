use tantalum_ast::{BinaryOperator, BinaryOperatorKind, Expression, ExpressionKind, Type, TypeKind};
use tantalum_lexer::Lexer;
use tantalum_span::Span;

use pretty_assertions::assert_eq;

use crate::Parser;

macro_rules! prefix_unary_expression_test_cases {
    {
        $($kind:ident: $operator:literal),*
    } => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $kind() {
                const SOURCE: &str = concat!($operator, "1");
                let lexer = tantalum_lexer::Lexer::new(stringify!($kind), SOURCE);
                let mut parser = crate::Parser::new(lexer);

                let result = parser.parse_expression();

                pretty_assertions::assert_eq!(
                    result,
                    Ok(
                        tantalum_ast::Expression {
                            span: tantalum_span::Span::new(stringify!($kind), 0, SOURCE.len(), 1, 1),
                            kind: tantalum_ast::ExpressionKind::UnaryOperation {
                                operator: tantalum_ast::UnaryOperator {
                                    span: tantalum_span::Span::new(stringify!($kind), 0, 1, 1, 1),
                                    kind: tantalum_ast::UnaryOperatorKind::$kind
                                },
                                operand: Box::new(
                                    tantalum_ast::Expression {
                                        span: tantalum_span::Span::new(stringify!($kind), 1, SOURCE.len(), 1, 2),
                                        kind: tantalum_ast::ExpressionKind::IntegerLiteral {
                                            value: "1",
                                            radix: 10
                                        }
                                    }
                                )
                            }
                        }
                    )
                );
            }
        )*
    };
}

macro_rules! postfix_unary_expression_test_cases {
    {
        $($kind:ident: $operator:literal),*
    } => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $kind() {
                const SOURCE: &str = concat!("1", $operator);
                let lexer = tantalum_lexer::Lexer::new(stringify!($kind), SOURCE);
                let mut parser = crate::Parser::new(lexer);

                let result = parser.parse_expression();

                pretty_assertions::assert_eq!(
                    result,
                    Ok(
                        tantalum_ast::Expression {
                            span: tantalum_span::Span::new(stringify!($kind), 0, SOURCE.len(), 1, 1),
                            kind: tantalum_ast::ExpressionKind::UnaryOperation {
                                operator: tantalum_ast::UnaryOperator {
                                    span: tantalum_span::Span::new(stringify!($kind), 1, SOURCE.len(), 1, 2),
                                    kind: tantalum_ast::UnaryOperatorKind::$kind
                                },
                                operand: Box::new(
                                    tantalum_ast::Expression {
                                        span: tantalum_span::Span::new(stringify!($kind), 0, 1, 1, 1),
                                        kind: tantalum_ast::ExpressionKind::IntegerLiteral {
                                            value: "1",
                                            radix: 10
                                        }
                                    }
                                )
                            }
                        }
                    )
                );
            }
        )*
    }
}

macro_rules! binary_expression_test_cases {
    {
        $($kind:ident: $operator:literal),*
    } => {
        #[allow(non_snake_case)]
        mod BinaryOperation {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $kind() {
                    const SOURCE: &str = concat!("1 ", $operator, " 2");
                    let lexer = tantalum_lexer::Lexer::new(stringify!($kind), SOURCE);
                    let mut parser = crate::Parser::new(lexer);

                    let result = parser.parse_expression();

                    pretty_assertions::assert_eq!(
                        result,
                        Ok(
                            tantalum_ast::Expression {
                                span: tantalum_span::Span::new(stringify!($kind), 0, SOURCE.len(), 1, 1),
                                kind: tantalum_ast::ExpressionKind::BinaryOperation {
                                    left: Box::new(
                                        tantalum_ast::Expression {
                                            span: tantalum_span::Span::new(stringify!($kind), 0, 1, 1, 1),
                                            kind: tantalum_ast::ExpressionKind::IntegerLiteral {
                                                value: "1",
                                                radix: 10
                                            }
                                        }
                                    ),
                                    operator: tantalum_ast::BinaryOperator {
                                        span: tantalum_span::Span::new(stringify!($kind), 2, (2 + ($operator).len()), 1, 3),
                                        kind: tantalum_ast::BinaryOperatorKind::$kind
                                    },
                                    right: Box::new(
                                        tantalum_ast::Expression {
                                            span: tantalum_span::Span::new(stringify!($kind), (3 + ($operator).len()), SOURCE.len(), 1, (4 + ($operator).len())),
                                            kind: tantalum_ast::ExpressionKind::IntegerLiteral {
                                                value: "2",
                                                radix: 10
                                            }
                                        }
                                    )
                                }
                            }
                        )
                    );
                }
            )*
        }
    };
}

prefix_unary_expression_test_cases! {
    Negation: "-",
    LogicalNegation: "!",
    BitwiseNegation: "~"
}

postfix_unary_expression_test_cases! {
    AddressOf: ".&",
    Deref: ".*"
}

binary_expression_test_cases! {
    Addition: "+",
    Subtraction: "-",
    Multiplication: "*",
    Division: "/",
    Modulus: "%",

    LogicalAnd: "&&",
    LogicalOr: "||",

    Equality: "==",
    NotEqual: "!=",
    LessThan: "<",
    LessThanOrEqual: "<=",
    GreaterThan: ">",
    GreaterThanOrEqual: ">=",

    BitwiseAnd: "&",
    BitwiseOr: "|",
    BitwiseXor: "^",

    ShiftLeft: "<<",
    ShiftRight: ">>"
}

#[test]
fn function_call() {
    const SOURCE: &str = "foo()";
    let lexer = tantalum_lexer::Lexer::new("function_call", SOURCE);
    let mut parser = crate::Parser::new(lexer);

    let result = parser.parse_expression();

    pretty_assertions::assert_eq!(
        result,
        Ok(tantalum_ast::Expression {
            span: tantalum_span::Span::new("function_call", 0, SOURCE.len(), 1, 1),
            kind: tantalum_ast::ExpressionKind::FunctionCall {
                source: Box::new(tantalum_ast::Expression {
                    span: tantalum_span::Span::new("function_call", 0, 3, 1, 1),
                    kind: tantalum_ast::ExpressionKind::Variable { name: "foo" }
                }),
                arguments: vec![]
            }
        })
    );
}

#[test]
fn function_call_with_arguments() {
    const SOURCE: &str = "foo(1, 2)";
    let lexer = tantalum_lexer::Lexer::new("function_call_with_arguments", SOURCE);
    let mut parser = crate::Parser::new(lexer);

    let result = parser.parse_expression();

    pretty_assertions::assert_eq!(
        result,
        Ok(tantalum_ast::Expression {
            span: tantalum_span::Span::new("function_call_with_arguments", 0, SOURCE.len(), 1, 1),
            kind: tantalum_ast::ExpressionKind::FunctionCall {
                source: Box::new(tantalum_ast::Expression {
                    span: tantalum_span::Span::new("function_call_with_arguments", 0, 3, 1, 1),
                    kind: tantalum_ast::ExpressionKind::Variable { name: "foo" }
                }),
                arguments: vec![
                    tantalum_ast::Expression {
                        span: tantalum_span::Span::new("function_call_with_arguments", 4, 5, 1, 5),
                        kind: tantalum_ast::ExpressionKind::IntegerLiteral {
                            value: "1",
                            radix: 10
                        }
                    },
                    tantalum_ast::Expression {
                        span: tantalum_span::Span::new("function_call_with_arguments", 7, 8, 1, 8),
                        kind: tantalum_ast::ExpressionKind::IntegerLiteral {
                            value: "2",
                            radix: 10
                        }
                    }
                ]
            }
        })
    );
}

#[test]
fn array_access() {
    const SOURCE: &str = "foo[1]";
    let lexer = tantalum_lexer::Lexer::new("array_access", SOURCE);
    let mut parser = crate::Parser::new(lexer);

    let result = parser.parse_expression();

    pretty_assertions::assert_eq!(
        result,
        Ok(tantalum_ast::Expression {
            span: tantalum_span::Span::new("array_access", 0, SOURCE.len(), 1, 1),
            kind: tantalum_ast::ExpressionKind::ArrayAccess {
                source: Box::new(tantalum_ast::Expression {
                    span: tantalum_span::Span::new("array_access", 0, 3, 1, 1),
                    kind: tantalum_ast::ExpressionKind::Variable { name: "foo" }
                }),
                index: Box::new(tantalum_ast::Expression {
                    span: tantalum_span::Span::new("array_access", 4, 5, 1, 5),
                    kind: tantalum_ast::ExpressionKind::IntegerLiteral {
                        value: "1",
                        radix: 10
                    }
                })
            }
        })
    );
}

#[test]
fn basic_addition() {
    let lexer = Lexer::new("basic_addition", "1 + 2");
    let mut parser = Parser::new(lexer);

    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            span: Span::new("basic_addition", 0, 5, 1, 1),
            kind: ExpressionKind::BinaryOperation {
                left: Box::new(Expression {
                    span: Span::new("basic_addition", 0, 1, 1, 1),
                    kind: ExpressionKind::IntegerLiteral {
                        value: "1",
                        radix: 10
                    }
                }),
                operator: BinaryOperator {
                    span: Span::new("basic_addition", 2, 3, 1, 3),
                    kind: BinaryOperatorKind::Addition
                },
                right: Box::new(Expression {
                    span: Span::new("basic_addition", 4, 5, 1, 5),
                    kind: ExpressionKind::IntegerLiteral {
                        value: "2",
                        radix: 10
                    }
                })
            }
        })
    );
}

#[test]
fn multiplication_with_addition_rhs() {
    let lexer = Lexer::new("multiplication_with_addition_rhs", "1 * 2 + 3");
    let mut parser = Parser::new(lexer);

    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            span: Span::new("multiplication_with_addition_rhs", 0, 9, 1, 1),
            kind: ExpressionKind::BinaryOperation {
                left: Box::new(Expression {
                    span: Span::new("multiplication_with_addition_rhs", 0, 5, 1, 1),
                    kind: ExpressionKind::BinaryOperation {
                        left: Box::new(Expression {
                            span: Span::new("multiplication_with_addition_rhs", 0, 1, 1, 1),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "1",
                                radix: 10
                            }
                        }),
                        operator: BinaryOperator {
                            span: Span::new("multiplication_with_addition_rhs", 2, 3, 1, 3),
                            kind: BinaryOperatorKind::Multiplication
                        },
                        right: Box::new(Expression {
                            span: Span::new("multiplication_with_addition_rhs", 4, 5, 1, 5),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "2",
                                radix: 10
                            }
                        })
                    }
                }),
                operator: BinaryOperator {
                    span: Span::new("multiplication_with_addition_rhs", 6, 7, 1, 7),
                    kind: BinaryOperatorKind::Addition
                },
                right: Box::new(Expression {
                    span: Span::new("multiplication_with_addition_rhs", 8, 9, 1, 9),
                    kind: ExpressionKind::IntegerLiteral {
                        value: "3",
                        radix: 10
                    }
                })
            }
        })
    );
}

#[test]
fn multiplication_with_addition_lhs() {
    let lexer = Lexer::new("multiplication_with_addition_lhs", "1 + 2 * 3");
    let mut parser = Parser::new(lexer);

    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            span: Span::new("multiplication_with_addition_lhs", 0, 9, 1, 1),
            kind: ExpressionKind::BinaryOperation {
                left: Box::new(Expression {
                    span: Span::new("multiplication_with_addition_lhs", 0, 1, 1, 1),
                    kind: ExpressionKind::IntegerLiteral {
                        value: "1",
                        radix: 10
                    }
                }),
                operator: BinaryOperator {
                    span: Span::new("multiplication_with_addition_lhs", 2, 3, 1, 3),
                    kind: BinaryOperatorKind::Addition
                },
                right: Box::new(Expression {
                    span: Span::new("multiplication_with_addition_lhs", 4, 9, 1, 5),
                    kind: ExpressionKind::BinaryOperation {
                        left: Box::new(Expression {
                            span: Span::new("multiplication_with_addition_lhs", 4, 5, 1, 5),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "2",
                                radix: 10
                            }
                        }),
                        operator: BinaryOperator {
                            span: Span::new("multiplication_with_addition_lhs", 6, 7, 1, 7),
                            kind: BinaryOperatorKind::Multiplication
                        },
                        right: Box::new(Expression {
                            span: Span::new("multiplication_with_addition_lhs", 8, 9, 1, 9),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "3",
                                radix: 10
                            }
                        })
                    }
                })
            }
        })
    );
}

#[test]
fn type_cast() {
    let lexer = Lexer::new("type_cast", "1:u8");
    let mut parser = Parser::new(lexer);

    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            span: Span::new("type_cast", 0, 4, 1, 1),
            kind: ExpressionKind::TypeCast {
                expression: Box::new(Expression {
                    span: Span::new("type_cast", 0, 1, 1, 1),
                    kind: ExpressionKind::IntegerLiteral {
                        value: "1",
                        radix: 10
                    }
                }),
                ty: Type {
                    span: Span::new("type_cast", 2, 4, 1, 3),
                    kind: TypeKind::Named("u8")
                }
            }
        })
    );
}

#[test]
fn type_cast_with_binary_expression() {
    let lexer = Lexer::new("type_cast_with_binary_expression", "1 + 2:u8");
    let mut parser = Parser::new(lexer);

    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            span: Span::new("type_cast_with_binary_expression", 0, 8, 1, 1),
            kind: ExpressionKind::BinaryOperation {
                left: Box::new(Expression {
                    span: Span::new("type_cast_with_binary_expression", 0, 1, 1, 1),
                    kind: ExpressionKind::IntegerLiteral {
                        value: "1",
                        radix: 10
                    }
                }),
                operator: BinaryOperator {
                    span: Span::new("type_cast_with_binary_expression", 2, 3, 1, 3),
                    kind: BinaryOperatorKind::Addition
                },
                right: Box::new(Expression {
                    span: Span::new("type_cast_with_binary_expression", 4, 8, 1, 5),
                    kind: ExpressionKind::TypeCast {
                        ty: Type {
                            span: Span::new("type_cast_with_binary_expression", 6, 8, 1, 7),
                            kind: TypeKind::Named("u8")
                        },
                        expression: Box::new(Expression {
                            span: Span::new("type_cast_with_binary_expression", 4, 5, 1, 5),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "2",
                                radix: 10
                            }
                        })
                    }
                })
            }
        })
    );
}
