use crate::Parser;
use tantalum_syntax::{SyntaxKind, SyntaxToken};

pub fn prefix_op(p: &mut Parser) -> Option<SyntaxToken> {
    let first = p.peek(1)?;
    match first.kind() {
        SyntaxKind::Minus | SyntaxKind::Bang => Some(first),
        _ => None,
    }
}

pub fn prefix_op_bp(kind: SyntaxKind) -> ((), u8) {
    match kind {
        SyntaxKind::Minus | SyntaxKind::Bang => ((), 1),
        _ => ((), 0),
    }
}

pub fn consume_prefix_op(p: &mut Parser) -> Option<SyntaxToken> {
    let op = prefix_op(p)?;

    p.advance();

    p.token(op);

    Some(op)
}

#[test]
fn operators() {
    use crate::whitespace::skip_whitespace;
    use tantalum_syntax::pretty;

    crate::setup_parser!(
        files with parser => {
            operators => "- !",
        }
    );

    let open = parser.open(SyntaxKind::File);
    while !parser.at_end() {
        skip_whitespace(&mut parser);
        let _ = consume_prefix_op(&mut parser);
        skip_whitespace(&mut parser);
    }
    let _ = parser.close(open);

    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    File @ #0 0..3
      Minus @ #0 0..1 "-"
      Whitespace @ #0 1..2 " "
      Bang @ #0 2..3 "!"
    "#);
}

#[test]
fn minus_bang() {
    use crate::expressions::expression;
    use tantalum_syntax::pretty;

    crate::setup_parser!(
        files with parser => {
            operators => "- ! 5",
        }
    );

    expression(&mut parser);

    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    PrefixExpression @ #0 0..5
      Minus @ #0 0..1 "-"
      Whitespace @ #0 1..2 " "
      PrefixExpression @ #0 2..5
        Bang @ #0 2..3 "!"
        Whitespace @ #0 3..4 " "
        Literal @ #0 4..5
          IntegerLiteral @ #0 4..5 "5"
    "#);
}

#[test]
fn bang_minus() {
    use crate::expressions::expression;
    use tantalum_syntax::pretty;

    crate::setup_parser!(
        files with parser => {
            operators => "! - 5",
        }
    );

    expression(&mut parser);

    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    PrefixExpression @ #0 0..5
      Bang @ #0 0..1 "!"
      Whitespace @ #0 1..2 " "
      PrefixExpression @ #0 2..5
        Minus @ #0 2..3 "-"
        Whitespace @ #0 3..4 " "
        Literal @ #0 4..5
          IntegerLiteral @ #0 4..5 "5"
    "#);
}
