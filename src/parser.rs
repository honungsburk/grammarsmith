use crate::position::*;

/// A trait for tokens that can be parsed.
///
/// This trait defines the basic requirements for a token type that can be used
/// with the Parser. Tokens must have a kind that can be compared for equality.
pub trait Token {
    /// The kind/type of the token.
    ///
    /// This associated type represents the enum or type used to distinguish
    /// different kinds of tokens (e.g., Identifier, Number, Operator, etc.).
    type Kind: PartialEq;

    /// Convert the token to its kind.
    ///
    /// Returns the Kind value that represents this token's type.
    fn to_kind(&self) -> Self::Kind;
}

/// A trait for tokens that represent the end of the file.
///
/// This trait extends the Token trait to provide functionality specific to
/// end-of-file (EOF) handling in the parser.
pub trait EndOfFile: Token {
    /// Creates a new token that represents the end of the file.
    fn eof() -> Self;

    /// Returns the Kind value that represents an end-of-file token.
    fn eof_kind() -> Self::Kind;
}

/// A parser for a token stream.
///
/// The Parser provides methods for traversing and analyzing a sequence of tokens.
/// It maintains a current position in the token stream and provides various
/// methods for checking and consuming tokens.
///
/// # Type Parameters
/// * `'a` - The lifetime of the token references
/// * `T` - The token type that implements both Token and EndOfFile traits
pub struct Parser<'a, T>
where
    T: Token + EndOfFile,
{
    current: usize,
    tokens: &'a [WithSpan<T>],
    eof_token: &'a WithSpan<T>,
}

impl<'a, T> Parser<'a, T>
where
    T: Token + EndOfFile,
{
    /// Creates a new Parser instance.
    ///
    /// # Arguments
    /// * `tokens` - A vector of tokens with their associated spans
    /// * `eof_token` - A reference to the EOF token that will be returned when reaching the end
    pub fn new(tokens: &'a [WithSpan<T>], eof_token: &'a WithSpan<T>) -> Self {
        Parser {
            current: 0,
            tokens: tokens,
            eof_token: eof_token,
        }
    }

    /// Returns the kind of the current token without advancing the parser.
    pub fn peek(&self) -> T::Kind {
        return self.peek_token().value.to_kind();
    }

    /// Returns a reference to the current token with its span information.
    pub fn peek_token(&self) -> &'a WithSpan<T> {
        self.tokens.get(self.current).unwrap_or(&self.eof_token)
    }

    /// Returns a reference to the previously consumed token.
    ///
    /// If no tokens have been consumed yet, returns the EOF token.
    pub fn previous(&self) -> &'a WithSpan<T> {
        return self.tokens.get(self.current - 1).unwrap_or(&self.eof_token);
    }

    /// Returns true if the parser has reached the end of the token stream.
    pub fn is_at_end(&self) -> bool {
        return self.peek() == T::eof_kind();
    }

    /// Checks if the current token matches the specified kind without advancing.
    ///
    /// # Arguments
    /// * `token` - The token kind to check against
    ///
    /// # Returns
    /// `true` if the current token matches the specified kind, `false` otherwise
    pub fn check(&self, token: T::Kind) -> bool {
        if self.is_at_end() {
            false
        } else {
            token == self.peek()
        }
    }

    /// Checks if the current token matches any of the specified kinds.
    ///
    /// # Arguments
    /// * `tokens` - A slice of token kinds to check against
    pub fn check_one_of(&mut self, tokens: &[T::Kind]) -> bool {
        tokens.contains(&self.peek())
    }

    /// Advances the parser to the next token and returns the previous token.
    ///
    /// If the parser is at the end of the token stream, it will not advance
    /// but still return the previous token.
    pub fn advance(&mut self) -> &'a WithSpan<T> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    /// Checks if the current token matches any of the specified kinds and advances if true.
    ///
    /// # Arguments
    /// * `tokens` - An iterator of token kinds to check against
    ///
    /// # Returns
    /// `true` if a match was found and consumed, `false` otherwise
    pub fn is_one_of<I: IntoIterator<Item = T::Kind>>(&mut self, tokens: I) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    /// Checks if the current token matches the specified kind and advances if true.
    ///
    /// This is a convenience method that combines `check()` and `advance()`.
    pub fn is(&mut self, token: T::Kind) -> bool {
        if self.check(token) {
            self.advance();
            return true;
        }
        return false;
    }

    /// Similar to `is()` but with a more semantic name for optional tokens.
    ///
    /// This method is particularly useful when parsing optional syntax elements.
    pub fn optional(&mut self, token: T::Kind) -> bool {
        if self.check(token) {
            self.advance();
            return true;
        }
        return false;
    }

    /// Discards tokens until one matching the specified kinds is found.
    ///
    /// This method is useful for error recovery in parsing, allowing the parser
    /// to skip invalid tokens until it finds a synchronization point.
    ///
    /// # Arguments
    /// * `tokens` - A slice of token kinds to look for
    ///
    /// # Returns
    /// The span covering all skipped tokens, or None if no tokens were skipped
    pub fn drop_until(&mut self, tokens: &[T::Kind]) -> Option<Span> {
        let mut dropped_span: Option<Span> = None;
        while !self.is_at_end() && !tokens.contains(&self.peek()) {
            let token = self.advance();
            dropped_span = dropped_span
                .map(|s| s.union(&token.span))
                .or(Some(token.span));
        }
        dropped_span
    }
}
