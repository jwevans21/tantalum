use tantalum_lexer::Lexer;

use crate::Parser;

macro_rules! prefix_unary_expression_test_cases {
    {
        $($kind:ident: $operator:literal),*
    } => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $kind() {
                const SOURCE: &str = concat!($operator, "1");
                let lexer = tantalum_lexer::Lexer::new(stringify!($kind), SOURCE);
                let mut parser = crate::Parser::new(lexer);

                let result = parser.parse_expression();

                insta::assert_ron_snapshot!(result);
            }
        )*
    };
}

macro_rules! postfix_unary_expression_test_cases {
    {
        $($kind:ident: $operator:literal),*
    } => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $kind() {
                const SOURCE: &str = concat!("1", $operator);
                let lexer = tantalum_lexer::Lexer::new(stringify!($kind), SOURCE);
                let mut parser = crate::Parser::new(lexer);

                let result = parser.parse_expression();

                insta::assert_ron_snapshot!(result);
            }
        )*
    }
}

macro_rules! binary_expression_test_cases {
    {
        $($kind:ident: $operator:literal),*
    } => {
        #[allow(non_snake_case)]
        mod BinaryOperation {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $kind() {
                    const SOURCE: &str = concat!("1 ", $operator, " 2");
                    let lexer = tantalum_lexer::Lexer::new(stringify!($kind), SOURCE);
                    let mut parser = crate::Parser::new(lexer);

                    let result = parser.parse_expression();

                    insta::assert_ron_snapshot!(result);
                }
            )*
        }
    };
}

prefix_unary_expression_test_cases! {
    Negation: "-",
    LogicalNegation: "!",
    BitwiseNegation: "~"
}

postfix_unary_expression_test_cases! {
    AddressOf: ".&",
    Deref: ".*"
}

binary_expression_test_cases! {
    Addition: "+",
    Subtraction: "-",
    Multiplication: "*",
    Division: "/",
    Modulus: "%",

    LogicalAnd: "&&",
    LogicalOr: "||",

    Equality: "==",
    NotEqual: "!=",
    LessThan: "<",
    LessThanOrEqual: "<=",
    GreaterThan: ">",
    GreaterThanOrEqual: ">=",

    BitwiseAnd: "&",
    BitwiseOr: "|",
    BitwiseXor: "^",

    ShiftLeft: "<<",
    ShiftRight: ">>"
}

#[test]
fn function_call() {
    const SOURCE: &str = "foo()";
    let lexer = tantalum_lexer::Lexer::new("function_call", SOURCE);
    let mut parser = crate::Parser::new(lexer);

    let result = parser.parse_expression();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn function_call_with_arguments() {
    const SOURCE: &str = "foo(1, 2)";
    let lexer = tantalum_lexer::Lexer::new("function_call_with_arguments", SOURCE);
    let mut parser = crate::Parser::new(lexer);

    let result = parser.parse_expression();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn array_access() {
    const SOURCE: &str = "foo[1]";
    let lexer = tantalum_lexer::Lexer::new("array_access", SOURCE);
    let mut parser = crate::Parser::new(lexer);

    let result = parser.parse_expression();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn basic_addition() {
    let lexer = Lexer::new("basic_addition", "1 + 2");
    let mut parser = Parser::new(lexer);

    let result = parser.parse_expression();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn multiplication_with_addition_rhs() {
    let lexer = Lexer::new("multiplication_with_addition_rhs", "1 * 2 + 3");
    let mut parser = Parser::new(lexer);

    let result = parser.parse_expression();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn type_cast() {
    let lexer = Lexer::new("type_cast", "1:u8");
    let mut parser = Parser::new(lexer);

    let result = parser.parse_expression();

    insta::assert_ron_snapshot!(result);
}

#[test]
fn type_cast_with_binary_expression() {
    let lexer = Lexer::new("type_cast_with_binary_expression", "1 + 2:u8");
    let mut parser = Parser::new(lexer);

    let result = parser.parse_expression();

    insta::assert_ron_snapshot!(result);
}
