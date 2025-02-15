use crate::multi_token_test_case;

multi_token_test_case! {
    CharacterLiteral {
        basic: "'a'" => "'a'" (0..3, 1, 1),
        escaped: "'\\''" => "'\\''" (0..4, 1, 1),
        escaped_newline: "'\\n'" => "'\\n'" (0..4, 1, 1)
        {}
    }
}
