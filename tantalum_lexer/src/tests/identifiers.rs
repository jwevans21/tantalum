use crate::multi_token_test_case;

multi_token_test_case! {
    Identifier {
        letter: "a" => "a" (0..1, 1, 1),
        letter_digit: "a1" => "a1" (0..2, 1, 1),
        letter_underscore: "a_" => "a_" (0..2, 1, 1),
        letter_digit_underscore: "a1_" => "a1_" (0..3, 1, 1),
        letter_underscore_digit: "a_1" => "a_1" (0..3, 1, 1),
        underscore_letter: "_a" => "_a" (0..2, 1, 1),
        underscore_letter_digit: "_a1" => "_a1" (0..3, 1, 1),
        underscore_digit: "_1" => "_1" (0..2, 1, 1)
        {}
    }
}
