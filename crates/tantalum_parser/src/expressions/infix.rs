use crate::Parser;
use tantalum_source::SourceSpan;
use tantalum_syntax::{SyntaxKind, SyntaxToken};

pub fn infix_op(p: &mut Parser) -> Option<SyntaxToken> {
    let first = p.peek(1)?;
    let second = p.peek(2);

    match (first.kind(), second.map(|token| token.kind())) {
        (SyntaxKind::LAngle, Some(SyntaxKind::LAngle)) => Some(SyntaxToken::new(
            SyntaxKind::ShiftLeft,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),
        (SyntaxKind::RAngle, Some(SyntaxKind::RAngle)) => Some(SyntaxToken::new(
            SyntaxKind::ShiftRight,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),

        (SyntaxKind::LAngle, Some(SyntaxKind::Equals)) => Some(SyntaxToken::new(
            SyntaxKind::LessThanOrEqual,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),

        (SyntaxKind::RAngle, Some(SyntaxKind::Equals)) => Some(SyntaxToken::new(
            SyntaxKind::GreaterThanOrEqual,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),

        (SyntaxKind::Equals, Some(SyntaxKind::Equals)) => Some(SyntaxToken::new(
            SyntaxKind::Equality,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),
        (SyntaxKind::Bang, Some(SyntaxKind::Equals)) => Some(SyntaxToken::new(
            SyntaxKind::Inequality,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),

        (SyntaxKind::Ampersand, Some(SyntaxKind::Ampersand)) => Some(SyntaxToken::new(
            SyntaxKind::And,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),

        (SyntaxKind::Pipe, Some(SyntaxKind::Pipe)) => Some(SyntaxToken::new(
            SyntaxKind::Or,
            SourceSpan::merge(&[first.span(), second.unwrap().span()]),
        )),

        (
            SyntaxKind::Star
            | SyntaxKind::Slash
            | SyntaxKind::Percent
            | SyntaxKind::Plus
            | SyntaxKind::Minus
            | SyntaxKind::RAngle
            | SyntaxKind::LAngle
            | SyntaxKind::Pipe
            | SyntaxKind::Equals
            | SyntaxKind::Ampersand
            | SyntaxKind::Caret,
            _,
        ) => Some(first),

        _ => None,
    }
}

pub fn infix_op_bp(kind: SyntaxKind) -> (u8, u8) {
    match kind {
        // '*', '/', and '%'
        SyntaxKind::Star | SyntaxKind::Slash | SyntaxKind::Percent => (21, 22),
        // '+' and '-'
        SyntaxKind::Plus | SyntaxKind::Minus => (19, 20),
        // '<<' and '>>'
        SyntaxKind::ShiftLeft | SyntaxKind::ShiftRight => (17, 18),
        // '<', '<=', '>', and '>='
        SyntaxKind::LAngle
        | SyntaxKind::LessThanOrEqual
        | SyntaxKind::RAngle
        | SyntaxKind::GreaterThanOrEqual => (15, 16),
        // '==' and '!='
        SyntaxKind::Equality | SyntaxKind::Inequality => (13, 14),
        // '&'
        SyntaxKind::Ampersand => (11, 12),
        // '^'
        SyntaxKind::Caret => (9, 10),
        // '|'
        SyntaxKind::Pipe => (7, 8),
        // '&&'
        SyntaxKind::And => (5, 6),
        // '||'
        SyntaxKind::Or => (3, 4),
        // '='
        SyntaxKind::Equals => (1, 2),
        _ => (0, 0),
    }
}

pub fn consume_infix_op(p: &mut Parser) -> Option<SyntaxToken> {
    let token = infix_op(p)?;
    match token.kind() {
        // these operators are two characters long (also 2 tokens)
        SyntaxKind::ShiftLeft
        | SyntaxKind::ShiftRight
        | SyntaxKind::LessThanOrEqual
        | SyntaxKind::GreaterThanOrEqual
        | SyntaxKind::Equality
        | SyntaxKind::Inequality
        | SyntaxKind::And
        | SyntaxKind::Or => {
            p.advance();
            p.advance();
        }
        _ => {
            p.advance();
        }
    }
    p.token(token);
    Some(token)
}

#[test]
fn operators() {
    use crate::whitespace::skip_whitespace;
    use tantalum_syntax::pretty;

    crate::setup_parser!(
        files with parser => {
            operators => "\
            * / % \
            + - \
            << >> \
            < <= > >= \
            == != \
            & \
            ^ \
            | \
            && \
            || \
            =",
        }
    );

    let open = parser.open(SyntaxKind::File);
    while !parser.at_end() {
        skip_whitespace(&mut parser);
        let _ = consume_infix_op(&mut parser).expect("expected infix operator");
        skip_whitespace(&mut parser);
    }
    let _ = parser.close(open);

    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    File @ #0 0..45
      Star @ #0 0..1 "*"
      Whitespace @ #0 1..2 " "
      Slash @ #0 2..3 "/"
      Whitespace @ #0 3..4 " "
      Percent @ #0 4..5 "%"
      Whitespace @ #0 5..6 " "
      Plus @ #0 6..7 "+"
      Whitespace @ #0 7..8 " "
      Minus @ #0 8..9 "-"
      Whitespace @ #0 9..10 " "
      ShiftLeft @ #0 10..12 "<<"
      Whitespace @ #0 12..13 " "
      ShiftRight @ #0 13..15 ">>"
      Whitespace @ #0 15..16 " "
      LAngle @ #0 16..17 "<"
      Whitespace @ #0 17..18 " "
      LessThanOrEqual @ #0 18..20 "<="
      Whitespace @ #0 20..21 " "
      RAngle @ #0 21..22 ">"
      Whitespace @ #0 22..23 " "
      GreaterThanOrEqual @ #0 23..25 ">="
      Whitespace @ #0 25..26 " "
      Equality @ #0 26..28 "=="
      Whitespace @ #0 28..29 " "
      Inequality @ #0 29..31 "!="
      Whitespace @ #0 31..32 " "
      Ampersand @ #0 32..33 "&"
      Whitespace @ #0 33..34 " "
      Caret @ #0 34..35 "^"
      Whitespace @ #0 35..36 " "
      Pipe @ #0 36..37 "|"
      Whitespace @ #0 37..38 " "
      And @ #0 38..40 "&&"
      Whitespace @ #0 40..41 " "
      Or @ #0 41..43 "||"
      Whitespace @ #0 43..44 " "
      Equals @ #0 44..45 "="
    "#);
}

#[test]
fn precedence() {
    use crate::expressions::expression;
    use tantalum_syntax::pretty;

    crate::setup_parser!(
        files with parser => {
            precedence => "\
            1 + 2 * 3 \
            1 * 2 + 3 \
            1 + 2 >> 3 \
            1 >> 2 + 3 \
            1 <= 2 << 3 \
            1 << 2 <= 3 \
            1 <= 2 == 3 \
            1 == 2 <= 3 \
            1 == 2 & 3 \
            1 & 2 == 3 \
            1 & 2 ^ 3 \
            1 ^ 2 & 3 \
            1 ^ 2 | 3 \
            1 | 2 ^ 3 \
            1 | 2 && 3 \
            1 && 2 | 3 \
            1 && 2 || 3 \
            1 || 2 && 3 \
            1 || 2 = 3 \
            1 = 2 || 3",
        }
    );

    let open = parser.open(SyntaxKind::File);
    while !parser.at_end() {
        let _ = expression(&mut parser);
    }
    let _ = parser.close(open);

    let tree = parser.finish();

    pretty!(result = tree files);

    insta::assert_snapshot!(result, @r#"
    File @ #0 0..219
      BinaryExpression @ #0 0..10
        Literal @ #0 0..1
          IntegerLiteral @ #0 0..1 "1"
        Whitespace @ #0 1..2 " "
        Plus @ #0 2..3 "+"
        Whitespace @ #0 3..4 " "
        BinaryExpression @ #0 4..10
          Literal @ #0 4..5
            IntegerLiteral @ #0 4..5 "2"
          Whitespace @ #0 5..6 " "
          Star @ #0 6..7 "*"
          Whitespace @ #0 7..8 " "
          Literal @ #0 8..9
            IntegerLiteral @ #0 8..9 "3"
          Whitespace @ #0 9..10 " "
      BinaryExpression @ #0 10..20
        BinaryExpression @ #0 10..16
          Literal @ #0 10..11
            IntegerLiteral @ #0 10..11 "1"
          Whitespace @ #0 11..12 " "
          Star @ #0 12..13 "*"
          Whitespace @ #0 13..14 " "
          Literal @ #0 14..15
            IntegerLiteral @ #0 14..15 "2"
          Whitespace @ #0 15..16 " "
        Plus @ #0 16..17 "+"
        Whitespace @ #0 17..18 " "
        Literal @ #0 18..19
          IntegerLiteral @ #0 18..19 "3"
        Whitespace @ #0 19..20 " "
      BinaryExpression @ #0 20..31
        BinaryExpression @ #0 20..26
          Literal @ #0 20..21
            IntegerLiteral @ #0 20..21 "1"
          Whitespace @ #0 21..22 " "
          Plus @ #0 22..23 "+"
          Whitespace @ #0 23..24 " "
          Literal @ #0 24..25
            IntegerLiteral @ #0 24..25 "2"
          Whitespace @ #0 25..26 " "
        ShiftRight @ #0 26..28 ">>"
        Whitespace @ #0 28..29 " "
        Literal @ #0 29..30
          IntegerLiteral @ #0 29..30 "3"
        Whitespace @ #0 30..31 " "
      BinaryExpression @ #0 31..42
        Literal @ #0 31..32
          IntegerLiteral @ #0 31..32 "1"
        Whitespace @ #0 32..33 " "
        ShiftRight @ #0 33..35 ">>"
        Whitespace @ #0 35..36 " "
        BinaryExpression @ #0 36..42
          Literal @ #0 36..37
            IntegerLiteral @ #0 36..37 "2"
          Whitespace @ #0 37..38 " "
          Plus @ #0 38..39 "+"
          Whitespace @ #0 39..40 " "
          Literal @ #0 40..41
            IntegerLiteral @ #0 40..41 "3"
          Whitespace @ #0 41..42 " "
      BinaryExpression @ #0 42..54
        Literal @ #0 42..43
          IntegerLiteral @ #0 42..43 "1"
        Whitespace @ #0 43..44 " "
        LessThanOrEqual @ #0 44..46 "<="
        Whitespace @ #0 46..47 " "
        BinaryExpression @ #0 47..54
          Literal @ #0 47..48
            IntegerLiteral @ #0 47..48 "2"
          Whitespace @ #0 48..49 " "
          ShiftLeft @ #0 49..51 "<<"
          Whitespace @ #0 51..52 " "
          Literal @ #0 52..53
            IntegerLiteral @ #0 52..53 "3"
          Whitespace @ #0 53..54 " "
      BinaryExpression @ #0 54..66
        BinaryExpression @ #0 54..61
          Literal @ #0 54..55
            IntegerLiteral @ #0 54..55 "1"
          Whitespace @ #0 55..56 " "
          ShiftLeft @ #0 56..58 "<<"
          Whitespace @ #0 58..59 " "
          Literal @ #0 59..60
            IntegerLiteral @ #0 59..60 "2"
          Whitespace @ #0 60..61 " "
        LessThanOrEqual @ #0 61..63 "<="
        Whitespace @ #0 63..64 " "
        Literal @ #0 64..65
          IntegerLiteral @ #0 64..65 "3"
        Whitespace @ #0 65..66 " "
      BinaryExpression @ #0 66..78
        BinaryExpression @ #0 66..73
          Literal @ #0 66..67
            IntegerLiteral @ #0 66..67 "1"
          Whitespace @ #0 67..68 " "
          LessThanOrEqual @ #0 68..70 "<="
          Whitespace @ #0 70..71 " "
          Literal @ #0 71..72
            IntegerLiteral @ #0 71..72 "2"
          Whitespace @ #0 72..73 " "
        Equality @ #0 73..75 "=="
        Whitespace @ #0 75..76 " "
        Literal @ #0 76..77
          IntegerLiteral @ #0 76..77 "3"
        Whitespace @ #0 77..78 " "
      BinaryExpression @ #0 78..90
        Literal @ #0 78..79
          IntegerLiteral @ #0 78..79 "1"
        Whitespace @ #0 79..80 " "
        Equality @ #0 80..82 "=="
        Whitespace @ #0 82..83 " "
        BinaryExpression @ #0 83..90
          Literal @ #0 83..84
            IntegerLiteral @ #0 83..84 "2"
          Whitespace @ #0 84..85 " "
          LessThanOrEqual @ #0 85..87 "<="
          Whitespace @ #0 87..88 " "
          Literal @ #0 88..89
            IntegerLiteral @ #0 88..89 "3"
          Whitespace @ #0 89..90 " "
      BinaryExpression @ #0 90..101
        BinaryExpression @ #0 90..97
          Literal @ #0 90..91
            IntegerLiteral @ #0 90..91 "1"
          Whitespace @ #0 91..92 " "
          Equality @ #0 92..94 "=="
          Whitespace @ #0 94..95 " "
          Literal @ #0 95..96
            IntegerLiteral @ #0 95..96 "2"
          Whitespace @ #0 96..97 " "
        Ampersand @ #0 97..98 "&"
        Whitespace @ #0 98..99 " "
        Literal @ #0 99..100
          IntegerLiteral @ #0 99..100 "3"
        Whitespace @ #0 100..101 " "
      BinaryExpression @ #0 101..112
        Literal @ #0 101..102
          IntegerLiteral @ #0 101..102 "1"
        Whitespace @ #0 102..103 " "
        Ampersand @ #0 103..104 "&"
        Whitespace @ #0 104..105 " "
        BinaryExpression @ #0 105..112
          Literal @ #0 105..106
            IntegerLiteral @ #0 105..106 "2"
          Whitespace @ #0 106..107 " "
          Equality @ #0 107..109 "=="
          Whitespace @ #0 109..110 " "
          Literal @ #0 110..111
            IntegerLiteral @ #0 110..111 "3"
          Whitespace @ #0 111..112 " "
      BinaryExpression @ #0 112..122
        BinaryExpression @ #0 112..118
          Literal @ #0 112..113
            IntegerLiteral @ #0 112..113 "1"
          Whitespace @ #0 113..114 " "
          Ampersand @ #0 114..115 "&"
          Whitespace @ #0 115..116 " "
          Literal @ #0 116..117
            IntegerLiteral @ #0 116..117 "2"
          Whitespace @ #0 117..118 " "
        Caret @ #0 118..119 "^"
        Whitespace @ #0 119..120 " "
        Literal @ #0 120..121
          IntegerLiteral @ #0 120..121 "3"
        Whitespace @ #0 121..122 " "
      BinaryExpression @ #0 122..132
        Literal @ #0 122..123
          IntegerLiteral @ #0 122..123 "1"
        Whitespace @ #0 123..124 " "
        Caret @ #0 124..125 "^"
        Whitespace @ #0 125..126 " "
        BinaryExpression @ #0 126..132
          Literal @ #0 126..127
            IntegerLiteral @ #0 126..127 "2"
          Whitespace @ #0 127..128 " "
          Ampersand @ #0 128..129 "&"
          Whitespace @ #0 129..130 " "
          Literal @ #0 130..131
            IntegerLiteral @ #0 130..131 "3"
          Whitespace @ #0 131..132 " "
      BinaryExpression @ #0 132..142
        BinaryExpression @ #0 132..138
          Literal @ #0 132..133
            IntegerLiteral @ #0 132..133 "1"
          Whitespace @ #0 133..134 " "
          Caret @ #0 134..135 "^"
          Whitespace @ #0 135..136 " "
          Literal @ #0 136..137
            IntegerLiteral @ #0 136..137 "2"
          Whitespace @ #0 137..138 " "
        Pipe @ #0 138..139 "|"
        Whitespace @ #0 139..140 " "
        Literal @ #0 140..141
          IntegerLiteral @ #0 140..141 "3"
        Whitespace @ #0 141..142 " "
      BinaryExpression @ #0 142..152
        Literal @ #0 142..143
          IntegerLiteral @ #0 142..143 "1"
        Whitespace @ #0 143..144 " "
        Pipe @ #0 144..145 "|"
        Whitespace @ #0 145..146 " "
        BinaryExpression @ #0 146..152
          Literal @ #0 146..147
            IntegerLiteral @ #0 146..147 "2"
          Whitespace @ #0 147..148 " "
          Caret @ #0 148..149 "^"
          Whitespace @ #0 149..150 " "
          Literal @ #0 150..151
            IntegerLiteral @ #0 150..151 "3"
          Whitespace @ #0 151..152 " "
      BinaryExpression @ #0 152..163
        BinaryExpression @ #0 152..158
          Literal @ #0 152..153
            IntegerLiteral @ #0 152..153 "1"
          Whitespace @ #0 153..154 " "
          Pipe @ #0 154..155 "|"
          Whitespace @ #0 155..156 " "
          Literal @ #0 156..157
            IntegerLiteral @ #0 156..157 "2"
          Whitespace @ #0 157..158 " "
        And @ #0 158..160 "&&"
        Whitespace @ #0 160..161 " "
        Literal @ #0 161..162
          IntegerLiteral @ #0 161..162 "3"
        Whitespace @ #0 162..163 " "
      BinaryExpression @ #0 163..174
        Literal @ #0 163..164
          IntegerLiteral @ #0 163..164 "1"
        Whitespace @ #0 164..165 " "
        And @ #0 165..167 "&&"
        Whitespace @ #0 167..168 " "
        BinaryExpression @ #0 168..174
          Literal @ #0 168..169
            IntegerLiteral @ #0 168..169 "2"
          Whitespace @ #0 169..170 " "
          Pipe @ #0 170..171 "|"
          Whitespace @ #0 171..172 " "
          Literal @ #0 172..173
            IntegerLiteral @ #0 172..173 "3"
          Whitespace @ #0 173..174 " "
      BinaryExpression @ #0 174..186
        BinaryExpression @ #0 174..181
          Literal @ #0 174..175
            IntegerLiteral @ #0 174..175 "1"
          Whitespace @ #0 175..176 " "
          And @ #0 176..178 "&&"
          Whitespace @ #0 178..179 " "
          Literal @ #0 179..180
            IntegerLiteral @ #0 179..180 "2"
          Whitespace @ #0 180..181 " "
        Or @ #0 181..183 "||"
        Whitespace @ #0 183..184 " "
        Literal @ #0 184..185
          IntegerLiteral @ #0 184..185 "3"
        Whitespace @ #0 185..186 " "
      BinaryExpression @ #0 186..198
        Literal @ #0 186..187
          IntegerLiteral @ #0 186..187 "1"
        Whitespace @ #0 187..188 " "
        Or @ #0 188..190 "||"
        Whitespace @ #0 190..191 " "
        BinaryExpression @ #0 191..198
          Literal @ #0 191..192
            IntegerLiteral @ #0 191..192 "2"
          Whitespace @ #0 192..193 " "
          And @ #0 193..195 "&&"
          Whitespace @ #0 195..196 " "
          Literal @ #0 196..197
            IntegerLiteral @ #0 196..197 "3"
          Whitespace @ #0 197..198 " "
      BinaryExpression @ #0 198..209
        BinaryExpression @ #0 198..205
          Literal @ #0 198..199
            IntegerLiteral @ #0 198..199 "1"
          Whitespace @ #0 199..200 " "
          Or @ #0 200..202 "||"
          Whitespace @ #0 202..203 " "
          Literal @ #0 203..204
            IntegerLiteral @ #0 203..204 "2"
          Whitespace @ #0 204..205 " "
        Equals @ #0 205..206 "="
        Whitespace @ #0 206..207 " "
        Literal @ #0 207..208
          IntegerLiteral @ #0 207..208 "3"
        Whitespace @ #0 208..209 " "
      BinaryExpression @ #0 209..219
        Literal @ #0 209..210
          IntegerLiteral @ #0 209..210 "1"
        Whitespace @ #0 210..211 " "
        Equals @ #0 211..212 "="
        Whitespace @ #0 212..213 " "
        BinaryExpression @ #0 213..219
          Literal @ #0 213..214
            IntegerLiteral @ #0 213..214 "2"
          Whitespace @ #0 214..215 " "
          Or @ #0 215..217 "||"
          Whitespace @ #0 217..218 " "
          Literal @ #0 218..219
            IntegerLiteral @ #0 218..219 "3"
    "#);
}
