use tantalum_lexer::Lexer;
use tantalum_source::{SourceFile, SourceLocation, SourceSpan};
use tantalum_syntax::{SyntaxKind, SyntaxNode, SyntaxToken, SyntaxTree};

#[cfg(test)]
mod tests;

mod expressions;
mod identifiers;
mod items;
mod paths;
mod statements;
mod types;
mod whitespace;

/// The different events that can be emitted when parsing.
///
/// These will determine if a new tree should be opened, closed, or a token should be added to the
/// currently opened tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ParserEvent {
    /// Open a new token of the specified [`SyntaxKind`].
    ///
    /// This will be resolved into a [`SyntaxTree`] when closed.
    ///
    /// [`SyntaxKind`]: tantalum_syntax::SyntaxKind
    /// [`SyntaxTree`]: tantalum_syntax::SyntaxTree
    Open {
        kind: SyntaxKind,
        start: SourceLocation,
    },
    /// Close the current open [`SyntaxTree`].
    ///
    /// [`SyntaxTree`]: tantalum_syntax::SyntaxTree
    Close { end: SourceLocation },
    /// Add a terminal token to the current open [`SyntaxTree`].
    ///
    /// This will be transformed into a [`SyntaxNode::Token`] when the tree is constructed.
    ///
    /// [`SyntaxTree`]: tantalum_syntax::SyntaxTree
    /// [`SyntaxNode::Token`]: tantalum_syntax::SyntaxNode::Token
    Token { kind: SyntaxKind, span: SourceSpan },
}

/// A mark for an event that has been opened. Used by the parser to then either close or adjust the
/// currently opened tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct MarkOpened {
    index: usize,
}

/// A mark for an event that has been closed. Used to update the tree structure based on later
/// tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct MarkClosed {
    open_index: usize,
}

/// The parser for the Tantalum language.
///
/// Controls the parsing by emitting events that are then used to construct the syntax tree.
///
/// All parsing functions are implemented as functions that take a `&mut Parser`. The exposed
/// interface are the [`Parser::parse`] and [`Parser::finish`] functions.
///
/// The interface of the parser is based on [Resilient LL Parsing Tutorial] which is licence under
/// [MIT OR Apache-2.0].
///
/// [Resilient LL Parsing Tutorial]: https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html
/// [MIT OR Apache-2.0]: https://matklad.github.io/about.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parser<'file> {
    /// The lexer to use for parsing.
    lexer: Lexer<'file>,
    /// The source file being parsed.
    source: &'file SourceFile,
    /// The events emitted by the parser.
    events: Vec<ParserEvent>,
}

impl<'file> Parser<'file> {
    /// Create a new [`Parser`] from a [`Lexer`].
    ///
    /// [`Lexer`]: tantalum_lexer::Lexer
    #[must_use]
    pub fn new(lexer: Lexer<'file>) -> Self {
        let source = lexer.source();
        Self {
            lexer,
            source,
            events: Vec::new(),
        }
    }

    /// Get a string of the specified `SyntaxToken` from the source file.
    #[must_use]
    pub fn get_span_str(&self, span: &SourceSpan) -> Option<&str> {
        self.source.read_span(*span)
    }

    /// Parse the source file.
    ///
    /// This just runs the `file` parser.
    ///
    /// To get the resulting `SyntaxTree`, call `finish`.
    ///
    /// ```rust
    /// use tantalum_source::SourceFileCollection;
    /// use tantalum_lexer::Lexer;
    /// use tantalum_parser::Parser;
    ///
    /// let mut files = SourceFileCollection::new();
    /// let file_id = files.add_file("main.tan".to_string(), "fn main() {}".to_string());
    /// let file = files.get_file(file_id).expect("should have file, since it was just added");
    ///
    /// let mut parser = Parser::new(Lexer::new(file));
    ///
    /// parser.parse();
    ///
    /// let tree = parser.finish();
    /// ```
    pub fn parse(&mut self) {
        use crate::items::file;
        let _ = file(self);
    }

