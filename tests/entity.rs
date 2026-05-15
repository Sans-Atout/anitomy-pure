use std::fmt::{Debug, Display};

use anitomy_rust::{
    elements::{Category, Element, Elements},
    errors::ParsingError,
};

fn has_send_sync<T: Sized + Send + Sync + Unpin>() {}
fn has_needed<T: Clone + Debug>() {}
fn has_copy<T: Copy>() {}
fn test_error<T: Sized + Send + Sync + Unpin + Debug + Clone + Copy + PartialEq + Eq + Display>() {}

/// Test if all Entity int this library has Send and Sync trait implemented
///
/// This test is successful if all entity have the Sync and Send trait.
/// If the library compile, then this test must pass (has Send and Sync
///  traits is checked at compile time).
#[test]
fn has_entity_sync_send() {
    has_send_sync::<Element>();
    has_send_sync::<Category>();
    has_send_sync::<Elements>();
}

/// Test if all Entity int this library has Send and Sync trait implemented
///
/// This test is successful if all entity have Copy, Clone and Debug trait
/// If the library compile, then this test must pass
#[test]
fn has_entity_needed() {
    has_needed::<Category>();
    has_needed::<Element>();
    has_needed::<Elements>();
    has_copy::<Category>();
}

#[test]
fn category_singular() {
    assert!(!Category::AnimeSeason.is_singular());
    assert!(Category::AnimeYear.is_singular());
}

#[test]
fn category_search() {
    assert!(Category::AnimeType.is_searchable());
    assert!(!Category::AnimeSeason.is_searchable())
}

#[test]
fn has_error_needed() {
    test_error::<ParsingError>();
}

#[test]
fn error_display() {
    println!("{}", ParsingError::NoExtension);
    println!("{}", ParsingError::StringIsEmpty);
}
