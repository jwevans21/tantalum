use crate::{identifiers::identifier, MarkClosed, Parser};
use tantalum_source::SourceSpan;
use tantalum_syntax::{SyntaxKind, SyntaxToken};

pub fn path(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::Path);

    // try to parse leading separator
    path_separator(p);

    // parse first segment
    let _ = path_segment(p);

    while path_separator(p) {
        let _ = path_segment(p);
    }

    p.close(open)
}

fn path_separator(p: &mut Parser) -> bool {
    if p.peek(1)
        .is_some_and(|token| token.kind() == SyntaxKind::Colon)
        && p.peek(2)
            .is_some_and(|token| token.kind() == SyntaxKind::Colon)
    {
        let first = p.advance().unwrap();
        let second = p.advance().unwrap();

        p.token(SyntaxToken::new(
            SyntaxKind::PathSeparator,
            SourceSpan::merge(&[first.span(), second.span()]),
        ));

        true
    } else {
        false
    }
}

pub fn path_segment(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::PathSegment);

    let Some(_) = identifier(p) else {
        eprintln!("error: expected identifier");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };

    p.close(open)
}

#[test]
fn absolute_path() {
    crate::setup_parser!(
        files with parser => {
            absolute_path => "::hello::world",
        }
    );

    let _ = path(&mut parser);

    let tree = parser.finish();

    crate::pretty_snapshot!(
        tree with files => r#"
Path @ #0 0..14
  PathSeparator @ #0 0..2 "::"
  PathSegment @ #0 2..7
    Identifier @ #0 2..7 "hello"
  PathSeparator @ #0 7..9 "::"
  PathSegment @ #0 9..14
    Identifier @ #0 9..14 "world"
"#
    );
}
