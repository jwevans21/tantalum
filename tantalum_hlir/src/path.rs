#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path(Vec<PathSegment>);

impl Path {
    #[must_use]
    pub fn new(segments: Vec<PathSegment>) -> Self {
        Self(segments)
    }

    pub fn push(&mut self, segment: PathSegment) {
        self.0.push(segment);
    }

    pub fn pop(&mut self) -> Option<PathSegment> {
        self.0.pop()
    }

    #[must_use]
    pub fn segments(&self) -> &[PathSegment] {
        &self.0
    }
}

impl From<&str> for Path {
    fn from(name: &str) -> Self {
        Self(vec![PathSegment::from(name)])
    }
}

impl From<String> for Path {
    fn from(name: String) -> Self {
        Self(vec![PathSegment::from(name)])
    }
}

impl core::fmt::Debug for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Path({self})")
    }
}

impl core::fmt::Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for segment in self.segments() {
            write!(f, "::{segment}")?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathSegment {
    pub name: String,
}

impl PathSegment {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl From<&str> for PathSegment {
    fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
}

impl From<String> for PathSegment {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl core::fmt::Debug for PathSegment {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl core::fmt::Display for PathSegment {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.name)
    }
}
