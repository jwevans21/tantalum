//! # Tantalum Span
//!
//! Provides a span to locate the positions and ranges of tokens in a file.

use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    num::Saturating,
    ops::{Deref, Range},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Location<'file_name> {
    /// The name of the file that this location is in
    #[cfg_attr(feature = "serde", serde(borrow))]
    file_name: &'file_name str,
    /// The byte of the location in the file
    position: Saturating<usize>,
    /// The line number of the location in the file
    line: Saturating<usize>,
    /// The column number of the location in the file
    column: Saturating<usize>,
}

impl<'file_name> Location<'file_name> {
    #[must_use]
    #[inline]
    pub fn new(file_name: &'file_name str) -> Self {
        return Self {
            file_name,
            position: Saturating(0),
            line: Saturating(1),
            column: Saturating(1),
        };
    }

    #[must_use]
    #[inline]
    pub fn new_at(file_name: &'file_name str, position: usize, line: usize, column: usize) -> Self {
        return Self {
            file_name,
            position: Saturating(position),
            line: Saturating(line),
            column: Saturating(column),
        };
    }

    #[inline]
    pub fn advance(&mut self, character: char) {
        if character == '\n' {
            self.line += 1;
            self.column = Saturating(1);
        } else {
            self.column += 1;
        }

        self.position += character.len_utf8();
    }

    #[must_use]
    #[inline]
    pub fn file_name(&self) -> &'file_name str {
        return self.file_name;
    }

    #[must_use]
    #[inline]
    pub fn position(&self) -> usize {
        return self.position.0;
    }

    #[must_use]
    #[inline]
    pub fn line(&self) -> usize {
        return self.line.0;
    }

    #[must_use]
    #[inline]
    pub fn column(&self) -> usize {
        return self.column.0;
    }

    #[must_use]
    #[inline]
    pub fn range(&self, other: &Self) -> Range<usize> {
        return (self.position())..(other.position());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span<'file_name> {
    /// The starting location of the span
    #[cfg_attr(feature = "serde", serde(borrow))]
    start: Location<'file_name>,
    /// The ending location of the span
    end: Location<'file_name>,
}

impl<'file_name> Span<'file_name> {
    #[must_use]
    #[inline]
    pub fn new(start: Location<'file_name>, end: Location<'file_name>) -> Self {
        return Self { start, end };
    }

    #[must_use]
    #[inline]
    pub fn file_name(&self) -> &'file_name str {
        return self.start.file_name();
    }

    #[must_use]
    #[inline]
    pub fn range(&self) -> Range<usize> {
        return self.start.range(&self.end);
    }

    #[must_use]
    #[inline]
    pub fn start(&self) -> Location<'file_name> {
        return self.start;
    }

    #[must_use]
    #[inline]
    pub fn end(&self) -> Location<'file_name> {
        return self.end;
    }

    #[must_use]
    #[inline]
    pub fn line(&self) -> usize {
        return self.start.line();
    }

    #[must_use]
    #[inline]
    pub fn column(&self) -> usize {
        return self.start.column();
    }
}

impl Display for Span<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return write!(f, "{}..{}", self.start.position(), self.end.position());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Spanned<'file_name, T: Debug + Clone + PartialEq + Eq> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    span: Span<'file_name>,
    data: T,
}

impl<'file_name, T> Spanned<'file_name, T>
where
    T: Debug + Clone + PartialEq + Eq,
{
    #[must_use]
    #[inline]
    pub fn new(span: Span<'file_name>, data: T) -> Self {
        return Self { span, data };
    }

    #[must_use]
    #[inline]
    pub fn spanning(start: Location<'file_name>, end: Location<'file_name>, data: T) -> Self {
        return Self::new(Span::new(start, end), data);
    }

    #[must_use]
    #[inline]
    pub fn join_spans(start: Span<'file_name>, end: Span<'file_name>, data: T) -> Self {
        return Self::new(Span::new(start.start, end.end), data);
    }

    pub fn map<U, F>(self, f: F) -> Spanned<'file_name, U>
    where
        U: Debug + Clone + PartialEq + Eq,
        F: FnOnce(T) -> U,
    {
        return Spanned::new(self.span, f(self.data));
    }

    #[must_use]
    #[inline]
    pub fn start(&self) -> Location<'file_name> {
        return self.span.start();
    }

    #[must_use]
    #[inline]
    pub fn end(&self) -> Location<'file_name> {
        return self.span.end();
    }

    #[must_use]
    #[inline]
    pub fn data(&self) -> &T {
        return &self.data;
    }

    #[must_use]
    #[inline]
    pub fn span(&self) -> Span<'file_name> {
        return self.span;
    }

    #[must_use]
    #[inline]
    pub fn range(&self) -> Range<usize> {
        return self.span.range();
    }

    #[must_use]
    #[inline]
    pub fn line(&self) -> usize {
        return self.span.line();
    }

    #[must_use]
    #[inline]
    pub fn column(&self) -> usize {
        return self.span.column();
    }

    #[must_use]
    #[inline]
    pub fn file_name(&self) -> &'file_name str {
        return self.span.file_name();
    }
}

impl<T> Copy for Spanned<'_, T> where T: Copy + Debug + Clone + PartialEq + Eq {}

impl<T> AsRef<T> for Spanned<'_, T>
where
    T: Debug + Clone + PartialEq + Eq,
{
    #[inline]
    fn as_ref(&self) -> &T {
        return &self.data;
    }
}

impl<T> Deref for Spanned<'_, T>
where
    T: Debug + Clone + PartialEq + Eq,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        return &self.data;
    }
}

impl<T> Display for Spanned<'_, T>
where
    T: Display + Debug + Clone + PartialEq + Eq,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return write!(f, "{} @ {}", self.data, self.span);
    }
}
