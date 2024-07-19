// A table of contents entry is a tuple of line offset and the line itself
use std::fmt;
pub struct TocEntry {
    offset: usize,
    line: String,
}

impl TocEntry {
    pub fn new(offset: usize, line: String) -> TocEntry {
        TocEntry { offset, line }
    }
}

impl fmt::Display for TocEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line offset {:3?} - {}", self.offset, self.line)
    }
}
