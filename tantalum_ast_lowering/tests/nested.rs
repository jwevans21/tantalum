use tantalum_ast_lowering::ASTLoweringContext;

// const SOURCE: &str = r"
// extern fn printf(format: *u8, ...): i32;
//
// fn main(argc: i32, argv: [*u8]): i32 {
//     let x: i32 = argc;
//     let y: i32 = 42;
//     let z: i32 = x + y;
//
//     {
//         let z: i32 = 42;
//     }
//
//     let z: i32 = z + 1;
//
//     return z;
// }
// ";

const SOURCE: &str = r#"
extern fn printf(format: *u8, ...): i32;

extern fn print(data: str): i32;

fn add(a: i32, b: i32): i32 {
    return a + b;
}

fn main(argc: i32, argv: [*u8]): i32 {
    let a = 1;
    let b: i32 = 2;
    
    {
        let a = 3;
        let b: f32 = a;
    }
    
    if argc == 1 {
        let a = 4;
    }
    
    let c = 1.0;
    let d = c + 1:f64;

    let e = add(a, b);

    print("Hello, world!");

    return a;
}
"#;

#[test]
fn basic_assertions() {
    let mut ast =
        tantalum_parser::Parser::new(tantalum_lexer::Lexer::new("basic_assertions", SOURCE));

    let ast = match ast.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{err}");
            panic!();
        }
    };

    let mut context = ASTLoweringContext::new();

    context.lower(&ast);

    let package = context.finish();

    assert!(false);

    // assert_eq!(package.prototypes.len(), 2); // printf and main
    // assert_eq!(package.functions.len(), 1); // main
    // assert_eq!(package.literals.len(), 2); // 42 and 1

    insta::assert_debug_snapshot!(package);
}
