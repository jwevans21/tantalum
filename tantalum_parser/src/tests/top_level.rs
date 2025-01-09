use tantalum_lexer::Lexer;

use crate::Parser;

#[test]
fn function_declaration() {
    let source = r"fn main() {}";
    let mut parser = Parser::new(Lexer::new("function_declaration", source));

    let result = parser.parse_top_level();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn function_declaration_with_parameters() {
    let source = r"fn main(argc: i32, argv: [*const u8]) {}";
    let mut parser = Parser::new(Lexer::new("function_declaration_with_parameters", source));

    let result = parser.parse_top_level();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn function_declaration_with_return_type() {
    let source = r"fn main(): i32 {}";
    let mut parser = Parser::new(Lexer::new("function_declaration_with_return_type", source));

    let result = parser.parse_top_level();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn external_function_declaration() {
    let source = r"extern fn puts(s: *const u8): i32;";
    let mut parser = Parser::new(Lexer::new("external_function_declaration", source));

    let result = parser.parse_top_level();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn external_function_declaration_with_variadic() {
    let source = r"extern fn printf(format: *const u8, ...): i32;";
    let mut parser = Parser::new(Lexer::new(
        "external_function_declaration_with_variadic",
        source,
    ));

    let result = parser.parse_top_level();

    insta::assert_ron_snapshot!(result);
}
