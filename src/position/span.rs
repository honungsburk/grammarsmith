use super::BytePos;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A trait for getting the span of a value.
///
/// This trait is implemented by types that have an associated position range (span)
/// in source text, such as tokens, AST nodes, or other syntactic elements.
pub trait GetSpan {
    fn get_span(&self) -> Span;
}

/// A trait for setting the span of a value.
///
/// This trait allows modifying the position range (span) of an element,
/// typically used during parsing or AST transformations.
pub trait SetSpan {
    fn set_span(&mut self, span: Span);
}

/// A span represents a range of positions in source text, typically used for
/// error reporting, syntax highlighting, and other source-mapping features.
///
/// The range is inclusive of the start position and exclusive of the end position,
/// following the common convention for ranges in Rust. For example, a span of
/// `start: 5, end: 10` covers the bytes/characters at positions 5, 6, 7, 8, and 9.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Span {
    pub start: BytePos,
    pub end: BytePos,
}

impl Span {
    /// Creates a span from start and end positions (exclusive of end).
    ///
    /// Returns `None` if start > end, as this would represent an invalid span.
    ///
    /// # Examples
    /// ```
    /// use grammarsmith::position::*;
    /// let span = Span::new(0, 5); // Valid span covering positions 0-4
    /// let invalid = Span::new(5, 3); // Returns None
    /// ```
    pub fn new(start: usize, end: usize) -> Option<Self> {
        if start > end {
            None
        } else {
            Some(Span {
                start: BytePos(start),
                end: BytePos(end),
            })
        }
    }

    /// Creates a span that covers a single position.
    ///
    /// This is useful for representing zero-width spans like the position of
    /// a delimiter or the insertion point for error recovery.
    ///
    /// # Examples
    /// ```
    /// use grammarsmith::position::*;
    /// let point = Span::point(42); // Span from position 42 to 42
    /// ```
    pub fn point(pos: usize) -> Self {
        Span {
            start: BytePos(pos),
            end: BytePos(pos),
        }
    }

    /// This function does not check if the start is less than the end.
    pub fn new_unchecked(start: usize, end: usize) -> Self {
        Span {
            start: BytePos(start),
            end: BytePos(end),
        }
    }

    /// creates the empty span. 0, 0
    pub const fn empty() -> Self {
        Span {
            start: BytePos(0),
            end: BytePos(0),
        }
    }

    /// Union two spans.
    ///
    /// Example: [0, 10) ∪ [10, 20) = [0, 20)
    pub fn union(&self, other: &Self) -> Self {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Like union, but returns self if other is None.
    pub fn maybe_union(&self, other: &Option<Self>) -> Self {
        match other {
            Some(other) => self.union(other),
            None => self.clone(),
        }
    }

    /// Extend the span to include the given position.
    ///
    /// Example: [15, 10) ∪ 9 = [9, 20)
    pub fn extend(&self, pos: &BytePos) -> Self {
        let mut span = self.clone();
        if span.start.0 > pos.0 {
            span.start = *pos;
        }
        if span.end.0 < pos.0 {
            span.end = *pos;
        }
        span
    }

    /// Get the start position of the span.
    pub fn start(&self) -> usize {
        self.start.0
    }

    /// Get the end position of the span.
    pub fn end(&self) -> usize {
        self.end.0
    }

    /// Get the length of the span.
    pub fn len(&self) -> usize {
        self.end.0 - self.start.0
    }

    /// Check if the span contains a given position.
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start.0 && offset < self.end.0
    }

    /// Check if the span intersects with another span.
    pub fn intersects(&self, other: &Self) -> bool {
        self.start.0 <= other.end.0 && self.end.0 >= other.start.0
    }
}

impl<T> From<WithSpan<T>> for Span {
    fn from(with_span: WithSpan<T>) -> Span {
        with_span.span
    }
}

impl<T> From<&WithSpan<T>> for Span {
    fn from(with_span: &WithSpan<T>) -> Span {
        with_span.span
    }
}

/// Wraps a value with its associated source position information.
///
/// This is commonly used to attach location information to AST nodes,
/// tokens, or other elements that need to be traced back to their
/// original position in the source text.
///
/// # Examples
/// ```
/// enum Token {
///     Identifier(String),
///     While,
///     If,
///     MoreTokens,
/// }
///
/// use grammarsmith::position::*;
/// let token = Token::Identifier("foo".to_string());
/// let span = Span::new(0, 3).unwrap();
/// let spanned_token = WithSpan::new(token, span);
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct WithSpan<T> {
    pub value: T,
    pub span: Span,
}

impl<T> GetSpan for WithSpan<T> {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl<T> SetSpan for WithSpan<T> {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl<T> WithSpan<T> {
    pub const fn new(value: T, span: Span) -> Self {
        WithSpan { value, span }
    }

    pub const fn empty(value: T) -> Self {
        Self {
            value,
            span: Span {
                start: BytePos(0),
                end: BytePos(0),
            },
        }
    }

    pub const fn new_unchecked(value: T, start: usize, end: usize) -> Self {
        Self {
            value,
            span: Span {
                start: BytePos(start),
                end: BytePos(end),
            },
        }
    }

    //TODO Move to AsRef trait impl?
    pub const fn as_ref(&self) -> WithSpan<&T> {
        WithSpan {
            span: self.span,
            value: &self.value,
        }
    }
}
