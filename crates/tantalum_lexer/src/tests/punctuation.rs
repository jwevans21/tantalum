use crate::single_token_test_case;

single_token_test_case! {
    LeftParen : "(" => "(" (0..1, 1, 1),
    RightParen : ")" => ")" (0..1, 1, 1),
    LeftBrace : "{" => "{" (0..1, 1, 1),
    RightBrace : "}" => "}" (0..1, 1, 1),
    LeftBracket : "[" => "[" (0..1, 1, 1),
    RightBracket : "]" => "]" (0..1, 1, 1),
    Comma : "," => "," (0..1, 1, 1),
    Semicolon : ";" => ";" (0..1, 1, 1)
}
