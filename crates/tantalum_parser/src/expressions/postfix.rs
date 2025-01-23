use crate::Parser;
use tantalum_source::SourceSpan;
use tantalum_syntax::{SyntaxKind, SyntaxToken};

pub fn postfix_op(p: &mut Parser) -> Option<SyntaxToken> {
    let first = p.peek(1)?;
    let second = p.peek(2);

    match (first.kind(), second.map(|t| t.kind())) {
        (SyntaxKind::Dot, Some(SyntaxKind::Star)) => Some(SyntaxToken::new(
            SyntaxKind::Deref,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),
        (SyntaxKind::Dot, Some(SyntaxKind::Ampersand)) => Some(SyntaxToken::new(
            SyntaxKind::Ref,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),
        _ => None,
    }
}

pub fn postfix_op_bp(kind: SyntaxKind) -> (u8, ()) {
    match kind {
        SyntaxKind::Deref | SyntaxKind::Ref => (1, ()),
        _ => (0, ()),
    }
}

pub fn consume_postfix_op(p: &mut Parser) -> Option<SyntaxToken> {
    let op = postfix_op(p)?;
    match op.kind() {
        SyntaxKind::Deref | SyntaxKind::Ref => {
            p.advance();
            p.advance();
        }
        _ => {
            p.advance();
        }
    }

    p.token(op);

    Some(op)
}

#[test]
fn operators() {
    use crate::whitespace::skip_whitespace;
    use tantalum_syntax::pretty;

    crate::setup_parser!(
        files with parser => {
            operators => ".* .&",
        }
    );

    let open = parser.open(SyntaxKind::File);
    while !parser.at_end() {
        skip_whitespace(&mut parser);
        let _ = consume_postfix_op(&mut parser).expect("expected infix operator");
        skip_whitespace(&mut parser);
    }
    let _ = parser.close(open);

    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    File @ #0 0..5
      Deref @ #0 0..2 ".*"
      Whitespace @ #0 2..3 " "
      Ref @ #0 3..5 ".&"
    "#);
}

#[test]
fn deref() {
    use crate::expressions::expression;
    use tantalum_syntax::pretty;

    crate::setup_parser!(
        files with parser => {
            deref => "identifier.*",
        }
    );

    let _ = expression(&mut parser);

    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    PostfixExpression @ #0 0..12
      Variable @ #0 0..10
        Identifier @ #0 0..10 "identifier"
      Deref @ #0 10..12 ".*"
    "#);
}

#[test]
fn ref_deref() {
    use crate::expressions::expression;
    use tantalum_syntax::pretty;

    crate::setup_parser!(
        files with parser => {
            ref_deref => "identifier.&.*",
        }
    );

    let _ = expression(&mut parser);

    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    PostfixExpression @ #0 0..14
      PostfixExpression @ #0 0..12
        Variable @ #0 0..10
          Identifier @ #0 0..10 "identifier"
        Ref @ #0 10..12 ".&"
      Deref @ #0 12..14 ".*"
    "#);
}
