use crate::Parser;
use tantalum_syntax::{SyntaxKind, SyntaxToken};

pub fn identifier(p: &mut Parser) -> Option<SyntaxToken> {
    p.consume_if(SyntaxKind::Identifier)
}

#[test]
fn basic_identifier() {
    use tantalum_syntax::pretty;
    crate::setup_parser!(
        files with parser => {
            basic_identifier => "hello",
        }
    );

    let ident = identifier(&mut parser);

    assert!(ident.is_some());

    let ident = ident.unwrap();

    pretty!(result = ident files);

    insta::assert_snapshot!(result, @r#"Identifier @ #0 0..5 "hello""#);
}
