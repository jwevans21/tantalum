use rstest::rstest;

use crate::Lexer;

mod identifiers;
mod keywords;
mod literals;
mod operators;
mod punctuation;

#[rstest]
#[case("", 0, None)]
#[case("", 1, None)]
#[case("", usize::MAX, None)]
#[case("abcdef", 0, None)]
#[case("abcdef", 1, Some('a'))]
#[case("abcdef", 2, Some('b'))]
#[case("abcdef", 3, Some('c'))]
#[case("abcdef", 4, Some('d'))]
#[case("abcdef", 5, Some('e'))]
#[case("abcdef", 6, Some('f'))]
#[case("abcdef", 7, None)]
fn peek_characters(#[case] source: &str, #[case] count: usize, #[case] expected: Option<char>) {
    let lexer = Lexer::new("main.tan", source);

    assert_eq!(lexer.peek_characters(count), expected);
}

#[rstest]
#[case("", 0, None)]
#[case("", 1, None)]
#[case("", usize::MAX, None)]
#[case("abcdef", 0, None)]
#[case("abcdef", 1, Some('a'))]
#[case("abcdef", 2, Some('b'))]
#[case("abcdef", 3, Some('c'))]
#[case("abcdef", 4, Some('d'))]
#[case("abcdef", 5, Some('e'))]
#[case("abcdef", 6, Some('f'))]
#[case("abcdef", 7, None)]
#[case("hello\nworld", 7, Some('w'))]
fn next_characters(#[case] source: &str, #[case] count: usize, #[case] expected: Option<char>) {
    let mut lexer = Lexer::new("main.ta", source);

    assert_eq!(lexer.next_characters(count), expected);

    assert_eq!(
        lexer.location.position(),
        if count > source.len() { 0 } else { count }
    );
}

#[macro_export]
macro_rules! single_token_test_case {
    {$($kind:ident : $source:literal => $lexeme:literal ($span:expr, $lines:expr, $columns:expr)),*} => {
        $(
            #[test]
            #[expect(non_snake_case)]
            fn $kind() {
                let mut lexer = $crate::Lexer::new("main.ta", $source);

                let token = lexer.next_token();

                pretty_assertions::assert_eq!(
                    token,
                    Some(
                        tantalum_span::Spanned::new(
                            tantalum_span::Span::new(
                                tantalum_span::Location::new_at("main.ta", ($span).start, $lines, $columns),
                                tantalum_span::Location::new_at("main.ta", ($span).end, $lines, $columns + $lexeme.len()),
                            ),
                            $crate::token::Token::new(
                                $lexeme,
                                $crate::token_kind::TokenKind::$kind
                            )
                        )
                    )
                );

                pretty_assertions::assert_eq!(lexer.next_token(), None);
            }
        )*
    };
}

#[macro_export]
macro_rules! multi_token_test_case {
    {
        $($kind:ident {
            $($name:ident: $source:literal => $lexeme:literal ($span:expr, $lines:expr, $columns:expr)),*
            {
                $($kind2:ident {
                    $($name2:ident: $source2:literal => $lexeme2:literal ($span2:expr, $lines2:expr, $columns2:expr)),*
                }),*
            }
        }),*
    } => {
        $(
            #[expect(non_snake_case)]
            mod $kind {
                $(
                    #[test]
                    fn $name() {
                        let mut lexer = $crate::Lexer::new("main.ta", $source);

                        let token = lexer.next_token();

                        pretty_assertions::assert_eq!(
                            token,
                            Some(
                                tantalum_span::Spanned::new(
                                    tantalum_span::Span::new(
                                        tantalum_span::Location::new_at("main.ta", ($span).start, $lines, $columns),
                                        tantalum_span::Location::new_at("main.ta", ($span).end, $lines, $columns + $lexeme.len()),
                                    ),
                                    $crate::token::Token::new(
                                        $lexeme,
                                        $crate::token_kind::TokenKind::$kind
                                    )
                                )
                            )
                        );

                        // pretty_assertions::assert_eq!(lexer.next_token(), None);
                    }
                )*

                $(
                    $(
                        #[test]
                        fn $name2() {
                            let mut lexer = $crate::Lexer::new("main.ta", $source2);

                            let token = lexer.next_token();

                            pretty_assertions::assert_eq!(
                                token,
                                Some(
                                    tantalum_span::Spanned::new(
                                        tantalum_span::Span::new(
                                            tantalum_span::Location::new_at("main.ta", ($span2).start, $lines2, $columns2),
                                            tantalum_span::Location::new_at("main.ta", ($span2).end, $lines2, $columns2 + $lexeme2.len()),
                                        ),
                                        $crate::token::Token::new(
                                            $lexeme2,
                                            $crate::token_kind::TokenKind::$kind2
                                        )
                                    )
                                )
                            );

                            // pretty_assertions::assert_eq!(lexer.next_token(), None);
                        }
                    )*
                )*
            }
        )*
    };
}
