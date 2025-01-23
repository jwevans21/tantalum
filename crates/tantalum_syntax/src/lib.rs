use std::fmt::Write;
use tantalum_source::{SourceFileCollection, SourceLocation, SourceSpan};

pub trait PrettyPrint {
    /// Pretty print the syntax token or tree.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the output fails.
    fn pretty_print(
        &self,
        f: &mut dyn Write,
        indent: usize,
        files: &SourceFileCollection,
    ) -> core::fmt::Result;
}

#[macro_export]
macro_rules! pretty {
    ($result:ident = $tree:ident $files:ident) => {
        let $result = {
            use $crate::PrettyPrint;
            let mut result = String::new();
            $tree
                .pretty_print(&mut result, 0, &$files)
                .expect("pretty print failed");
            result
        };
    };
}

/// A token in the Tantalum language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyntaxToken {
    kind: SyntaxKind,
    span: SourceSpan,
}

impl SyntaxToken {
    /// Create a new `SyntaxToken` from a `SyntaxKind` and `SourceSpan`.
    #[must_use]
    pub const fn new(kind: SyntaxKind, span: SourceSpan) -> Self {
        Self { kind, span }
    }

    /// Get the kind of the token.
    #[must_use]
    pub const fn kind(&self) -> SyntaxKind {
        self.kind
    }

    /// Get the span of the token.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }
}

