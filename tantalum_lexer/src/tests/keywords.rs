use crate::single_token_test_case;

single_token_test_case! {
    KeywordFn : "fn" => "fn" (0..2, 1, 1),
    KeywordLet : "let" => "let" (0..3, 1, 1),
    KeywordIf : "if" => "if" (0..2, 1, 1),
    KeywordElse : "else" => "else" (0..4, 1, 1),
    KeywordWhile : "while" => "while" (0..5, 1, 1),
    KeywordFor : "for" => "for" (0..3, 1, 1),
    KeywordReturn : "return" => "return" (0..6, 1, 1),
    KeywordBreak : "break" => "break" (0..5, 1, 1),
    KeywordContinue : "continue" => "continue" (0..8, 1, 1),
    KeywordConst : "const" => "const" (0..5, 1, 1),
    KeywordTrue : "true" => "true" (0..4, 1, 1),
    KeywordFalse : "false" => "false" (0..5, 1, 1)
}
