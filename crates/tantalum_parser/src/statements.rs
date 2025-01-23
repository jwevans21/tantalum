use crate::{
    expressions::expression, identifiers::identifier, types::ty, whitespace::skip_whitespace,
    MarkClosed, Parser,
};
use tantalum_syntax::SyntaxKind;

pub fn statement(p: &mut Parser) -> MarkClosed {
    if p.is_at(SyntaxKind::Let) {
        let_statement(p)
    } else if p.is_at(SyntaxKind::Return) {
        return_statement(p)
    } else {
        let expr = expression(p);

        if p.is_at(SyntaxKind::Semicolon) {
            let open = p.open_before(expr, SyntaxKind::ExpressionStatement);

            let _ = p.consume(); // consume semicolon

            p.close(open)
        } else {
            expr
        }
    }
}

fn let_statement(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::LetStatement);

    if p.consume_if(SyntaxKind::Let).is_none() {
        eprintln!("error: expected `let`");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    }
    skip_whitespace(p);

    if identifier(p).is_none() {
        eprintln!("error: expected identifier");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    }

    if p.is_at(SyntaxKind::Colon) {
        let _ = p.consume(); // consume colon
        skip_whitespace(p);
        let _ = ty(p);
    }
    skip_whitespace(p);

    if p.consume_if(SyntaxKind::Equals).is_none() {
        eprintln!("error: expected `=`");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    }
    skip_whitespace(p);

    let _ = expression(p);
    skip_whitespace(p);

    if p.consume_if(SyntaxKind::Semicolon).is_none() {
        eprintln!("error: expected `;`");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    }

    p.close(open)
}

fn return_statement(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::ReturnStatement);

    if p.consume_if(SyntaxKind::Return).is_none() {
        eprintln!("error: expected `return`");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    }
    skip_whitespace(p);

    if p.is_at(SyntaxKind::Semicolon) {
        let _ = p.consume();
        return p.close(open);
    }

    let _ = expression(p);
    skip_whitespace(p);

    if p.consume_if(SyntaxKind::Semicolon).is_none() {
        eprintln!("error: expected `;`");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    }

    p.close(open)
}

#[test]
fn basic_let_statement() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            basic_let_statement => "let x = 5;",
        }
    );

    let _ = statement(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    LetStatement @ #0 0..10
      Let @ #0 0..3 "let"
      Whitespace @ #0 3..4 " "
      Identifier @ #0 4..5 "x"
      Whitespace @ #0 5..6 " "
      Equals @ #0 6..7 "="
      Whitespace @ #0 7..8 " "
      Literal @ #0 8..9
        IntegerLiteral @ #0 8..9 "5"
      Semicolon @ #0 9..10 ";"
    "#);
}

#[test]
fn let_statement_with_type() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            let_statement_with_type => "let x: i32 = 5;",
        }
    );

    let _ = statement(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    LetStatement @ #0 0..15
      Let @ #0 0..3 "let"
      Whitespace @ #0 3..4 " "
      Identifier @ #0 4..5 "x"
      Colon @ #0 5..6 ":"
      Whitespace @ #0 6..7 " "
      NamedType @ #0 7..10
        Path @ #0 7..10
          PathSegment @ #0 7..10
            Identifier @ #0 7..10 "i32"
      Whitespace @ #0 10..11 " "
      Equals @ #0 11..12 "="
      Whitespace @ #0 12..13 " "
      Literal @ #0 13..14
        IntegerLiteral @ #0 13..14 "5"
      Semicolon @ #0 14..15 ";"
    "#);
}

#[test]
fn return_expression() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            return_expression => "return 5;",
        }
    );

    let _ = statement(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    ReturnStatement @ #0 0..9
      Return @ #0 0..6 "return"
      Whitespace @ #0 6..7 " "
      Literal @ #0 7..8
        IntegerLiteral @ #0 7..8 "5"
      Semicolon @ #0 8..9 ";"
    "#);
}

#[test]
fn return_no_expression() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            return_no_expression => "return;",
        }
    );

    let _ = statement(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    ReturnStatement @ #0 0..7
      Return @ #0 0..6 "return"
      Semicolon @ #0 6..7 ";"
    "#);
}

#[test]
fn terminated_expression() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            terminated_expression => "5 + 5 ;",
        }
    );

    let _ = statement(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    ExpressionStatement @ #0 0..7
      BinaryExpression @ #0 0..6
        Literal @ #0 0..1
          IntegerLiteral @ #0 0..1 "5"
        Whitespace @ #0 1..2 " "
        Plus @ #0 2..3 "+"
        Whitespace @ #0 3..4 " "
        Literal @ #0 4..5
          IntegerLiteral @ #0 4..5 "5"
        Whitespace @ #0 5..6 " "
      Semicolon @ #0 6..7 ";"
    "#);
}
