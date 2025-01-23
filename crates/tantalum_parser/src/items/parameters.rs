use crate::{identifiers::identifier, types::ty, whitespace::skip_whitespace, MarkClosed, Parser};
use tantalum_source::SourceSpan;
use tantalum_syntax::{SyntaxKind, SyntaxToken};

pub fn parameter_list(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::ParameterList);

    skip_whitespace(p);

    while !p.is_at(SyntaxKind::RParen) {
        skip_whitespace(p);

        if p.is_at(SyntaxKind::Dot) {
            if let (Some(SyntaxKind::Dot), Some(SyntaxKind::Dot), Some(SyntaxKind::Dot)) = (
                p.peek(1).map(|t| t.kind()),
                p.peek(2).map(|t| t.kind()),
                p.peek(3).map(|t| t.kind()),
            ) {
                // advance past the '...'
                let first = p.advance().unwrap();
                p.advance();
                let last = p.advance().unwrap();

                p.token(SyntaxToken::new(
                    SyntaxKind::Variadic,
                    SourceSpan::merge(&[first.span(), last.span()]),
                ));
                break;
            }

            eprintln!("error: expected '...'");
            p.adjust(open, SyntaxKind::Error);
            return p.close(open);
        }

        let _ = parameter(p);
        skip_whitespace(p);

        if p.consume_if(SyntaxKind::Comma).is_none() {
            break;
        }
    }

    p.close(open)
}

fn parameter(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::Parameter);

    let Some(_) = identifier(p) else {
        eprintln!("error: expected identifier for parameter");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };

    skip_whitespace(p);

    let Some(_) = p.consume_if(SyntaxKind::Colon) else {
        eprintln!("error: expected ':' for parameter type");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };

    skip_whitespace(p);

    let _ = ty(p);

    p.close(open)
}

#[test]
fn basic_parameter_list() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            basic_parameter_list => "a: i32, b: *i32, ...",
        }
    );

    let _ = parameter_list(&mut parser);
    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    ParameterList @ #0 0..20
      Parameter @ #0 0..6
        Identifier @ #0 0..1 "a"
        Colon @ #0 1..2 ":"
        Whitespace @ #0 2..3 " "
        NamedType @ #0 3..6
          Path @ #0 3..6
            PathSegment @ #0 3..6
              Identifier @ #0 3..6 "i32"
      Comma @ #0 6..7 ","
      Whitespace @ #0 7..8 " "
      Parameter @ #0 8..15
        Identifier @ #0 8..9 "b"
        Colon @ #0 9..10 ":"
        Whitespace @ #0 10..11 " "
        PointerType @ #0 11..15
          Star @ #0 11..12 "*"
          NamedType @ #0 12..15
            Path @ #0 12..15
              PathSegment @ #0 12..15
                Identifier @ #0 12..15 "i32"
      Comma @ #0 15..16 ","
      Whitespace @ #0 16..17 " "
      Variadic @ #0 17..20 "..."
    "#);
}
