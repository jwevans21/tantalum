use tantalum_lexer::Lexer;

use crate::Parser;

#[test]
fn expression_statement() {
    let source = "42 + 42;";
    let mut parser = Parser::new(Lexer::new("expression_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn let_statement() {
    let source = "let x = 42;";
    let mut parser = Parser::new(Lexer::new("let_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn let_statement_with_type() {
    let source = "let x: *u8 = \"Hello, World!\";";
    let mut parser = Parser::new(Lexer::new("let_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn if_statement() {
    let source = "if true { return 42; }";
    let mut parser = Parser::new(Lexer::new("if_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn if_else_statement() {
    let source = "if true { return 42; } else { return 0; }";
    let mut parser = Parser::new(Lexer::new("if_else_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
#[allow(clippy::too_many_lines)]
fn for_loop_statement() {
    let source = "for let i = 0; i < 10; i = i + 1; { return i; }";
    let mut parser = Parser::new(Lexer::new("for_loop_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn while_statement() {
    let source = "while i < 10 { i = i + 1; }";
    let mut parser = Parser::new(Lexer::new("while_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn return_statement_integer() {
    let source = "return 42;";
    let mut parser = Parser::new(Lexer::new("return_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn return_statement_expression() {
    let source = "return 42 + 42;";
    let mut parser = Parser::new(Lexer::new("return_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn return_statement_no_expression() {
    let source = "return;";
    let mut parser = Parser::new(Lexer::new("return_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn break_statement() {
    let source = "break;";
    let mut parser = Parser::new(Lexer::new("break_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn continue_statement() {
    let source = "continue;";
    let mut parser = Parser::new(Lexer::new("continue_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn block_statement() {
    let source = "{ let x = 42; }";
    let mut parser = Parser::new(Lexer::new("block_statement", source));

    let result = parser.parse_statement();

    insta::assert_ron_snapshot!(result);
}