impl PrettyPrint for SyntaxToken {
    fn pretty_print(
        &self,
        f: &mut dyn Write,
        indent: usize,
        files: &SourceFileCollection,
    ) -> core::fmt::Result {
        write!(f, "{:indent$}{:?} @ {}", "", self.kind, self.span)?;

        if let Some(contents) = files.read_span(self.span) {
            write!(f, " {contents:?}")?;
        } else {
            write!(f, " <unavailable>")?;
        }

        writeln!(f)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxTree {
    kind: SyntaxKind,
    span: SourceSpan,
    children: Vec<SyntaxNode>,
}

impl SyntaxTree {
    #[must_use]
    pub fn new(kind: SyntaxKind, span: SourceSpan) -> Self {
        Self {
            kind,
            span,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    #[must_use]
    pub fn span(&self) -> SourceSpan {
        self.span
    }

    #[must_use]
    pub fn children(&self) -> &[SyntaxNode] {
        &self.children
    }

    /// Extend the span of the tree to include the given location.
    ///
    /// # Panics
    ///
    /// - if the location is in a different file than the tree.
    /// - if the location is before the start of the tree.
    pub fn extend_span(&mut self, location: SourceLocation) {
        assert_eq!(
            self.span.file(),
            location.file(),
            "cannot extend span across files"
        );
        assert!(
            location.offset() >= self.span.start(),
            "cannot extend span before start"
        );

        self.span = SourceSpan::new(self.span.file(), self.span.start(), location.offset());
    }

    pub fn add_child(&mut self, child: SyntaxNode) {
        self.children.push(child);
    }
}

impl PrettyPrint for SyntaxTree {
    fn pretty_print(
        &self,
        f: &mut dyn Write,
        indent: usize,
        files: &SourceFileCollection,
    ) -> core::fmt::Result {
        write!(f, "{:indent$}{:?} @ {}", "", self.kind, self.span)?;

        writeln!(f)?;

        for child in &self.children {
            child.pretty_print(f, indent + 2, files)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxNode {
    Token(SyntaxToken),
    Tree(SyntaxTree),
}

impl SyntaxNode {
    #[must_use]
    pub fn token(&self) -> Option<&SyntaxToken> {
        match self {
            Self::Token(token) => Some(token),
            Self::Tree(_) => None,
        }
    }

    #[must_use]
    pub fn tree(&self) -> Option<&SyntaxTree> {
        match self {
            Self::Tree(tree) => Some(tree),
            Self::Token(_) => None,
        }
    }

    #[must_use]
    pub fn kind(&self) -> SyntaxKind {
        match self {
            Self::Token(token) => token.kind(),
            Self::Tree(tree) => tree.kind(),
        }
    }

    #[must_use]
    pub fn span(&self) -> SourceSpan {
        match self {
            Self::Token(token) => token.span(),
            Self::Tree(tree) => tree.span(),
        }
    }
}

impl PrettyPrint for SyntaxNode {
    fn pretty_print(
        &self,
        f: &mut dyn Write,
        indent: usize,
        files: &SourceFileCollection,
    ) -> core::fmt::Result {
        match self {
            Self::Token(token) => token.pretty_print(f, indent, files),
            Self::Tree(tree) => tree.pretty_print(f, indent, files),
        }
    }
}

/// The different kinds of syntax tokens and trees in the Tantalum language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    /// Error Tree or Token
    Error,
    ///////////////////////////////////////////////////////////////////////////
    // Basic Tokens
    ///////////////////////////////////////////////////////////////////////////
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `<`
    LAngle,
    /// `>`
    RAngle,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    /// `.`
    Dot,
    /// `&`
    Ampersand,
    /// `|`
    Pipe,
    /// `^`
    Caret,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `\`
    Backslash,
    /// `%`
    Percent,
    /// `=`
    Equals,
    /// `!`
    Bang,
    /// `~`
    Tilde,
    /// `?`
    Question,
    /// `@`
    At,
    /// `"`
    DoubleQuote,
    /// `'`
    SingleQuote,

    /// `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`
    Digit,
    /// [a-zA-Z]
    Letter,
    /// `_`
    Underscore,
    /// Any whitespace character
    Whitespace,

    /// Any other character
    Other,

    ///////////////////////////////////////////////////////////////////////////
    // Terminal Combinations
    ///////////////////////////////////////////////////////////////////////////
    /// `<<`
    ShiftLeft,
    /// `>>`
    ShiftRight,
    /// `==`
    Equality,
    /// `!=`
    Inequality,
    /// `<=`
    LessThanOrEqual,
    /// `>=`
    GreaterThanOrEqual,
    /// `&&`
    And,
    /// `||`
    Or,
    /// `.*`
    Deref,
    /// `.&`
    Ref,

    /// `::`
    PathSeparator,
    /// `..`
    DoubleDot,
    /// `...`
    TripleDot,

    Identifier,

    ///////////////////////////////////////////////////////////////////////////
    // Keywords For Items
    ///////////////////////////////////////////////////////////////////////////
    /// `fn`
    Fn,

    ///////////////////////////////////////////////////////////////////////////
    // Keywords For Statements
    ///////////////////////////////////////////////////////////////////////////
    /// `let`
    Let,
    /// `return`
    Return,

    ///////////////////////////////////////////////////////////////////////////
    // Keywords For Expressions
    ///////////////////////////////////////////////////////////////////////////
    /// `if`
    If,
    /// `else`
    Else,

    /// `true`
    True,
    /// `false`
    False,

    ///////////////////////////////////////////////////////////////////////////
    // Root Level Items
    ///////////////////////////////////////////////////////////////////////////
    /// The main root of the syntax tree.
    File,

    /// A function definition.
    Function,

    /// A Parameter List.
    ParameterList,

    /// A Parameter.
    Parameter,

    /// Representation of a function having variadic arguments.
    Variadic,

    ///////////////////////////////////////////////////////////////////////////
    // Paths
    ///////////////////////////////////////////////////////////////////////////
    /// A path (e.g. `::u8` or `m::test`).
    Path,
    PathSegment,

    ///////////////////////////////////////////////////////////////////////////
    // Types
    ///////////////////////////////////////////////////////////////////////////
    /// A named type (e.g. `u8` or `MyStruct`). Represented as a path.
    NamedType,
    /// Pointer type (e.g. `*u8` or `*MyStruct`).
    PointerType,
    /// Reference type (e.g. `&u8` or `&MyStruct`).
    ReferenceType,

    ///////////////////////////////////////////////////////////////////////////
    // Statements
    ///////////////////////////////////////////////////////////////////////////
    /// A let statement (e.g. `let x = 5;`).
    LetStatement,
    /// A return statement (e.g. `return 5;` or `return;`).
    ReturnStatement,
    /// An expression statement (e.g. `5 + 5;`).
    ExpressionStatement,

    ///////////////////////////////////////////////////////////////////////////
    // Expressions
    ///////////////////////////////////////////////////////////////////////////
    /// A curly brace block.
    Block,

    IfExpression,
    Condition,

    Expression,
    BinaryExpression,
    PrefixExpression,
    PostfixExpression,

    Variable,
    Literal,
    IntegerLiteral,
    FloatLiteral,

    Grouping,
}
