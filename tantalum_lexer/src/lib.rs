//! # Tantalum Lexer
//!
//! Consumes source code written in the tantalum language and produces tokens
//! to be consumed by a parser.

#[cfg(test)]
mod tests;

pub mod token;
pub mod token_kind;

use tantalum_span::{Location, Span, Spanned};

use crate::token::Token;
use crate::token_kind::TokenKind;

/// The lexer for the Tantalum language, this will consume source code and produce
/// tokens with origin information. This lexer is designed to be used in a streaming
/// fashion, where tokens are consumed as they are needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer<'file_name, 'source> {
    /// The current location of the lexer
    location: Location<'file_name>,
    /// The source code to lexed into tokens
    source: &'source str,
}

impl<'file_name, 'source> Lexer<'file_name, 'source> {
    /// Create a lexer for the Tantalum language over the provided `source`
    ///
    /// The file name is provided to allow useful diagnostics to be produced
    ///
    /// # Example
    /// ```
    /// use tantalum_lexer::Lexer;
    ///
    /// let lexer = Lexer::new("main.tan", "fn main(): i32 { return 0; }");
    ///
    /// // Use the lexer here (will likely require `lexer` to be mutable)
    /// ```
    #[must_use]
    #[inline]
    pub fn new(file_name: &'file_name str, source: &'source str) -> Self {
        return Self {
            location: Location::new(file_name),
            source,
        };
    }

