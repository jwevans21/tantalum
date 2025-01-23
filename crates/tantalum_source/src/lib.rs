//! # Tantalum Source
//!
//! The representation of a source file in Tantalum.
//!
//! Also contains associated types and utilities for working with source files.

use std::collections::HashMap;
use std::io::Read;

/// A source file in Tantalum.
///
/// Used to allow for multiple source files to be used in a single compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SourceFileId(usize);

impl core::fmt::Display for SourceFileId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl SourceFileId {
    /// Create a new `SourceFileId` from a `usize`.
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Get the next `SourceFileId` after this one.
    #[must_use]
    pub const fn next_id(self) -> Self {
        Self(self.0 + 1)
    }
}

/// A range of characters in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    /// The source file that this span is in.
    file: SourceFileId,
    /// The start of the span (in bytes).
    start: usize,
    /// The end of the span (in bytes).
    end: usize,
}

impl SourceSpan {
    /// Create a new `SourceSpan` from a `SourceFileId`, start and end.
    ///
    /// # Panics
    ///
    /// Panics if `start` is greater than `end`.
    #[must_use]
    pub const fn new(file: SourceFileId, start: usize, end: usize) -> Self {
        assert!(start <= end, "start must be less than or equal to end");
        Self { file, start, end }
    }

    /// Get the file that the span is in.
    #[must_use]
    pub const fn file(&self) -> SourceFileId {
        self.file
    }

    /// Get the start of the span.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Get the end of the span.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Get the length of the span.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if the span is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Get the range of the span.
    #[must_use]
    pub const fn range(&self) -> core::ops::Range<usize> {
        self.start..self.end
    }

    /// Merge a list of spans into a single span.
    ///
    /// # Panics
    ///
    /// - if the list of spans is empty.
    /// - if the spans are in different files.
    #[must_use]
    pub fn merge(spans: &[Self]) -> Self {
        assert!(!spans.is_empty(), "cannot merge an empty list of spans");
        assert!(
            spans.iter().all(|span| span.file == spans[0].file),
            "spans must be in the same file"
        );

        let start = spans.iter().map(Self::start).min().unwrap();
        let end = spans.iter().map(Self::end).max().unwrap();
        Self::new(spans[0].file(), start, end)
    }
}

impl core::fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}..{}", self.file, self.start, self.end)
    }
}

/// A location in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    /// The source file that this location is in.
    file: SourceFileId,
    /// The offset of the location (in bytes).
    offset: usize,
}

impl SourceLocation {
    /// Create a new `SourceLocation` from a `SourceFileId` and an offset.
    #[must_use]
    pub const fn new(file: SourceFileId, offset: usize) -> Self {
        Self { file, offset }
    }

    /// Get the file that the location is in.
    #[must_use]
    pub const fn file(&self) -> SourceFileId {
        self.file
    }

    /// Get the offset of the location.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Get the location after this one.
    #[must_use]
    pub const fn next_by(self, offset: usize) -> Self {
        Self {
            offset: self.offset + offset,
            ..self
        }
    }
}

/// A source file in Tantalum.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    /// The ID of the source file.
    id: SourceFileId,
    /// The name of the source file.
    name: String,
    /// The contents of the source file.
    contents: String,
}

impl SourceFile {
    /// Create a new `SourceFile` from an ID, name and contents.
    #[must_use]
    pub const fn new(id: SourceFileId, name: String, contents: String) -> Self {
        Self { id, name, contents }
    }

    /// Create a new `SourceFile` from a path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read.
    pub fn from_path<P: AsRef<std::path::Path>>(
        id: SourceFileId,
        path: P,
    ) -> std::io::Result<Self> {
        let name = path.as_ref().to_string_lossy().to_string();

        let file = std::fs::File::open(path)?;
        let mut reader = std::io::BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        Ok(Self::new(id, name, contents))
    }

    /// Get the ID of the source file.
    #[must_use]
    pub const fn id(&self) -> SourceFileId {
        self.id
    }

    /// Get the name of the source file.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the contents of the source file.
    #[must_use]
    pub fn contents(&self) -> &str {
        &self.contents
    }

    /// Create a `SourceLocation` at the start of the source file.
    #[must_use]
    pub fn location_start(&self) -> SourceLocation {
        SourceLocation::new(self.id, 0)
    }

    /// Get a slice of the contents of the source file.
    #[must_use]
    pub fn read_span(&self, span: SourceSpan) -> Option<&str> {
        self.contents.get(span.range())
    }

    /// Get the next character in the source file.
    #[must_use]
    pub fn read_char(&self, location: SourceLocation) -> Option<char> {
        self.contents.get(location.offset..)?.chars().next()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFileCollection {
    next_id: SourceFileId,
    files: HashMap<SourceFileId, SourceFile>,
}

impl SourceFileCollection {
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_id: SourceFileId::new(0),
            files: HashMap::new(),
        }
    }

    #[must_use]
    pub fn add_file(&mut self, name: String, contents: String) -> SourceFileId {
        let id = self.next_id;
        self.next_id = self.next_id.next_id();

        let file = SourceFile::new(id, name, contents);
        self.files.insert(id, file);

        id
    }

    /// Add a file to the context from a path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read.
    pub fn add_file_from_path<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> std::io::Result<SourceFileId> {
        let id = self.next_id;
        self.next_id = self.next_id.next_id();

        let file = SourceFile::from_path(id, path)?;
        self.files.insert(id, file);

        Ok(id)
    }

    /// Get a file from the context.
    #[must_use]
    pub fn get_file(&self, id: SourceFileId) -> Option<&SourceFile> {
        self.files.get(&id)
    }

    #[must_use]
    pub fn get_file_name(&self, id: SourceFileId) -> Option<&str> {
        self.get_file(id).map(SourceFile::name)
    }

    /// Get the contents of a source file.
    #[must_use]
    pub fn get_file_contents(&self, id: SourceFileId) -> Option<&str> {
        self.get_file(id).map(SourceFile::contents)
    }

    /// Get the start location of a source file.
    #[must_use]
    pub fn get_location_start(&self, id: SourceFileId) -> Option<SourceLocation> {
        self.get_file(id).map(SourceFile::location_start)
    }

    /// Get a slice of the contents of the source file.
    #[must_use]
    pub fn read_span(&self, span: SourceSpan) -> Option<&str> {
        self.get_file(span.file())
            .and_then(|file| file.read_span(span))
    }

    /// Get the next character in the source file.
    #[must_use]
    pub fn read_char(&self, location: SourceLocation) -> Option<char> {
        self.get_file(location.file())
            .and_then(|file| file.read_char(location))
    }
}

impl Default for SourceFileCollection {
    fn default() -> Self {
        Self::new()
    }
}
