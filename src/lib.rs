#![deny(unsafe_code)]
#![deny(missing_docs)]


//! Fast, zero-dependency library for parsing anime video filenames.
//! 
//! 

use elements::Elements;
use errors::ParsingError;
use utils::remove_ignored_string;

/// Public types returned by the parser: [`Category`](elements::Category), [`Element`](elements::Element), [`Elements`](elements::Elements).
pub mod elements;
/// Error type returned by [`Parser::parse`].
pub mod errors;
pub mod token;
pub mod keyword;
pub mod split;
pub mod utils;

use crate::split::split_raw_data;
