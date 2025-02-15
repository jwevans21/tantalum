use insta::assert_ron_snapshot;

use tantalum_lexer::Lexer;
use tantalum_parser::Parser;

macro_rules! snapshot_from_file {
    ($file_name:ident) => {
        #[test]
        fn $file_name() {
            let source = include_str!(concat!("../../examples/", stringify!($file_name), ".ta"));
            let lexer = Lexer::new(concat!(stringify!($file_name), ".ta"), source);

            let mut parser = Parser::new(lexer);

            let ast = parser.parse();

            assert_ron_snapshot!(ast);
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
