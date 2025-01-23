//! # Tantalum Lexer
//!
//! This crate provides a lexer for the Tantalum language.

use tantalum_source::{SourceFile, SourceLocation, SourceSpan};
use tantalum_syntax::{SyntaxKind, SyntaxToken};

/// A lexer for the Tantalum language.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lexer<'file> {
    /// The source file that the lexer is currently lexing.
    file: &'file SourceFile,
    /// The current location in the source file.
    location: SourceLocation,
}

impl<'file> Lexer<'file> {
    /// Create a new lexer from a source file.
    ///
    /// ```rust
    /// use tantalum_lexer::Lexer;
    /// use tantalum_source::SourceFileCollection;
    ///
    /// let mut files = SourceFileCollection::new();
    ///
    /// let file_id = files.add_file("file.tan".to_string(), "fn main() {}".to_string());
    ///
    /// let file = files.get_file(file_id).expect("file should exist since it was just added");
    ///
    /// let lexer = Lexer::new(file);
    /// ```
    #[must_use]
    pub fn new(file: &'file SourceFile) -> Self {
        Self {
            file,
            location: file.location_start(),
        }
    }

    /// Get the source file that the lexer is currently lexing.
    #[must_use]
    pub fn source(&self) -> &'file SourceFile {
        self.file
    }

    /// Get the current location in the source file.
    #[must_use]
    pub fn location(&self) -> SourceLocation {
        self.location
    }

    /// Get the character at a location in the source file.
    fn at_location(&self, location: SourceLocation) -> Option<char> {
        self.file.read_char(location)
    }

    /// Extract the next token from the source file.
    ///
    /// ```rust
    /// use tantalum_lexer::Lexer;
    /// use tantalum_source::SourceFileCollection;
    /// use tantalum_syntax::SyntaxKind;
    ///
    /// let mut files = SourceFileCollection::new();
    ///
    /// let file_id = files.add_file("file.tan".to_string(), "fn main() {}".to_string());
    ///
    /// let file = files.get_file(file_id).expect("file should exist since it was just added");
    ///
    /// let mut lexer = Lexer::new(file);
    ///
    /// let token = lexer.next_token().expect("token should exist since the file is not empty");
    ///
    /// assert_eq!(token.kind(), SyntaxKind::Fn);
    /// ```
    #[must_use]
    pub fn next_token(&mut self) -> Option<SyntaxToken> {
        let start = self.location;

        let (kind, end) = self.characters_to_token(start)?;

        self.location = end;

        Some(SyntaxToken::new(
            kind,
            SourceSpan::new(self.file.id(), start.offset(), end.offset()),
        ))
    }

    /// Peek at the next token in the source file.
    ///
    /// ```rust
    /// use tantalum_lexer::Lexer;
    /// use tantalum_source::SourceFileCollection;
    /// use tantalum_syntax::SyntaxKind;
    ///
    /// let mut files = SourceFileCollection::new();
    ///
    /// let file_id = files.add_file("file.tan".to_string(), "fn main() {}".to_string());
    ///
    /// let file = files.get_file(file_id).expect("file should exist since it was just added");
    ///
    /// let mut lexer = Lexer::new(file);
    ///
    /// let token = lexer.peek_token(1).expect("token should exist since the file is not empty");
    ///
    /// assert_eq!(token.kind(), SyntaxKind::Fn);
    ///
    /// let token = lexer.peek_token(2).expect("token should exist since the file is not empty");
    ///
    /// assert_eq!(token.kind(), SyntaxKind::Whitespace);
    ///
    /// let token = lexer.peek_token(3).expect("token should exist since the file is not empty");
    ///
    /// assert_eq!(token.kind(), SyntaxKind::Identifier);
    /// ```
    #[must_use]
    pub fn peek_token(&self, mut n: usize) -> Option<SyntaxToken> {
        let mut start = self.location;

        let (kind, end) = loop {
            n -= 1;
            let (kind, end) = self.characters_to_token(start)?;

            if n == 0 {
                break (kind, end);
            }

            start = end;
        };

        Some(SyntaxToken::new(
            kind,
            SourceSpan::new(self.file.id(), start.offset(), end.offset()),
        ))
    }

    /// Convert a character to a [`SyntaxKind`].
    ///
    /// [`SyntaxKind`]: tantalum_syntax::SyntaxKind
    fn characters_to_token(&self, start: SourceLocation) -> Option<(SyntaxKind, SourceLocation)> {
        let mut current = start;

        let c = self.at_location(current)?;
        current = current.next_by(c.len_utf8());

        let token = match c {
            '(' => SyntaxKind::LParen,
            ')' => SyntaxKind::RParen,
            '{' => SyntaxKind::LBrace,
            '}' => SyntaxKind::RBrace,
            '[' => SyntaxKind::LBracket,
            ']' => SyntaxKind::RBracket,
            '<' => SyntaxKind::LAngle,
            '>' => SyntaxKind::RAngle,
            ',' => SyntaxKind::Comma,
            ':' => SyntaxKind::Colon,
            ';' => SyntaxKind::Semicolon,
            '.' => SyntaxKind::Dot,
            '&' => SyntaxKind::Ampersand,
            '|' => SyntaxKind::Pipe,
            '^' => SyntaxKind::Caret,
            '+' => SyntaxKind::Plus,
            '-' => SyntaxKind::Minus,
            '*' => SyntaxKind::Star,
            '/' => SyntaxKind::Slash,
            '\\' => SyntaxKind::Backslash,
            '%' => SyntaxKind::Percent,
            '=' => SyntaxKind::Equals,
            '!' => SyntaxKind::Bang,
            '~' => SyntaxKind::Tilde,
            '?' => SyntaxKind::Question,
            '@' => SyntaxKind::At,
            '0'..='9' => SyntaxKind::Digit,
            '_' | 'a'..='z' | 'A'..='Z' => {
                while let Some(c) = self.at_location(current) {
                    match c {
                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                            current = current.next_by(c.len_utf8());
                        }
                        _ => break,
                    }
                }

                match self.file.read_span(SourceSpan::new(
                    self.file.id(),
                    self.location.offset(),
                    current.offset(),
                ))? {
                    "fn" => SyntaxKind::Fn,
                    "let" => SyntaxKind::Let,
                    "return" => SyntaxKind::Return,
                    "if" => SyntaxKind::If,
                    "else" => SyntaxKind::Else,
                    "true" => SyntaxKind::True,
                    "false" => SyntaxKind::False,
                    _ => SyntaxKind::Identifier,
                }
            }
            c if c.is_whitespace() => SyntaxKind::Whitespace,
            _ => SyntaxKind::Other,
        };

        Some((token, current))
    }
}

