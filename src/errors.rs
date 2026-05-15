use std::fmt;

/// Errors that can be returned by [`Parser::parse`](crate::Parser::parse).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingError {
    /// The filename string is empty after stripping the extension.
    StringIsEmpty,
    /// No file extension could be found. Reserved for future use.
    NoExtension,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::StringIsEmpty => fmt.write_str("Parsing Error : nothing to parse"),
            ParsingError::NoExtension => fmt.write_str("Parsing Error : no extension"),
        }
    }
}

impl std::error::Error for ParsingError {}
