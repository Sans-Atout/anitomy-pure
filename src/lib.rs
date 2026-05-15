#![deny(unsafe_code)]
#![deny(missing_docs)]

//! Fast, zero-dependency library for parsing anime video filenames.
//!
//! See [`Parser`] for the entry point.

use elements::Elements;
use errors::ParsingError;
use parsing::{
    episode::{detect_episode_number_alt, parse_episode_number},
    extensions::{get_extension, remove_extension},
    string::{parse_anime_title, parse_episode_title, parse_release_group},
};
use utils::remove_ignored_string;

use crate::{parsing::parsing_keywords, split::split_raw_data};

/// Public types returned by the parser: [`Category`](elements::Category), [`Element`](elements::Element), [`Elements`](elements::Elements).
pub mod elements;
/// Error type returned by [`Parser::parse`].
pub mod errors;
pub(crate) mod keyword;
pub(crate) mod parsing;
pub(crate) mod split;
pub(crate) mod token;
pub(crate) mod utils;

/// Builder for parsing an anime video filename.
///
/// Construct with [`Parser::new`], configure with the builder methods, then call [`Parser::parse`].
///
/// # Example
///
/// ```rust
/// use anitomy_pure::{Parser, elements::Category};
///
/// let result = Parser::new("[HorribleSubs] Boku no Hero Academia - 73 [1080p].mkv")
///     .parse()
///     .unwrap();
///
/// assert_eq!(result.find(Category::AnimeTitle).unwrap().value, "Boku no Hero Academia");
/// assert_eq!(result.find(Category::EpisodeNumber).unwrap().value, "73");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Parser {
    file_name: String,
    ignored_string: Vec<String>,
    allowed_delimiters: Vec<char>,
    ep_number: bool,
    ep_title: bool,
    file_extension: bool,
    release_group: bool,
}

impl Parser {
    /// Creates a new parser for the given filename.
    pub fn new(f_name: &str) -> Parser {
        Parser {
            file_name: f_name.to_string(),
            ignored_string: Vec::new(),
            allowed_delimiters: vec![' ', '_', '.', '-', '&', '+', ',', '|'],
            ep_number: true,
            ep_title: true,
            file_extension: true,
            release_group: true,
        }
    }

    /// Whether to extract the episode number. Default: `true`.
    pub fn ep_number(mut self, need_to_parse: bool) -> Self {
        self.ep_number = need_to_parse;
        self
    }

    /// Whether to extract the episode title. Default: `true`.
    pub fn ep_title(mut self, need_to_parse: bool) -> Self {
        self.ep_title = need_to_parse;
        self
    }

    /// Whether to extract the file extension. Default: `true`.
    pub fn file_extension(mut self, need_to_parse: bool) -> Self {
        self.file_extension = need_to_parse;
        self
    }

    /// Whether to extract the release group. Default: `true`.
    pub fn release_group(mut self, need_to_parse: bool) -> Self {
        self.release_group = need_to_parse;
        self
    }

    /// Replaces the filename to parse.
    pub fn file_name(mut self, name: &str) -> Self {
        self.file_name = name.to_string();
        self
    }

    /// Substrings to strip from the filename before parsing (e.g. `vec!["[SubGroup]"]`).
    pub fn ignored_string(mut self, i: Vec<&str>) -> Self {
        self.ignored_string = i.into_iter().map(str::to_owned).collect();
        self
    }

    /// Characters treated as token delimiters. Default: `[' ', '_', '.', '-', '&', '+', ',', '|']`.
    pub fn allowed_delimiters(mut self, d: Vec<char>) -> Self {
        self.allowed_delimiters = d;
        self
    }

    /// Parses the filename and returns the extracted elements.
    ///
    /// Returns [`Err(ParsingError::StringIsEmpty)`](errors::ParsingError) if the filename is empty
    /// after stripping the extension.
    pub fn parse(&self) -> Result<Elements, ParsingError> {
        let mut found_elements = Elements::new().add(elements::Category::FileName, &self.file_name);

        // Remove file name extension
        let extension = get_extension(&self.file_name).unwrap_or_default();
        if !extension.is_empty() {
            found_elements.push(elements::Category::FileExtension, &extension);
        }

        let to_parse_str = remove_extension(&self.file_name);
        if to_parse_str.is_empty() {
            return Err(ParsingError::StringIsEmpty);
        }

        // Incomplete/corrupted filename: has an opening bracket with no matching close.
        // Nothing meaningful can be extracted; return only FileName + FileExtension.
        if to_parse_str.contains('[') && !to_parse_str.contains(']') {
            return Ok(found_elements);
        }

        let mut tokens = split_raw_data(
            &remove_ignored_string(&to_parse_str, &self.ignored_string),
            &self.allowed_delimiters,
        );
        parsing_keywords(&mut found_elements, &mut tokens);

        if self.ep_number {
            parse_episode_number(&self.allowed_delimiters, &mut tokens, &mut found_elements);
            detect_episode_number_alt(&mut tokens, &mut found_elements);
        }

        parse_anime_title(&mut tokens, &mut found_elements, &self.allowed_delimiters);

        if self.release_group {
            parse_release_group(&mut tokens, &mut found_elements, &self.allowed_delimiters);
        }

        if self.ep_title {
            parse_episode_title(&mut tokens, &mut found_elements, &self.allowed_delimiters);
        }

        Ok(found_elements)
    }
}
