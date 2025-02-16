use super::BytePos;

/// Helper struct to convert BytePos into line numbers.
///
/// # Examples
/// ```
/// use grammarsmith::position::{LineOffsets, BytePos};
/// let offsets = LineOffsets::new("abc\ndef");
/// assert_eq!(offsets.line(BytePos(0)), 1);
/// assert_eq!(offsets.line(BytePos(1)), 1);
/// assert_eq!(offsets.line(BytePos(4)), 2);
/// assert_eq!(offsets.line(BytePos(3)), 1);
/// assert_eq!(offsets.line(BytePos(7)), 2);
/// ```
pub struct LineOffsets {
    offsets: Vec<usize>,
    len: usize,
}

impl LineOffsets {
    pub fn new(data: &str) -> Self {
        let mut offsets = vec![0];
        let len = data.len();

        for (i, val) in data.bytes().enumerate() {
            if val == b'\n' {
                offsets.push(i + 1);
            }
        }

        Self { offsets, len }
    }

    /// Find the line number for a given BytePos
    pub fn line(&self, pos: BytePos) -> usize {
        let offset = pos.0;

        assert!(offset <= self.len);

        match self.offsets.binary_search(&offset) {
            Ok(line) => line + 1,
            Err(line) => line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let offsets = LineOffsets::new("");
        assert_eq!(offsets.line(BytePos(0)), 1);
    }

    #[test]
    fn test_single_line() {
        let offsets = LineOffsets::new("hello world");
        assert_eq!(offsets.line(BytePos(0)), 1);
        assert_eq!(offsets.line(BytePos(5)), 1);
        assert_eq!(offsets.line(BytePos(10)), 1);
    }

    #[test]
    fn test_multiple_lines() {
        let offsets = LineOffsets::new("line1\nline2\nline3");
        assert_eq!(offsets.line(BytePos(0)), 1); // start of line1
        assert_eq!(offsets.line(BytePos(5)), 1); // end of line1
        assert_eq!(offsets.line(BytePos(6)), 2); // start of line2
        assert_eq!(offsets.line(BytePos(11)), 2); // end of line2
        assert_eq!(offsets.line(BytePos(12)), 3); // start of line3
        assert_eq!(offsets.line(BytePos(16)), 3); // end of line3
    }

    #[test]
    fn test_trailing_newline() {
        let offsets = LineOffsets::new("hello\n");
        assert_eq!(offsets.line(BytePos(0)), 1);
        assert_eq!(offsets.line(BytePos(5)), 1);
        assert_eq!(offsets.line(BytePos(6)), 2);
    }

    #[test]
    fn test_multiple_consecutive_newlines() {
        let offsets = LineOffsets::new("a\n\n\nb");
        assert_eq!(offsets.line(BytePos(0)), 1); // 'a'
        assert_eq!(offsets.line(BytePos(2)), 2); // empty line
        assert_eq!(offsets.line(BytePos(3)), 3); // empty line
        assert_eq!(offsets.line(BytePos(4)), 4); // 'b'
    }

    #[test]
    fn test_different_line_endings() {
        let offsets = LineOffsets::new("line1\r\nline2\nline3");
        assert_eq!(offsets.line(BytePos(0)), 1); // start of line1
        assert_eq!(offsets.line(BytePos(6)), 1); // end of line1
        assert_eq!(offsets.line(BytePos(7)), 2); // start of line2
        assert_eq!(offsets.line(BytePos(12)), 2); // end of line2
        assert_eq!(offsets.line(BytePos(13)), 3); // start of line3
    }

    #[test]
    #[should_panic]
    fn test_position_beyond_length() {
        let offsets = LineOffsets::new("hello");
        offsets.line(BytePos(10)); // should panic
    }
}
