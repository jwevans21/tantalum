/// The known kinds of tokens for the Tantalum language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,

    Colon,
    ColonColon,
    Dot,
    DotStar,
    DotAmpersand,
    DotDotDot,

    Equal,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    AmpersandAmpersand,
    PipePipe,
    Exclamation,

    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftAngleLeftAngle,
    RightAngleRightAngle,

    EqualEqual,
    ExclamationEqual,
    LeftAngle,
    LeftAngleEqual,
    RightAngle,
    RightAngleEqual,

    KeywordFn,
    KeywordExtern,
    KeywordLet,
    KeywordIf,
    KeywordElse,
    KeywordWhile,
    KeywordFor,
    KeywordReturn,
    KeywordBreak,
    KeywordContinue,
    KeywordConst,
    KeywordTrue,
    KeywordFalse,

    Identifier,

    BinaryIntegerLiteral,
    OctalIntegerLiteral,
    DecimalIntegerLiteral,
    HexadecimalIntegerLiteral,

    FloatLiteral,

    StringLiteral,

    CharacterLiteral,

    /// Any unknown character found in the source code
    ///
    /// This will likely cause a syntax error to be raised, but can also be
    /// ignored by the parser while attempting to recover from this unexpected
    /// token.
    #[default]
    Unknown,
}