    #[must_use]
    #[inline]
    pub fn location(&self) -> Location<'file_name> {
        return self.location;
    }

    #[must_use]
    #[inline]
    pub fn source(&self) -> &'source str {
        return self.source;
    }

    #[must_use]
    #[inline]
    pub fn file_name(&self) -> &'file_name str {
        return self.location.file_name();
    }

    #[must_use]
    #[inline]
    #[expect(clippy::too_many_lines, reason = "Long match statement")]
    pub fn next_token(&mut self) -> Option<Spanned<'file_name, Token<'source>>> {
        self.skip_whitespace();

        let start = self.location;

        /// Produce a Token based on a type and length
        macro_rules! lex {
            // Special case  for tokens already consumed
            ($production:path, 0) => {{
                return self.create_token($production, start);
            }};
            // General case for tokens that need to be consumed
            ($production:path, $length:expr) => {{
                let _: char = self.next_characters($length)?;
                return self.create_token($production, start);
            }};
        }

        match self.peek_characters(1)? {
            '(' => lex!(TokenKind::LeftParen, 1),
            ')' => lex!(TokenKind::RightParen, 1),
            '{' => lex!(TokenKind::LeftBrace, 1),
            '}' => lex!(TokenKind::RightBrace, 1),
            '[' => lex!(TokenKind::LeftBracket, 1),
            ']' => lex!(TokenKind::RightBracket, 1),
            ',' => lex!(TokenKind::Comma, 1),
            ';' => lex!(TokenKind::Semicolon, 1),

            ':' => {
                if let Some(':') = self.peek_characters(2) {
                    lex!(TokenKind::ColonColon, 2)
                }
                lex!(TokenKind::Colon, 1)
            }
            '.' => match self.peek_characters(2) {
                Some('*') => lex!(TokenKind::DotStar, 2),
                Some('&') => lex!(TokenKind::DotAmpersand, 2),
                Some('.')
                    if self
                        .peek_characters(3)
                        .is_some_and(|character| return character == '.') =>
                {
                    lex!(TokenKind::DotDotDot, 3)
                }
                _ => lex!(TokenKind::Dot, 1),
            },
            '=' => {
                if let Some('=') = self.peek_characters(2) {
                    lex!(TokenKind::EqualEqual, 2)
                }
                lex!(TokenKind::Equal, 1)
            }
            '+' => lex!(TokenKind::Plus, 1),
            '-' => lex!(TokenKind::Minus, 1),
            '*' => lex!(TokenKind::Star, 1),
            '/' => lex!(TokenKind::Slash, 1),
            '%' => lex!(TokenKind::Percent, 1),
            '&' => {
                if let Some('&') = self.peek_characters(2) {
                    lex!(TokenKind::AmpersandAmpersand, 2)
                }
                lex!(TokenKind::Ampersand, 1)
            }
            '|' => {
                if let Some('|') = self.peek_characters(2) {
                    lex!(TokenKind::PipePipe, 2)
                }
                lex!(TokenKind::Pipe, 1)
            }
            '!' => {
                if let Some('=') = self.peek_characters(2) {
                    lex!(TokenKind::ExclamationEqual, 2)
                }
                lex!(TokenKind::Exclamation, 1)
            }
            '^' => lex!(TokenKind::Caret, 1),
            '~' => lex!(TokenKind::Tilde, 1),
            '<' => match self.peek_characters(2) {
                Some('<') => lex!(TokenKind::LeftAngleLeftAngle, 2),
                Some('=') => lex!(TokenKind::LeftAngleEqual, 2),
                _ => lex!(TokenKind::LeftAngle, 1),
            },
            '>' => match self.peek_characters(2) {
                Some('>') => lex!(TokenKind::RightAngleRightAngle, 2),
                Some('=') => lex!(TokenKind::RightAngleEqual, 2),
                _ => lex!(TokenKind::RightAngle, 1),
            },

            'a'..='z' | 'A'..='Z' | '_' => {
                while let Some(character) = self.peek_characters(1) {
                    if character.is_ascii_alphanumeric() || character == '_' {
                        let _: char = self.next_characters(1)?;
                    } else {
                        break;
                    }
                }

                match self.source.get(start.range(&self.location))? {
                    "fn" => lex!(TokenKind::KeywordFn, 0),
                    "extern" => lex!(TokenKind::KeywordExtern, 0),
                    "let" => lex!(TokenKind::KeywordLet, 0),
                    "if" => lex!(TokenKind::KeywordIf, 0),
                    "else" => lex!(TokenKind::KeywordElse, 0),
                    "while" => lex!(TokenKind::KeywordWhile, 0),
                    "for" => lex!(TokenKind::KeywordFor, 0),
                    "return" => lex!(TokenKind::KeywordReturn, 0),
                    "break" => lex!(TokenKind::KeywordBreak, 0),
                    "continue" => lex!(TokenKind::KeywordContinue, 0),
                    "const" => lex!(TokenKind::KeywordConst, 0),
                    "true" => lex!(TokenKind::KeywordTrue, 0),
                    "false" => lex!(TokenKind::KeywordFalse, 0),
                    _ => lex!(TokenKind::Identifier, 0),
                }
            }

            '0'..='9' => {
                // Check for binary, octal, decimal, or hexadecimal integer literals
                match self.peek_characters(2) {
                    Some('b')
                        if self
                            .peek_characters(3)
                            .is_some_and(|character| return character.is_digit(2)) =>
                    {
                        let _: char = self.next_characters(2)?;

                        while let Some(character) = self.peek_characters(1) {
                            if character.is_digit(2) {
                                let _: char = self.next_characters(1)?;
                            } else {
                                break;
                            }
                        }

                        lex!(TokenKind::BinaryIntegerLiteral, 0);
                    }
                    Some('o')
                        if self
                            .peek_characters(3)
                            .is_some_and(|character| return character.is_digit(8)) =>
                    {
                        let _: char = self.next_characters(2)?;

                        while let Some(character) = self.peek_characters(1) {
                            if character.is_digit(8) {
                                let _: char = self.next_characters(1)?;
                            } else {
                                break;
                            }
                        }

                        lex!(TokenKind::OctalIntegerLiteral, 0);
                    }
                    Some('x')
                        if self
                            .peek_characters(3)
                            .is_some_and(|character| return character.is_ascii_hexdigit()) =>
                    {
                        let _: char = self.next_characters(2)?;

                        while let Some(character) = self.peek_characters(1) {
                            if character.is_ascii_hexdigit() {
                                let _: char = self.next_characters(1)?;
                            } else {
                                break;
                            }
                        }

                        lex!(TokenKind::HexadecimalIntegerLiteral, 0);
                    }
                    _ => {
                        while let Some(character) = self.peek_characters(1) {
                            if character.is_ascii_digit() {
                                let _: char = self.next_characters(1)?;
                            } else {
                                break;
                            }
                        }

                        match self.peek_characters(1) {
                            Some('.')
                                if self
                                    .peek_characters(2)
                                    .is_some_and(|character| character.is_ascii_digit()) =>
                            {
                                let _: char = self.next_characters(1)?;

                                while let Some(character) = self.peek_characters(1) {
                                    if character.is_ascii_digit() {
                                        let _: char = self.next_characters(1)?;
                                    } else {
                                        break;
                                    }
                                }

                                match self.peek_characters(1) {
                                    Some('e' | 'E') => {
                                        let _: char = self.next_characters(1)?;

                                        if let Some('+' | '-') = self.peek_characters(1) {
                                            let _: char = self.next_characters(1)?;
                                        }

                                        while let Some(character) = self.peek_characters(1) {
                                            if character.is_ascii_digit() {
                                                let _: char = self.next_characters(1)?;
                                            } else {
                                                break;
                                            }
                                        }

                                        lex!(TokenKind::FloatLiteral, 0);
                                    }
                                    _ => lex!(TokenKind::FloatLiteral, 0),
                                }
                            }
                            _ => lex!(TokenKind::DecimalIntegerLiteral, 0),
                        }
                    }
                }
            }

            '"' => {
                let _: char = self.next_characters(1)?;

                while let Some(character) = self.peek_characters(1) {
                    match character {
                        '\\' => {
                            let _: char = self.next_characters(1)?;
                            let _: char = self.next_characters(1)?;
                        }
                        '"' => {
                            let _: char = self.next_characters(1)?;
                            break;
                        }
                        _ => {
                            let _: char = self.next_characters(1)?;
                        }
                    }
                }

                lex!(TokenKind::StringLiteral, 0);
            }

            '\'' => {
                let _: char = self.next_characters(1)?;

                match self.peek_characters(1) {
                    Some('\\') => {
                        let _: char = self.next_characters(1)?;
                        let _: char = self.next_characters(1)?;
                    }
                    _ => {
                        let _: char = self.next_characters(1)?;
                    }
                }

                match self.peek_characters(1) {
                    Some('\'') => {
                        let _: char = self.next_characters(1)?;
                    }
                    _ => {
                        return None;
                    }
                }

                lex!(TokenKind::CharacterLiteral, 0);
            }

            _ => lex!(TokenKind::Unknown, 1),
        }
    }

    /// Build a token with the current state based on a token type and the
    /// starting position of the token
    fn create_token(
        &self,
        token_kind: TokenKind,
        start: Location<'file_name>,
    ) -> Option<Spanned<'file_name, Token<'source>>> {
        let span = Span::new(start, self.location);
        return Some(Spanned::new(
            span,
            Token::new(self.source.get(span.range())?, token_kind),
        ));
    }

    /// Skip any whitespace characters in the source code
    fn skip_whitespace(&mut self) {
        while let Some(character) = self.peek_characters(1) {
            if character.is_whitespace() {
                let _: char = self
                    .next_characters(1)
                    .expect("Failed to unwrap character already shown to exist");
            } else {
                break;
            }
        }
    }

    /// Peek into the source code by `count` characters
    ///
    /// Return the the `count` character from the current position if it exists,
    /// otherwise it returns `None`
    #[must_use]
    #[inline]
    fn peek_characters(&self, count: usize) -> Option<char> {
        let next = self.source.get(self.location.position()..)?;

        if next.len() < count {
            return None;
        }

        return next.chars().take(count).last();
    }

    /// Advance the lexer by `count` characters
    ///
    /// If `count` exceeds the remaining length then `None` is returned and
    /// the `Lexer` has not been updated.
    ///
    /// This will update the position based on UTF-8 character lengths, it will
    /// also update the line and column as appropriate
    #[must_use]
    #[inline]
    fn next_characters(&mut self, count: usize) -> Option<char> {
        let next = self.source.get(self.location.position()..)?;

        if next.len() < count {
            return None;
        }

        return next
            .chars()
            .take(count)
            .inspect(|character| {
                self.location.advance(*character);
            })
            .last();
    }
}

impl<'file_name, 'source> Iterator for Lexer<'file_name, 'source> {
    type Item = Spanned<'file_name, Token<'source>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return self.next_token();
    }
}
