use crate::multi_token_test_case;

multi_token_test_case! {
    BinaryIntegerLiteral {
        zero: "0b0" => "0b0" (0..3, 1, 1),
        one: "0b1" => "0b1" (0..3, 1, 1),
        two: "0b10" => "0b10" (0..4, 1, 1),
        three: "0b11" => "0b11" (0..4, 1, 1),
        leading_zero: "0b01" => "0b01" (0..4, 1, 1)
        {
            DecimalIntegerLiteral {
                invalid_next: "0b2" => "0" (0..1, 1, 1)
            }
        }
    },
    OctalIntegerLiteral {
        zero: "0o0" => "0o0" (0..3, 1, 1),
        one: "0o1" => "0o1" (0..3, 1, 1),
        two: "0o2" => "0o2" (0..3, 1, 1),
        eight: "0o10" => "0o10" (0..4, 1, 1),
        leading_zero: "0o01" => "0o01" (0..4, 1, 1)
        {
            DecimalIntegerLiteral {
                invalid_next: "0o8" => "0" (0..1, 1, 1)
            }
        }
    },
    DecimalIntegerLiteral {
        zero: "0" => "0" (0..1, 1, 1),
        one: "1" => "1" (0..1, 1, 1),
        two: "2" => "2" (0..1, 1, 1),
        three: "3" => "3" (0..1, 1, 1),
        leading_zero: "01" => "01" (0..2, 1, 1)
        {}
    },
    HexadecimalIntegerLiteral {
        zero: "0x0" => "0x0" (0..3, 1, 1),
        one: "0x1" => "0x1" (0..3, 1, 1),
        two: "0x2" => "0x2" (0..3, 1, 1),
        three: "0x3" => "0x3" (0..3, 1, 1),
        leading_zero: "0x01" => "0x01" (0..4, 1, 1),
        lowercase_a: "0xa" => "0xa" (0..3, 1, 1),
        uppercase_a: "0xA" => "0xA" (0..3, 1, 1),
        lowercase_f: "0xf" => "0xf" (0..3, 1, 1),
        uppercase_f: "0xF" => "0xF" (0..3, 1, 1)
        {
            DecimalIntegerLiteral {
                invalid_next: "0xg" => "0" (0..1, 1, 1)
            }
        }
    }
}
