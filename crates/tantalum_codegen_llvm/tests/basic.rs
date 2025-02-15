use inkwell::context::Context;
use tantalum_ast_lowering::ASTLoweringContext;
use tantalum_codegen_llvm::LLVMCodegenContext;
use tantalum_lexer::Lexer;
use tantalum_parser::Parser;

const SOURCE: &str = r#"
fn add(a: i32, b: i32): i32 {
    return a + b;
}

fn main(): i32 {
    let c = 30;
    return add(c, 2);
}
"#;

#[test]
fn basic() {
    let lexer = Lexer::new("basic.ta", SOURCE);
    let mut parser = Parser::new(lexer);

    let ast = match parser.parse() {
        Err(err) => panic!("{}", err),
        Ok(ast) => ast,
    };

    let mut lowering_context = ASTLoweringContext::new();
    lowering_context.lower(&ast);

    let package = lowering_context.finish();

    let context = Context::create();
    let mut codegen_context = LLVMCodegenContext::new(&context);

    codegen_context.build(package);

    eprintln!("{:#?}", codegen_context);

    eprintln!("{}", codegen_context.dump_to_string());

    match codegen_context.verify() {
        Ok(_) => {}
        Err(err) => panic!("{}", err),
    }

    match codegen_context.compile("basic.s") {
        Ok(_) => {}
        Err(err) => panic!("{}", err),
    }

    assert!(false);
}
