//! Top-level token produced by the filename splitter.
use crate::{parsing::number::is_digit, split::split_token};

use super::subtoken::{SubToken, SubTokenCategory};

/// A segment of the filename, optionally enclosed in brackets, holding a list of [`SubToken`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    tokens: Vec<SubToken>,
    raw_token: String,
    inside_delimiter: bool,
    weak_delimiter: bool,
    paren_delimiter: bool,
    japanese_corner: bool,
}

impl Token {
    /// Creates a token by splitting `raw` into subtokens on the given delimiters.
    pub fn new(
        raw: &str,
        delimiter: &[char],
        in_delimiter: bool,
        is_weak: bool,
        is_corner: bool,
    ) -> Token {
        let splited_token = split_token(raw, delimiter);
        let all_tokens: Vec<SubToken> = splited_token.iter().map(|t| SubToken::new(t)).collect();
        Token {
            weak_delimiter: is_weak && all_tokens.len() == 1,
            paren_delimiter: is_weak,
            japanese_corner: is_corner,
            tokens: all_tokens,
            raw_token: raw.to_string(),
            inside_delimiter: in_delimiter,
        }
    }

    /// Returns `true` if at least one subtoken is still unclassified.
    pub fn contains_unknow(&self) -> bool {
        self.tokens.iter().any(|t| t.is_category(SubTokenCategory::Unknow))
    }

    /// Returns `true` if every subtoken is still unclassified.
    pub fn is_full_unknow(&self) -> bool {
        self.tokens.iter().all(|t| t.is_category(SubTokenCategory::Unknow))
    }

    /// Returns `true` if this token is a single unclassified digit string.
    pub fn is_isolated_number(&self) -> bool {
        self.tokens.len() == 1
            && is_digit(self.tokens[0].value())
            && !self.tokens[0].is_category(SubTokenCategory::Found)
    }

    /// Returns a mutable reference to the list of subtokens.
    pub fn sub_tokens(&mut self) -> &mut Vec<SubToken> {
        &mut self.tokens
    }

    /// Returns a reference to the raw token string — zero allocation.
    pub fn raw_token(&self) -> &str {
        &self.raw_token
    }

    /// Returns both the raw string slice and the mutable subtoken vec in one call,
    /// so the borrow checker can see them as disjoint field borrows.
    pub fn raw_and_subtokens(&mut self) -> (&str, &mut Vec<SubToken>) {
        (&self.raw_token, &mut self.tokens)
    }

    /// Returns `true` if this token was enclosed in square or curly brackets.
    pub fn is_inside_delimiter(&self) -> bool {
        self.inside_delimiter
    }

    /// Returns `true` if this token contains a single subtoken with a weak (dot/underscore) delimiter.
    pub fn is_weak(&self) -> bool {
        self.weak_delimiter
    }

    /// Returns `true` if this token was enclosed in parentheses.
    pub fn is_paren(&self) -> bool {
        self.paren_delimiter
    }

    /// Returns `true` if this token was enclosed in Japanese corner brackets (`「」`).
    pub fn is_japanese_corner(&self) -> bool {
        self.japanese_corner
    }
}