    /// Finish the parser and return the resulting `SyntaxTree`.
    ///
    /// # Panics
    ///
    /// - if there are unmatched open events.
    /// - if there are unmatched close events.
    /// - if there are unexpected token events (a token must be inside a tree).
    ///
    /// ```rust
    /// use tantalum_source::SourceFileCollection;
    /// use tantalum_lexer::Lexer;
    /// use tantalum_parser::Parser;
    ///
    /// let mut files = SourceFileCollection::new();
    /// let file_id = files.add_file("main.tan".to_string(), "fn main() {}".to_string());
    /// let file = files.get_file(file_id).expect("should have file, since it was just added");
    ///
    /// let mut parser = Parser::new(Lexer::new(file));
    ///
    /// parser.parse();
    ///
    /// let tree = parser.finish();
    /// ```
    #[must_use]
    pub fn finish(self) -> SyntaxTree {
        let mut stack = Vec::new();

        for event in self.events {
            match event {
                ParserEvent::Open { kind, start } => {
                    stack.push(SyntaxTree::new(
                        kind,
                        SourceSpan::new(start.file(), start.offset(), start.offset()),
                    ));
                }
                ParserEvent::Close { end } => {
                    let mut tree = stack.pop().expect("unmatched close event");
                    tree.extend_span(end);

                    if let Some(parent) = stack.last_mut() {
                        parent.add_child(SyntaxNode::Tree(tree));
                    } else {
                        return tree;
                    }
                }
                ParserEvent::Token { kind, span } => {
                    let token = SyntaxToken::new(kind, span);
                    if let Some(parent) = stack.last_mut() {
                        parent.add_child(SyntaxNode::Token(token));
                    } else {
                        panic!("unexpected token event, a token must be inside a tree");
                    }
                }
            }
        }

        assert_eq!(stack.len(), 1, "unmatched open event");
        stack.pop().expect("unmatched open event")
    }

    /// Open a new token of the specified [`SyntaxKind`].
    ///
    /// This resolves into a [`SyntaxTree`] when closed using [`Parser::close`].
    ///
    /// [`SyntaxKind`]: tantalum_syntax::SyntaxKind
    /// [`SyntaxTree`]: tantalum_syntax::SyntaxTree
    /// [`Parser::close`]: Parser::close
    #[must_use]
    pub(crate) fn open(&mut self, kind: SyntaxKind) -> MarkOpened {
        let start = self.lexer.location();
        self.events.push(ParserEvent::Open { kind, start });
        MarkOpened {
            index: self.events.len() - 1,
        }
    }

    /// Open a new token of the specified [`SyntaxKind`] before the specified mark.
    ///
    /// Useful for when adjusting the tree based on later tokens.
    ///
    /// Similarly to [`Parser::open`], this resolves into a [`SyntaxTree`] when closed using [`Parser::close`].
    ///
    /// [`SyntaxKind`]: tantalum_syntax::SyntaxKind
    /// [`SyntaxTree`]: tantalum_syntax::SyntaxTree
    /// [`Parser::open`]: Parser::open
    /// [`Parser::close`]: Parser::close
    pub(crate) fn open_before(&mut self, close: MarkClosed, kind: SyntaxKind) -> MarkOpened {
        match self.events.get(close.open_index) {
            Some(ParserEvent::Open { start, .. }) => {
                self.events.insert(
                    close.open_index,
                    ParserEvent::Open {
                        kind,
                        start: *start,
                    },
                );
                MarkOpened {
                    index: close.open_index,
                }
            }
            _ => panic!("can only open before other open events"),
        }
    }

    /// Adjust the kind of the current open token.
    ///
    /// # Panics
    ///
    /// - if the mark is not an open event.
    pub(crate) fn adjust(&mut self, mark: MarkOpened, updated_kind: SyntaxKind) {
        if let Some(ParserEvent::Open { kind, .. }) = self.events.get_mut(mark.index) {
            *kind = updated_kind;
        } else {
            panic!("can only adjust open events");
        }
    }

    /// Add a token to the current open tree.
    ///
    /// When the tree is constructed the token will be transformed into a `SyntaxNode::Token`.
    pub(crate) fn token(&mut self, token: SyntaxToken) -> MarkClosed {
        self.events.push(ParserEvent::Token {
            kind: token.kind(),
            span: token.span(),
        });

        MarkClosed {
            open_index: self.events.len() - 1,
        }
    }

