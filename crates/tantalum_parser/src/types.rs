use crate::{paths::path, MarkClosed, Parser};
use tantalum_syntax::SyntaxKind;

pub fn ty(p: &mut Parser) -> MarkClosed {
    if p.is_at_any(&[SyntaxKind::Identifier, SyntaxKind::Colon]) {
        named_ty(p)
    } else if p.is_at(SyntaxKind::Star) {
        pointer_ty(p)
    } else if p.is_at(SyntaxKind::Ampersand) {
        reference_ty(p)
    } else {
        eprintln!("error: unexpected token, expected type");
        let open = p.open(SyntaxKind::Error);
        let token = p.consume().unwrap();
        p.token(token);
        p.close(open)
    }
}

fn named_ty(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::NamedType);

    let _ = path(p);

    p.close(open)
}

fn pointer_ty(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::PointerType);

    let Some(_) = p.consume_if(SyntaxKind::Star) else {
        eprintln!("error: expected `*`");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };

    let _ = ty(p);

    p.close(open)
}

fn reference_ty(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::ReferenceType);

    let Some(_) = p.consume_if(SyntaxKind::Ampersand) else {
        eprintln!("error: expected `&`");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };

    let _ = ty(p);

    p.close(open)
}

#[test]
fn named_type() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            named_type => "u8",
        }
    );

    let _ = ty(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    NamedType @ #0 0..2
      Path @ #0 0..2
        PathSegment @ #0 0..2
          Identifier @ #0 0..2 "u8"
    "#);
}

#[test]
fn absolute_named_path() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            absolute_named_path => "::u8",
        }
    );

    let _ = ty(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    NamedType @ #0 0..4
      Path @ #0 0..4
        PathSeparator @ #0 0..2 "::"
        PathSegment @ #0 2..4
          Identifier @ #0 2..4 "u8"
    "#);
}

#[test]
fn pointer_type() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            pointer_type => "*u8",
        }
    );

    let _ = ty(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    PointerType @ #0 0..3
      Star @ #0 0..1 "*"
      NamedType @ #0 1..3
        Path @ #0 1..3
          PathSegment @ #0 1..3
            Identifier @ #0 1..3 "u8"
    "#);
}

#[test]
fn pointer_to_pointer_type() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            pointer_to_pointer_type => "**u8",
        }
    );

    let _ = ty(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    PointerType @ #0 0..4
      Star @ #0 0..1 "*"
      PointerType @ #0 1..4
        Star @ #0 1..2 "*"
        NamedType @ #0 2..4
          Path @ #0 2..4
            PathSegment @ #0 2..4
              Identifier @ #0 2..4 "u8"
    "#);
}

#[test]
fn reference_type() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            reference_type => "&u8",
        }
    );

    let _ = ty(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    ReferenceType @ #0 0..3
      Ampersand @ #0 0..1 "&"
      NamedType @ #0 1..3
        Path @ #0 1..3
          PathSegment @ #0 1..3
            Identifier @ #0 1..3 "u8"
    "#);
}
