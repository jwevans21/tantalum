use crate::multi_token_test_case;

multi_token_test_case! {
    FloatLiteral {
        zero: "0.0" => "0.0" (0..3, 1, 1),
        zero_exponent: "0.0e0" => "0.0e0" (0..5, 1, 1),
        zero_exponent_positive: "0.0e+0" => "0.0e+0" (0..6, 1, 1),
        zero_exponent_negative: "0.0e-0" => "0.0e-0" (0..6, 1, 1),
        zero_exponent_positive_capital: "0.0E+0" => "0.0E+0" (0..6, 1, 1),
        pi: "3.14159" => "3.14159" (0..7, 1, 1)
        {
            DecimalIntegerLiteral {
                trailing_dot: "0." => "0" (0..1, 1, 1)
            }
        }
    }
}
