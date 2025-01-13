use tantalum_ast_lowering::ASTLoweringContext;

const SOURCE: &str = r#"
extern fn printf(format: *u8, ...): i32;

fn main(argc: i32, argv: [*u8]): i32 {
    let x: i32 = argc;
    let y: i32 = 42;
    let z: i32 = x + y;

    {
        let z: i32 = 42;
    }

    let z: i32 = z + 1;

    return z;
}
"#;

#[test]
fn basic_assertions() {
    let mut ast =
        tantalum_parser::Parser::new(tantalum_lexer::Lexer::new("basic_assertions", SOURCE));

    let ast = match ast.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{:#?}", err);
            panic!();
        }
    };

    let mut context = ASTLoweringContext::new();

    context.lower(&ast);

    let package = context.finish();

    assert_eq!(package.prototypes.len(), 2); // printf and main
    assert_eq!(package.functions.len(), 1); // main
    assert_eq!(package.literals.len(), 2); // 42 and 1

    insta::assert_debug_snapshot!(package);
}