#[test]
fn keywords() {
    use crate::Lexer;
    use tantalum_source::SourceFileCollection;
    use tantalum_syntax::SyntaxKind;

    let mut files = SourceFileCollection::new();

    let file_id = files.add_file(
        "file.tan".to_string(),
        "fn let return if else true false".to_string(),
    );

    let file = files
        .get_file(file_id)
        .expect("file should exist since it was just added");

    let mut lexer = Lexer::new(file);

    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        if token.kind() == SyntaxKind::Whitespace {
            continue;
        }
        tokens.push(token);
    }

    assert_eq!(tokens.len(), 7);

    assert_eq!(tokens[0].kind(), SyntaxKind::Fn);
    assert_eq!(tokens[1].kind(), SyntaxKind::Let);
    assert_eq!(tokens[2].kind(), SyntaxKind::Return);
    assert_eq!(tokens[3].kind(), SyntaxKind::If);
    assert_eq!(tokens[4].kind(), SyntaxKind::Else);
    assert_eq!(tokens[5].kind(), SyntaxKind::True);
    assert_eq!(tokens[6].kind(), SyntaxKind::False);
}

#[test]
fn identifiers() {
    use crate::Lexer;
    use tantalum_source::SourceFileCollection;
    use tantalum_syntax::SyntaxKind;

    let mut files = SourceFileCollection::new();

    let file_id = files.add_file(
        "file.tan".to_string(),
        "a A _ a1 A1 _1 _a _A __AA01".to_string(),
    );

    let file = files
        .get_file(file_id)
        .expect("file should exist since it was just added");

    let mut lexer = Lexer::new(file);

    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        if token.kind() == SyntaxKind::Whitespace {
            continue;
        }
        tokens.push(token);
    }

    assert_eq!(tokens.len(), 9);

    assert_eq!(tokens[0].kind(), SyntaxKind::Identifier);
    assert_eq!(tokens[1].kind(), SyntaxKind::Identifier);
    assert_eq!(tokens[2].kind(), SyntaxKind::Identifier);
    assert_eq!(tokens[3].kind(), SyntaxKind::Identifier);
    assert_eq!(tokens[4].kind(), SyntaxKind::Identifier);
    assert_eq!(tokens[5].kind(), SyntaxKind::Identifier);
    assert_eq!(tokens[6].kind(), SyntaxKind::Identifier);
    assert_eq!(tokens[7].kind(), SyntaxKind::Identifier);
    assert_eq!(tokens[8].kind(), SyntaxKind::Identifier);
}
