use tantalum_ast::{
    BinaryOperator, BinaryOperatorKind, Expression, ExpressionKind, Statement, StatementKind, Type,
    TypeKind,
};
use tantalum_lexer::Lexer;
use tantalum_span::Span;

use pretty_assertions::assert_eq;

use crate::Parser;

#[test]
fn expression_statement() {
    let source = "42 + 42;";
    let mut parser = Parser::new(Lexer::new("expression_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("expression_statement", 0, 8, 1, 1),
            kind: StatementKind::Expression {
                expression: Expression {
                    span: Span::new("expression_statement", 0, 7, 1, 1),
                    kind: ExpressionKind::BinaryOperation {
                        left: Box::new(Expression {
                            span: Span::new("expression_statement", 0, 2, 1, 1),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "42",
                                radix: 10
                            },
                        }),
                        operator: BinaryOperator {
                            span: Span::new("expression_statement", 3, 4, 1, 4),
                            kind: BinaryOperatorKind::Addition,
                        },
                        right: Box::new(Expression {
                            span: Span::new("expression_statement", 5, 7, 1, 6),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "42",
                                radix: 10
                            },
                        }),
                    },
                },
            },
        })
    );
}

#[test]
fn let_statement() {
    let source = "let x = 42;";
    let mut parser = Parser::new(Lexer::new("let_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("let_statement", 0, 11, 1, 1),
            kind: StatementKind::VariableDeclaration {
                name: "x",
                ty: None,
                value: Expression {
                    span: Span::new("let_statement", 8, 10, 1, 9),
                    kind: ExpressionKind::IntegerLiteral {
                        value: "42",
                        radix: 10
                    },
                },
            },
        })
    );
}

#[test]
fn let_statement_with_type() {
    let source = "let x: *u8 = \"Hello, World!\";";
    let mut parser = Parser::new(Lexer::new("let_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("let_statement", 0, 29, 1, 1),
            kind: StatementKind::VariableDeclaration {
                name: "x",
                ty: Some(Type {
                    span: Span::new("let_statement", 7, 10, 1, 8),
                    kind: TypeKind::Pointer(Box::new(Type {
                        span: Span::new("let_statement", 8, 10, 1, 9),
                        kind: TypeKind::Named("u8"),
                    })),
                }),
                value: Expression {
                    span: Span::new("let_statement", 13, 28, 1, 14),
                    kind: ExpressionKind::StringLiteral {
                        value: "\"Hello, World!\""
                    },
                },
            },
        })
    );
}

#[test]
fn if_statement() {
    let source = "if true { return 42; }";
    let mut parser = Parser::new(Lexer::new("if_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("if_statement", 0, 22, 1, 1),
            kind: StatementKind::If {
                condition: Expression {
                    span: Span::new("if_statement", 3, 7, 1, 4),
                    kind: ExpressionKind::BooleanLiteral { value: true },
                },
                body: Box::new(Statement {
                    span: Span::new("if_statement", 8, 22, 1, 9),
                    kind: StatementKind::Block {
                        statements: vec![Statement {
                            span: Span::new("if_statement", 10, 20, 1, 11),
                            kind: StatementKind::Return {
                                value: Some(Expression {
                                    span: Span::new("if_statement", 17, 19, 1, 18),
                                    kind: ExpressionKind::IntegerLiteral {
                                        value: "42",
                                        radix: 10
                                    },
                                }),
                            },
                        }],
                    },
                }),
                else_branch: None,
            },
        })
    );
}