    /// Close the current open token.
    #[must_use]
    pub(crate) fn close(&mut self, mark: MarkOpened) -> MarkClosed {
        let end = self.lexer.location();
        self.events.push(ParserEvent::Close { end });
        MarkClosed {
            open_index: mark.index,
        }
    }

    /// Error with the specified message.
    pub(crate) fn error(&mut self, mark: MarkOpened, message: &str) -> MarkClosed {
        eprintln!("error: {message}");
        self.adjust(mark, SyntaxKind::Error);
        self.close(mark)
    }

    /// Peek the `n` tokens ahead.
    ///
    /// Does not consume the tokens or adjust the state of the lexer.
    #[must_use]
    pub(crate) fn peek(&self, n: usize) -> Option<SyntaxToken> {
        self.lexer.peek_token(n)
    }

    /// Consume the current [`SyntaxToken`], if it exists add it to the tree structure as
    /// a [`SyntaxNode::Token`].
    ///
    /// Returns the consumed token if it exists.
    ///
    /// [`SyntaxToken`]: tantalum_syntax::SyntaxToken
    /// [`SyntaxNode::Token`]: tantalum_syntax::SyntaxNode::Token
    #[must_use]
    pub(crate) fn consume(&mut self) -> Option<SyntaxToken> {
        let token = self.lexer.next_token()?;

        self.token(token);

        Some(token)
    }

    /// Advance the lexer and return the next token.
    ///
    /// This does not add the token to the tree structure.
    pub(crate) fn advance(&mut self) -> Option<SyntaxToken> {
        self.lexer.next_token()
    }

    /// Check if the parser is at the end of the source file.
    pub(crate) fn at_end(&self) -> bool {
        self.peek(1).is_none()
    }

    /// Check if the parser is at the specified [`SyntaxKind`].
    ///
    /// [`SyntaxKind`]: tantalum_syntax::SyntaxKind
    #[must_use]
    pub(crate) fn is_at(&self, kind: SyntaxKind) -> bool {
        self.peek(1).is_some_and(|token| token.kind() == kind)
    }

    /// Check if the parser is at any of the specified [`SyntaxKind`]s.
    ///
    /// [`SyntaxKind`]: tantalum_syntax::SyntaxKind
    #[must_use]
    pub(crate) fn is_at_any(&self, kinds: &[SyntaxKind]) -> bool {
        self.peek(1)
            .is_some_and(|token| kinds.contains(&token.kind()))
    }

    /// Check if the parser is at the specified [`SyntaxKind`] and consume it if it is.
    ///
    /// Returns the consumed token and adds it to the tree structure if it exists.
    ///
    /// [`SyntaxKind`]: tantalum_syntax::SyntaxKind
    #[must_use]
    pub(crate) fn consume_if(&mut self, kind: SyntaxKind) -> Option<SyntaxToken> {
        if self.is_at(kind) {
            self.consume()
        } else {
            None
        }
    }

    /// Check if the parser is at any of the specified [`SyntaxKind`]s and consume it if it is.
    ///
    /// Returns the consumed token and adds it to the tree structure if it exists.
    ///
    /// [`SyntaxKind`]: tantalum_syntax::SyntaxKind
    #[must_use]
    pub(crate) fn consume_if_any(&mut self, kinds: &[SyntaxKind]) -> Option<SyntaxToken> {
        if self.is_at_any(kinds) {
            self.consume()
        } else {
            None
        }
    }

    /// Collect the tokens into a `Vec` while the specified function returns `true`.
    ///
    /// Advances the parser, but does not add the tokens to the tree structure.
    pub(crate) fn advance_while<F>(&mut self, mut f: F) -> Vec<SyntaxToken>
    where
        F: FnMut(SyntaxKind) -> bool,
    {
        let mut tokens = Vec::new();

        while let Some(token) = self.peek(1) {
            if f(token.kind()) {
                tokens.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        tokens
    }
}
