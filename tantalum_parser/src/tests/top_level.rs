use tantalum_ast::{
    Parameter, ParameterKind, Statement, StatementKind, TopLevelExpression, TopLevelExpressionKind,
    Type, TypeKind,
};
use tantalum_lexer::Lexer;
use tantalum_span::Span;

use pretty_assertions::assert_eq;

use crate::Parser;

#[test]
fn function_declaration() {
    let source = r"fn main() {}";
    let mut parser = Parser::new(Lexer::new("function_declaration", source));

    let result = parser.parse_top_level();

    assert_eq!(
        result,
        Ok(TopLevelExpression {
            span: Span::new("function_declaration", 0, 12, 1, 1),
            kind: TopLevelExpressionKind::FunctionDeclaration {
                name: "main",
                parameters: Vec::new(),
                return_type: Type {
                    span: Span::new("function_declaration", 8, 9, 1, 9),
                    kind: TypeKind::Named("void")
                },
                body: Statement {
                    span: Span::new("function_declaration", 10, 12, 1, 11),
                    kind: StatementKind::Block {
                        statements: Vec::new()
                    }
                }
            }
        })
    );
}

#[test]
fn function_declaration_with_parameters() {
    let source = r"fn main(argc: i32, argv: [*const u8]) {}";
    let mut parser = Parser::new(Lexer::new("function_declaration_with_parameters", source));

    let result = parser.parse_top_level();

    assert_eq!(
        result,
        Ok(TopLevelExpression {
            span: Span::new("function_declaration_with_parameters", 0, 40, 1, 1),
            kind: TopLevelExpressionKind::FunctionDeclaration {
                name: "main",
                parameters: vec![
                    Parameter {
                        span: Span::new("function_declaration_with_parameters", 8, 17, 1, 9),
                        kind: ParameterKind::Named {
                            name: "argc",
                            ty: Type {
                                span: Span::new(
                                    "function_declaration_with_parameters",
                                    14,
                                    17,
                                    1,
                                    15
                                ),
                                kind: TypeKind::Named("i32")
                            }
                        }
                    },
                    Parameter {
                        span: Span::new("function_declaration_with_parameters", 19, 36, 1, 20),
                        kind: ParameterKind::Named {
                            name: "argv",
                            ty: Type {
                                span: Span::new(
                                    "function_declaration_with_parameters",
                                    25,
                                    36,
                                    1,
                                    26
                                ),
                                kind: TypeKind::UnsizedArray(Box::new(Type {
                                    span: Span::new(
                                        "function_declaration_with_parameters",
                                        26,
                                        35,
                                        1,
                                        27
                                    ),
                                    kind: TypeKind::Pointer(Box::new(Type {
                                        span: Span::new(
                                            "function_declaration_with_parameters",
                                            27,
                                            35,
                                            1,
                                            28
                                        ),
                                        kind: TypeKind::Const(Box::new(Type {
                                            span: Span::new(
                                                "function_declaration_with_parameters",
                                                33,
                                                35,
                                                1,
                                                34
                                            ),
                                            kind: TypeKind::Named("u8")
                                        }))
                                    }))
                                }))
                            }
                        }
                    }
                ],
                return_type: Type {
                    span: Span::new("function_declaration_with_parameters", 36, 37, 1, 37),
                    kind: TypeKind::Named("void")
                },
                body: Statement {
                    span: Span::new("function_declaration_with_parameters", 38, 40, 1, 39),
                    kind: StatementKind::Block {
                        statements: Vec::new()
                    }
                }
            }
        })
    );
}

#[test]
fn function_declaration_with_return_type() {
    let source = r"fn main(): i32 {}";
    let mut parser = Parser::new(Lexer::new("function_declaration_with_return_type", source));

    let result = parser.parse_top_level();

    assert_eq!(
        result,
        Ok(TopLevelExpression {
            span: Span::new("function_declaration_with_return_type", 0, 17, 1, 1),
            kind: TopLevelExpressionKind::FunctionDeclaration {
                name: "main",
                parameters: Vec::new(),
                return_type: Type {
                    span: Span::new("function_declaration_with_return_type", 11, 14, 1, 12),
                    kind: TypeKind::Named("i32")
                },
                body: Statement {
                    span: Span::new("function_declaration_with_return_type", 15, 17, 1, 16),
                    kind: StatementKind::Block {
                        statements: Vec::new()
                    }
                }
            }
        })
    );
}

#[test]
fn external_function_declaration() {
    let source = r"extern fn puts(s: *const u8): i32;";
    let mut parser = Parser::new(Lexer::new("external_function_declaration", source));

    let result = parser.parse_top_level();

    assert_eq!(
        result,
        Ok(TopLevelExpression {
            span: Span::new("external_function_declaration", 0, 34, 1, 1),
            kind: TopLevelExpressionKind::ExternalFunction {
                name: "puts",
                parameters: vec![Parameter {
                    span: Span::new("external_function_declaration", 15, 27, 1, 16),
                    kind: ParameterKind::Named {
                        name: "s",
                        ty: Type {
                            span: Span::new("external_function_declaration", 18, 27, 1, 19),
                            kind: TypeKind::Pointer(Box::new(Type {
                                span: Span::new("external_function_declaration", 19, 27, 1, 20),
                                kind: TypeKind::Const(Box::new(Type {
                                    span: Span::new("external_function_declaration", 25, 27, 1, 26),
                                    kind: TypeKind::Named("u8")
                                }))
                            }))
                        }
                    }
                }],
                return_type: Type {
                    span: Span::new("external_function_declaration", 30, 33, 1, 31),
                    kind: TypeKind::Named("i32")
                },
                is_variadic: false
            }
        })
    );
}

#[test]
fn external_function_declaration_with_variadic() {
    let source = r"extern fn printf(format: *const u8, ...): i32;";
    let mut parser = Parser::new(Lexer::new(
        "external_function_declaration_with_variadic",
        source,
    ));

    let result = parser.parse_top_level();

    assert_eq!(
        result,
        Ok(TopLevelExpression {
            span: Span::new("external_function_declaration_with_variadic", 0, 46, 1, 1),
            kind: TopLevelExpressionKind::ExternalFunction {
                name: "printf",
                parameters: vec![Parameter {
                    span: Span::new("external_function_declaration_with_variadic", 17, 34, 1, 18),
                    kind: ParameterKind::Named {
                        name: "format",
                        ty: Type {
                            span: Span::new(
                                "external_function_declaration_with_variadic",
                                25,
                                34,
                                1,
                                26
                            ),
                            kind: TypeKind::Pointer(Box::new(Type {
                                span: Span::new(
                                    "external_function_declaration_with_variadic",
                                    26,
                                    34,
                                    1,
                                    27
                                ),
                                kind: TypeKind::Const(Box::new(Type {
                                    span: Span::new(
                                        "external_function_declaration_with_variadic",
                                        32,
                                        34,
                                        1,
                                        33
                                    ),
                                    kind: TypeKind::Named("u8")
                                }))
                            }))
                        }
                    }
                }],
                return_type: Type {
                    span: Span::new("external_function_declaration_with_variadic", 42, 45, 1, 43),
                    kind: TypeKind::Named("i32")
                },
                is_variadic: true
            }
        })
    );
}