#[test]
fn if_else_statement() {
    let source = "if true { return 42; } else { return 0; }";
    let mut parser = Parser::new(Lexer::new("if_else_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("if_else_statement", 0, 41, 1, 1),
            kind: StatementKind::If {
                condition: Expression {
                    span: Span::new("if_else_statement", 3, 7, 1, 4),
                    kind: ExpressionKind::BooleanLiteral { value: true },
                },
                body: Box::new(Statement {
                    span: Span::new("if_else_statement", 8, 22, 1, 9),
                    kind: StatementKind::Block {
                        statements: vec![Statement {
                            span: Span::new("if_else_statement", 10, 20, 1, 11),
                            kind: StatementKind::Return {
                                value: Some(Expression {
                                    span: Span::new("if_else_statement", 17, 19, 1, 18),
                                    kind: ExpressionKind::IntegerLiteral {
                                        value: "42",
                                        radix: 10
                                    },
                                }),
                            },
                        }],
                    },
                }),
                else_branch: Some(Box::new(Statement {
                    span: Span::new("if_else_statement", 28, 41, 1, 29),
                    kind: StatementKind::Block {
                        statements: vec![Statement {
                            span: Span::new("if_else_statement", 30, 39, 1, 31),
                            kind: StatementKind::Return {
                                value: Some(Expression {
                                    span: Span::new("if_else_statement", 37, 38, 1, 38),
                                    kind: ExpressionKind::IntegerLiteral {
                                        value: "0",
                                        radix: 10
                                    },
                                }),
                            },
                        }],
                    },
                })),
            },
        })
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn for_loop_statement() {
    let source = "for let i = 0; i < 10; i = i + 1; { return i; }";
    let mut parser = Parser::new(Lexer::new("for_loop_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("for_loop_statement", 0, 47, 1, 1),
            kind: StatementKind::ForInitCondUpdate {
                initialization: Box::new(Statement {
                    span: Span::new("for_loop_statement", 4, 14, 1, 5),
                    kind: StatementKind::VariableDeclaration {
                        name: "i",
                        ty: None,
                        value: Expression {
                            span: Span::new("for_loop_statement", 12, 13, 1, 13),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "0",
                                radix: 10,
                            },
                        },
                    },
                }),
                condition: Box::new(Statement {
                    span: Span::new("for_loop_statement", 15, 22, 1, 16),
                    kind: StatementKind::Expression {
                        expression: Expression {
                            span: Span::new("for_loop_statement", 15, 21, 1, 16),
                            kind: ExpressionKind::BinaryOperation {
                                left: Box::new(Expression {
                                    span: Span::new("for_loop_statement", 15, 16, 1, 16),
                                    kind: ExpressionKind::Variable { name: "i" },
                                }),
                                operator: BinaryOperator {
                                    span: Span::new("for_loop_statement", 17, 18, 1, 18),
                                    kind: BinaryOperatorKind::LessThan,
                                },
                                right: Box::new(Expression {
                                    span: Span::new("for_loop_statement", 19, 21, 1, 20),
                                    kind: ExpressionKind::IntegerLiteral {
                                        value: "10",
                                        radix: 10,
                                    },
                                }),
                            },
                        },
                    },
                }),
                update: Box::new(Statement {
                    span: Span::new("for_loop_statement", 23, 33, 1, 24),
                    kind: StatementKind::Expression {
                        expression: Expression {
                            span: Span::new("for_loop_statement", 23, 32, 1, 24),
                            kind: ExpressionKind::BinaryOperation {
                                left: Box::new(Expression {
                                    span: Span::new("for_loop_statement", 23, 24, 1, 24),
                                    kind: ExpressionKind::Variable { name: "i" },
                                }),
                                operator: BinaryOperator {
                                    span: Span::new("for_loop_statement", 25, 26, 1, 26),
                                    kind: BinaryOperatorKind::Assignment,
                                },
                                right: Box::new(Expression {
                                    span: Span::new("for_loop_statement", 27, 32, 1, 28),
                                    kind: ExpressionKind::BinaryOperation {
                                        left: Box::new(Expression {
                                            span: Span::new("for_loop_statement", 27, 28, 1, 28),
                                            kind: ExpressionKind::Variable { name: "i" },
                                        }),
                                        operator: BinaryOperator {
                                            span: Span::new("for_loop_statement", 29, 30, 1, 30),
                                            kind: BinaryOperatorKind::Addition,
                                        },
                                        right: Box::new(Expression {
                                            span: Span::new("for_loop_statement", 31, 32, 1, 32),
                                            kind: ExpressionKind::IntegerLiteral {
                                                value: "1",
                                                radix: 10,
                                            },
                                        }),
                                    },
                                }),
                            },
                        },
                    },
                }),
                body: Box::new(Statement {
                    span: Span::new("for_loop_statement", 34, 47, 1, 35),
                    kind: StatementKind::Block {
                        statements: vec![Statement {
                            span: Span::new("for_loop_statement", 36, 45, 1, 37),
                            kind: StatementKind::Return {
                                value: Some(Expression {
                                    span: Span::new("for_loop_statement", 43, 44, 1, 44),
                                    kind: ExpressionKind::Variable { name: "i" },
                                }),
                            },
                        }],
                    },
                }),
            }
        })
    );
}

