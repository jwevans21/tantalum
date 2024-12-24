//! # Tantalum Span
//!
//! Provides a span to locate the positions and ranges of tokens in a file.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::Saturating,
    ops::Range,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span<'file_name> {
    /// The name of the file that this span is in
    #[cfg_attr(feature = "serde", serde(borrow))]
    file_name: &'file_name str,
    /// The start byte of the span in the file
    start: Saturating<usize>,
    /// The end byte of the span in the file
    end: Saturating<usize>,
    /// The line number of the span in the file
    line: Saturating<usize>,
    /// The column number of the span in the file
    column: Saturating<usize>,
}

impl<'file_name> Span<'file_name> {
    #[must_use]
    #[inline]
    pub fn new(
        file_name: &'file_name str,
        start: usize,
        end: usize,
        line: usize,
        column: usize,
    ) -> Self {
        return Self {
            file_name,
            start: Saturating(start),
            end: Saturating(end),
            line: Saturating(line),
            column: Saturating(column),
        };
    }

    #[must_use]
    #[inline]
    pub fn extend(mut self, other: &Self) -> Self {
        self.end = other.end;
        return self;
    }

    #[inline]
    pub fn advance(&mut self, character: char) {
        if character == '\n' {
            self.line += 1;
            self.column = Saturating(0);
        } else {
            self.column += 1;
        }

        self.start += character.len_utf8();
        self.end += character.len_utf8();
    }

    #[must_use]
    #[inline]
    pub fn contains(&self, other: &Self) -> bool {
        return self.file_name == other.file_name
            && self.start <= other.start
            && self.end >= other.end;
    }

    #[must_use]
    #[inline]
    pub fn file_name(&self) -> &'file_name str {
        return self.file_name;
    }

    #[must_use]
    #[inline]
    pub fn range(&self) -> Range<usize> {
        return (self.start.0)..(self.end.0);
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
}

impl Display for Span<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return write!(f, "{}:{}:{}", self.file_name, self.line, self.column);
    }
}
