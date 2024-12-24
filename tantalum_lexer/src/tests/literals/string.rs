use crate::multi_token_test_case;

multi_token_test_case! {
    StringLiteral {
        basic: "\"Hello, World!\"" => "\"Hello, World!\"" (0..15, 1, 1),
        escaped: "\"\\\"Hello, World!\\\"\"" => "\"\\\"Hello, World!\\\"\"" (0..19, 1, 1),
        escaped_newline: "\"Hello,\\nWorld!\"" => "\"Hello,\\nWorld!\"" (0..16, 1, 1)
        {}
    }
}