#[test]
fn while_statement() {
    let source = "while i < 10 { i = i + 1; }";
    let mut parser = Parser::new(Lexer::new("while_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("while_statement", 0, 27, 1, 1),
            kind: StatementKind::While {
                condition: Expression {
                    span: Span::new("while_statement", 6, 12, 1, 7),
                    kind: ExpressionKind::BinaryOperation {
                        left: Box::new(Expression {
                            span: Span::new("while_statement", 6, 7, 1, 7),
                            kind: ExpressionKind::Variable { name: "i" },
                        }),
                        operator: BinaryOperator {
                            span: Span::new("while_statement", 8, 9, 1, 9),
                            kind: BinaryOperatorKind::LessThan,
                        },
                        right: Box::new(Expression {
                            span: Span::new("while_statement", 10, 12, 1, 11),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "10",
                                radix: 10,
                            },
                        }),
                    },
                },
                body: Box::new(Statement {
                    span: Span::new("while_statement", 13, 27, 1, 14),
                    kind: StatementKind::Block {
                        statements: vec![Statement {
                            span: Span::new("while_statement", 15, 25, 1, 16),
                            kind: StatementKind::Expression {
                                expression: Expression {
                                    span: Span::new("while_statement", 15, 24, 1, 16),
                                    kind: ExpressionKind::BinaryOperation {
                                        left: Box::new(Expression {
                                            span: Span::new("while_statement", 15, 16, 1, 16),
                                            kind: ExpressionKind::Variable { name: "i" },
                                        }),
                                        operator: BinaryOperator {
                                            span: Span::new("while_statement", 17, 18, 1, 18),
                                            kind: BinaryOperatorKind::Assignment,
                                        },
                                        right: Box::new(Expression {
                                            span: Span::new("while_statement", 19, 24, 1, 20),
                                            kind: ExpressionKind::BinaryOperation {
                                                left: Box::new(Expression {
                                                    span: Span::new(
                                                        "while_statement",
                                                        19,
                                                        20,
                                                        1,
                                                        20
                                                    ),
                                                    kind: ExpressionKind::Variable { name: "i" },
                                                }),
                                                operator: BinaryOperator {
                                                    span: Span::new(
                                                        "while_statement",
                                                        21,
                                                        22,
                                                        1,
                                                        22
                                                    ),
                                                    kind: BinaryOperatorKind::Addition,
                                                },
                                                right: Box::new(Expression {
                                                    span: Span::new(
                                                        "while_statement",
                                                        23,
                                                        24,
                                                        1,
                                                        24
                                                    ),
                                                    kind: ExpressionKind::IntegerLiteral {
                                                        value: "1",
                                                        radix: 10,
                                                    },
                                                }),
                                            },
                                        }),
                                    },
                                },
                            },
                        }],
                    },
                }),
            },
        })
    );
}

#[test]
fn return_statement_integer() {
    let source = "return 42;";
    let mut parser = Parser::new(Lexer::new("return_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("return_statement", 0, 10, 1, 1),
            kind: StatementKind::Return {
                value: Some(Expression {
                    span: Span::new("return_statement", 7, 9, 1, 8),
                    kind: ExpressionKind::IntegerLiteral {
                        value: "42",
                        radix: 10
                    },
                },),
            },
        })
    );
}

#[test]
fn return_statement_expression() {
    let source = "return 42 + 42;";
    let mut parser = Parser::new(Lexer::new("return_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("return_statement", 0, 15, 1, 1),
            kind: StatementKind::Return {
                value: Some(Expression {
                    span: Span::new("return_statement", 7, 14, 1, 8),
                    kind: ExpressionKind::BinaryOperation {
                        left: Box::new(Expression {
                            span: Span::new("return_statement", 7, 9, 1, 8),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "42",
                                radix: 10
                            },
                        }),
                        operator: BinaryOperator {
                            span: Span::new("return_statement", 10, 11, 1, 11),
                            kind: BinaryOperatorKind::Addition,
                        },
                        right: Box::new(Expression {
                            span: Span::new("return_statement", 12, 14, 1, 13),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "42",
                                radix: 10
                            },
                        }),
                    },
                },),
            },
        })
    );
}

#[test]
fn return_statement_no_expression() {
    let source = "return;";
    let mut parser = Parser::new(Lexer::new("return_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("return_statement", 0, 7, 1, 1),
            kind: StatementKind::Return { value: None },
        })
    );
}

#[test]
fn break_statement() {
    let source = "break;";
    let mut parser = Parser::new(Lexer::new("break_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("break_statement", 0, 6, 1, 1),
            kind: StatementKind::Break,
        })
    );
}

#[test]
fn continue_statement() {
    let source = "continue;";
    let mut parser = Parser::new(Lexer::new("continue_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("continue_statement", 0, 9, 1, 1),
            kind: StatementKind::Continue,
        })
    );
}

#[test]
fn block_statement() {
    let source = "{ let x = 42; }";
    let mut parser = Parser::new(Lexer::new("block_statement", source));

    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            span: Span::new("block_statement", 0, 15, 1, 1),
            kind: StatementKind::Block {
                statements: vec![Statement {
                    span: Span::new("block_statement", 2, 13, 1, 3),
                    kind: StatementKind::VariableDeclaration {
                        name: "x",
                        ty: None,
                        value: Expression {
                            span: Span::new("block_statement", 10, 12, 1, 11),
                            kind: ExpressionKind::IntegerLiteral {
                                value: "42",
                                radix: 10
                            },
                        },
                    },
                }],
            },
        })
    );
}
