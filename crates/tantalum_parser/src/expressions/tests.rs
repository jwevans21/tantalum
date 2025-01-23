use super::*;
use tantalum_syntax::pretty;

#[test]
fn block_expression() {
    crate::setup_parser!(
        files with parser => {
            block_expression => "{ hello }",
        }
    );

    let _ = block(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    Block @ #0 0..9
      LBrace @ #0 0..1 "{"
      Whitespace @ #0 1..2 " "
      Variable @ #0 2..7
        Identifier @ #0 2..7 "hello"
      Whitespace @ #0 7..8 " "
      RBrace @ #0 8..9 "}"
    "#);
}

#[test]
fn block_expression_with_statements() {
    crate::setup_parser!(
        files with parser => {
            block_expression_with_statements => "{ let x = 5; }",
        }
    );

    let _ = block(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    Block @ #0 0..14
      LBrace @ #0 0..1 "{"
      Whitespace @ #0 1..2 " "
      LetStatement @ #0 2..12
        Let @ #0 2..5 "let"
        Whitespace @ #0 5..6 " "
        Identifier @ #0 6..7 "x"
        Whitespace @ #0 7..8 " "
        Equals @ #0 8..9 "="
        Whitespace @ #0 9..10 " "
        Literal @ #0 10..11
          IntegerLiteral @ #0 10..11 "5"
        Semicolon @ #0 11..12 ";"
      Whitespace @ #0 12..13 " "
      RBrace @ #0 13..14 "}"
    "#);
}

#[test]
fn if_expression() {
    crate::setup_parser!(
        files with parser => {
            if_expression => "if true {}",
        }
    );

    let _ = expression(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    IfExpression @ #0 0..10
      If @ #0 0..2 "if"
      Whitespace @ #0 2..3 " "
      Condition @ #0 3..8
        Literal @ #0 3..7
          True @ #0 3..7 "true"
        Whitespace @ #0 7..8 " "
      Block @ #0 8..10
        LBrace @ #0 8..9 "{"
        RBrace @ #0 9..10 "}"
    "#);
}

#[test]
fn if_else_expression() {
    crate::setup_parser!(
        files with parser => {
            if_else_expression => "if true {} else {}",
        }
    );

    let _ = expression(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    IfExpression @ #0 0..18
      If @ #0 0..2 "if"
      Whitespace @ #0 2..3 " "
      Condition @ #0 3..8
        Literal @ #0 3..7
          True @ #0 3..7 "true"
        Whitespace @ #0 7..8 " "
      Block @ #0 8..10
        LBrace @ #0 8..9 "{"
        RBrace @ #0 9..10 "}"
      Whitespace @ #0 10..11 " "
      Else @ #0 11..15 "else"
      Whitespace @ #0 15..16 " "
      Block @ #0 16..18
        LBrace @ #0 16..17 "{"
        RBrace @ #0 17..18 "}"
    "#);
}

#[test]
fn if_else_if_expression() {
    crate::setup_parser!(
        files with parser => {
            if_else_if_expression => "if true {} else if false {}",
        }
    );

    let _ = expression(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    IfExpression @ #0 0..27
      If @ #0 0..2 "if"
      Whitespace @ #0 2..3 " "
      Condition @ #0 3..8
        Literal @ #0 3..7
          True @ #0 3..7 "true"
        Whitespace @ #0 7..8 " "
      Block @ #0 8..10
        LBrace @ #0 8..9 "{"
        RBrace @ #0 9..10 "}"
      Whitespace @ #0 10..11 " "
      Else @ #0 11..15 "else"
      Whitespace @ #0 15..16 " "
      IfExpression @ #0 16..27
        If @ #0 16..18 "if"
        Whitespace @ #0 18..19 " "
        Condition @ #0 19..25
          Literal @ #0 19..24
            False @ #0 19..24 "false"
          Whitespace @ #0 24..25 " "
        Block @ #0 25..27
          LBrace @ #0 25..26 "{"
          RBrace @ #0 26..27 "}"
    "#);
}

#[test]
fn test_binary_expression() {
    crate::setup_parser!(
        files with parser => {
            binary_expression => "1 + 2",
        }
    );

    let _ = expression(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    BinaryExpression @ #0 0..5
      Literal @ #0 0..1
        IntegerLiteral @ #0 0..1 "1"
      Whitespace @ #0 1..2 " "
      Plus @ #0 2..3 "+"
      Whitespace @ #0 3..4 " "
      Literal @ #0 4..5
        IntegerLiteral @ #0 4..5 "2"
    "#);
}

#[test]
fn binary_expression_add_mul() {
    crate::setup_parser!(
        files with parser => {
            binary_expression_add_mul => "1 + 2 * 3",
        }
    );

    let _ = expression(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    BinaryExpression @ #0 0..9
      Literal @ #0 0..1
        IntegerLiteral @ #0 0..1 "1"
      Whitespace @ #0 1..2 " "
      Plus @ #0 2..3 "+"
      Whitespace @ #0 3..4 " "
      BinaryExpression @ #0 4..9
        Literal @ #0 4..5
          IntegerLiteral @ #0 4..5 "2"
        Whitespace @ #0 5..6 " "
        Star @ #0 6..7 "*"
        Whitespace @ #0 7..8 " "
        Literal @ #0 8..9
          IntegerLiteral @ #0 8..9 "3"
    "#);
}

#[test]
fn binary_expression_mul_add() {
    crate::setup_parser!(
        files with parser => {
            binary_expression_mul_add => "1 * 2 + 3",
        }
    );

    let _ = expression(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    BinaryExpression @ #0 0..9
      BinaryExpression @ #0 0..6
        Literal @ #0 0..1
          IntegerLiteral @ #0 0..1 "1"
        Whitespace @ #0 1..2 " "
        Star @ #0 2..3 "*"
        Whitespace @ #0 3..4 " "
        Literal @ #0 4..5
          IntegerLiteral @ #0 4..5 "2"
        Whitespace @ #0 5..6 " "
      Plus @ #0 6..7 "+"
      Whitespace @ #0 7..8 " "
      Literal @ #0 8..9
        IntegerLiteral @ #0 8..9 "3"
    "#);
}

#[test]
fn binary_expression_parentheses() {
    crate::setup_parser!(
        files with parser => {
            binary_expression_parentheses => "(1 + 2) * 3",
        }
    );

    let _ = expression(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    BinaryExpression @ #0 0..11
      Grouping @ #0 0..7
        LParen @ #0 0..1 "("
        BinaryExpression @ #0 1..6
          Literal @ #0 1..2
            IntegerLiteral @ #0 1..2 "1"
          Whitespace @ #0 2..3 " "
          Plus @ #0 3..4 "+"
          Whitespace @ #0 4..5 " "
          Literal @ #0 5..6
            IntegerLiteral @ #0 5..6 "2"
        RParen @ #0 6..7 ")"
      Whitespace @ #0 7..8 " "
      Star @ #0 8..9 "*"
      Whitespace @ #0 9..10 " "
      Literal @ #0 10..11
        IntegerLiteral @ #0 10..11 "3"
    "#);
}
