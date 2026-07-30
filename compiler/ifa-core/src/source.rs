use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceText {
    name: String,
    text: String,
    line_starts: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl SourceText {
    /// Creates a new source text value.
    ///
    /// Lines and columns are 1-based.
    /// Offsets are UTF-8 byte offsets.
    pub fn new<N, T>(name: N, text: T) -> Self
    where
        N: Into<String>,
        T: Into<String>,
    {
        let text = text.into();
        let line_starts = Self::compute_line_starts(&text);

        Self {
            name: name.into(),
            text,
            line_starts,
        }
    }

    /// Returns the logical source name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the full source text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the source length in UTF-8 bytes.
    pub fn len_bytes(&self) -> usize {
        self.text.len()
    }

    /// Returns true when the source contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the number of logical lines.
    ///
    /// An empty source contains one logical line.
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Returns the byte offset at which a line starts.
    ///
    /// Lines are 1-based.
    pub fn line_start(&self, line: usize) -> Option<usize> {
        self.line_starts.get(line.checked_sub(1)?).copied()
    }

    /// Returns the text of a line without its line terminator.
    ///
    /// Lines are 1-based.
    pub fn line_text(&self, line: usize) -> Option<&str> {
        let index = line.checked_sub(1)?;
        let start = *self.line_starts.get(index)?;

        let end = self
            .line_starts
            .get(index + 1)
            .copied()
            .unwrap_or(self.text.len());

        let slice = &self.text[start..end];

        Some(slice.trim_end_matches(&['\r', '\n'][..]))
    }

    /// Returns a UTF-8 slice for the given byte range.
    pub fn slice(&self, range: Range<usize>) -> Option<&str> {
        if range.start > range.end || range.end > self.text.len() {
            return None;
        }

        self.text.get(range)
    }

    /// Converts a byte offset into a 1-based line/column position.
    pub fn position_of(&self, offset: usize) -> Option<Position> {
        if offset > self.text.len() || !self.text.is_char_boundary(offset) {
            return None;
        }

        let line_index = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .checked_sub(1)?;

        let line_start = self.line_starts[line_index];
        let column = self.text[line_start..offset].chars().count() + 1;

        Some(Position {
            offset,
            line: line_index + 1,
            column,
        })
    }

    fn compute_line_starts(text: &str) -> Vec<usize> {
        let mut starts = vec![0];

        for (index, byte) in text.bytes().enumerate() {
            if byte == b'\n' {
                starts.push(index + 1);
            }
        }

        starts
    }
}

#[cfg(test)]
mod tests {
    use super::SourceText;

    #[test]
    fn empty_source_has_one_line() {
        let source = SourceText::new("empty.ifsl", "");

        assert_eq!(source.line_count(), 1);
        assert_eq!(source.line_start(1), Some(0));
        assert_eq!(source.line_text(1), Some(""));
    }

    #[test]
    fn counts_lines() {
        let source = SourceText::new("example.ifsl", "alpha\nbeta\ngamma");

        assert_eq!(source.line_count(), 3);
        assert_eq!(source.line_start(1), Some(0));
        assert_eq!(source.line_start(2), Some(6));
        assert_eq!(source.line_start(3), Some(11));
    }

    #[test]
    fn returns_line_text_without_newline() {
        let source = SourceText::new("example.ifsl", "alpha\nbeta\n");

        assert_eq!(source.line_text(1), Some("alpha"));
        assert_eq!(source.line_text(2), Some("beta"));
        assert_eq!(source.line_text(3), Some(""));
    }

    #[test]
    fn converts_offset_to_position() {
        let source = SourceText::new("example.ifsl", "alpha\nbeta");

        assert_eq!(
            source.position_of(0),
            Some(super::Position {
                offset: 0,
                line: 1,
                column: 1
            })
        );

        assert_eq!(
            source.position_of(7),
            Some(super::Position {
                offset: 7,
                line: 2,
                column: 2
            })
        );
    }

    #[test]
    fn handles_unicode_columns() {
        let source = SourceText::new("unicode.ifsl", "Raúl");

        assert_eq!(
            source.position_of(5),
            Some(super::Position {
                offset: 5,
                line: 1,
                column: 5
            })
        );
    }

    #[test]
    fn rejects_invalid_utf8_boundary() {
        let source = SourceText::new("unicode.ifsl", "Raúl");

        // The 'ú' character occupies two UTF-8 bytes.
        assert_eq!(source.position_of(3), None);
    }

    #[test]
    fn slices_source() {
        let source = SourceText::new("example.ifsl", "abcdef");

        assert_eq!(source.slice(1..4), Some("bcd"));
    }

    #[test]
    fn rejects_invalid_slice() {
        let source = SourceText::new("example.ifsl", "abcdef");

        assert_eq!(source.slice(std::ops::Range { start: 4, end: 2 }), None);
        assert_eq!(source.slice(0..99), None);
    }
}
