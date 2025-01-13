macro_rules! snapshot_from_file {
    ($file_name:ident) => {
        #[test]
        fn $file_name() {
            let source = include_str!(concat!("../../examples/", stringify!($file_name), ".ta"));
            let lexer = tantalum_lexer::Lexer::new(concat!(stringify!($file_name), ".ta"), source);

            let mut parser = tantalum_parser::Parser::new(lexer);

            let ast = parser.parse().expect("failed to parse source");

            let mut context = tantalum_ast_lowering::ASTLoweringContext::new();

            context.lower(&ast);

            let package = context.finish();

            insta::assert_debug_snapshot!(package);
        }
    };
    ($($file_name:ident),*) => {
        $(snapshot_from_file! { $file_name })*
    };
}

snapshot_from_file! {
    start,
    hello_world,
    conditionals,
    loops
}
