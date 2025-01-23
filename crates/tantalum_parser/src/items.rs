use crate::{
    expressions::block, identifiers::identifier, items::parameters::parameter_list, types::ty,
    whitespace::skip_whitespace, MarkClosed, Parser,
};
use tantalum_syntax::SyntaxKind;

mod parameters;

pub fn file(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::File);
    skip_whitespace(p);

    while !p.at_end() {
        if p.is_at(SyntaxKind::Fn) {
            function(p);
        } else {
            eprintln!("error: unexpected token");
            let open = p.open(SyntaxKind::Error);
            let token = p.consume().unwrap();
            p.token(token);
            let _ = p.close(open);
        }

        skip_whitespace(p);
    }

    p.close(open)
}

pub fn function(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::Function);
    let Some(_) = p.consume_if(SyntaxKind::Fn) else {
        eprintln!("error: expected `fn`");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };
    skip_whitespace(p);

    let Some(_) = identifier(p) else {
        eprintln!("error: expected identifier");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };
    skip_whitespace(p);

    let Some(_) = p.consume_if(SyntaxKind::LParen) else {
        eprintln!("error: expected '('");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };

    let _ = parameter_list(p);

    let Some(_) = p.consume_if(SyntaxKind::RParen) else {
        eprintln!("error: expected ')'");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };
    skip_whitespace(p);

    if p.consume_if(SyntaxKind::Colon).is_some() {
        skip_whitespace(p);
        let _ = ty(p);
    }

    skip_whitespace(p);

    let _ = block(p);

    p.close(open)
}

#[test]
fn basic_whitespace_file() {
    crate::setup_parser!(
        files with parser => {
            basic_whitespace_file => " \t\n",
        }
    );

    let _ = file(&mut parser);

    let tree = parser.finish();

    crate::pretty_snapshot!(
        tree with files => r#"
File @ #0 0..3
  Whitespace @ #0 0..3 " \t\n"
"#
    );
}

#[test]
fn basic_function() {
    crate::setup_parser!(
        files with parser => {
            basic_function => "fn main() {}",
        }
    );

    let _ = file(&mut parser);

    let tree = parser.finish();

    crate::pretty_snapshot!(
        tree with files => r#"
File @ #0 0..12
  Function @ #0 0..12
    Fn @ #0 0..2 "fn"
    Whitespace @ #0 2..3 " "
    Identifier @ #0 3..7 "main"
    LParen @ #0 7..8 "("
    ParameterList @ #0 8..8
    RParen @ #0 8..9 ")"
    Whitespace @ #0 9..10 " "
    Block @ #0 10..12
      LBrace @ #0 10..11 "{"
      RBrace @ #0 11..12 "}"
"#);
}

#[test]
fn function_with_parameters() {
    crate::setup_parser!(
        files with parser => {
            function_with_parameters => "fn add(a: i32, b: i32) {}",
        }
    );

    let _ = file(&mut parser);

    let tree = parser.finish();

    crate::pretty_snapshot!(
        tree with files => r#"
File @ #0 0..25
  Function @ #0 0..25
    Fn @ #0 0..2 "fn"
    Whitespace @ #0 2..3 " "
    Identifier @ #0 3..6 "add"
    LParen @ #0 6..7 "("
    ParameterList @ #0 7..21
      Parameter @ #0 7..13
        Identifier @ #0 7..8 "a"
        Colon @ #0 8..9 ":"
        Whitespace @ #0 9..10 " "
        NamedType @ #0 10..13
          Path @ #0 10..13
            PathSegment @ #0 10..13
              Identifier @ #0 10..13 "i32"
      Comma @ #0 13..14 ","
      Whitespace @ #0 14..15 " "
      Parameter @ #0 15..21
        Identifier @ #0 15..16 "b"
        Colon @ #0 16..17 ":"
        Whitespace @ #0 17..18 " "
        NamedType @ #0 18..21
          Path @ #0 18..21
            PathSegment @ #0 18..21
              Identifier @ #0 18..21 "i32"
    RParen @ #0 21..22 ")"
    Whitespace @ #0 22..23 " "
    Block @ #0 23..25
      LBrace @ #0 23..24 "{"
      RBrace @ #0 24..25 "}"
"#);
}

#[test]
fn function_with_return_type() {
    crate::setup_parser!(
        files with parser => {
            function_with_return_type => "fn main(): i32 {}",
        }
    );

    let _ = file(&mut parser);

    let tree = parser.finish();

    crate::pretty_snapshot!(
        tree with files => r#"
File @ #0 0..17
  Function @ #0 0..17
    Fn @ #0 0..2 "fn"
    Whitespace @ #0 2..3 " "
    Identifier @ #0 3..7 "main"
    LParen @ #0 7..8 "("
    ParameterList @ #0 8..8
    RParen @ #0 8..9 ")"
    Colon @ #0 9..10 ":"
    Whitespace @ #0 10..11 " "
    NamedType @ #0 11..14
      Path @ #0 11..14
        PathSegment @ #0 11..14
          Identifier @ #0 11..14 "i32"
    Whitespace @ #0 14..15 " "
    Block @ #0 15..17
      LBrace @ #0 15..16 "{"
      RBrace @ #0 16..17 "}"
"#);
}
