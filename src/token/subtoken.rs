//! Subtokens: the atomic units within a [`Token`](super::main_token::Token).

/// An atomic string fragment within a token, tagged with its classification state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubToken {
    value: String,
    category: SubTokenCategory,
}

impl SubToken {
    /// Creates an unclassified subtoken from the given string slice.
    pub fn new(v: &str) -> SubToken {
        SubToken {
            value: v.to_string(),
            category: SubTokenCategory::default(),
        }
    }

    /// Sets the classification category of this subtoken.
    pub fn category(&mut self, c: SubTokenCategory) -> &mut SubToken {
        self.category = c;
        self
    }

    /// Returns `true` if this subtoken's category matches `c`.
    pub fn is_category(&self, c: SubTokenCategory) -> bool {
        self.category == c
    }

    /// Returns the string value of this subtoken.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Classification state of a [`SubToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubTokenCategory {
    /// Not yet classified; visible to title and release-group extraction.
    #[default]
    Unknow,
    /// A delimiter character (space, dot, hyphen, etc.).
    Delimiter,
    /// Structurally invalid; ignored by all passes.
    Invalid,
    /// Consumed by a parsing pass; no longer available to downstream passes.
    Found,
}
