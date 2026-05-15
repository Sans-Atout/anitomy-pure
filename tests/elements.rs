use anitomy_rust::elements::Category;
use anitomy_rust::elements::Element;
use anitomy_rust::elements::Elements;

use pretty_assertions::assert_eq;
use pretty_assertions::assert_ne;
#[test]
fn elements_eq() {
    let e_1 = Elements::new().add(Category::AnimeSeason, "01");
    let e_2 = Elements::new().add(Category::AnimeSeason, "01");
    let e_3 = Elements::new()
        .add(Category::AnimeSeason, "02")
        .add(Category::AnimeSeason, "01");
    let e_4 = Elements::new().add(Category::EpisodeNumber, "01");
    assert_eq!(e_1, e_2);
    assert_eq!(e_1.size(), 1);
    assert_ne!(e_1, e_3);
    assert_ne!(e_2, e_3);
    assert_ne!(e_2, e_4);
}

#[test]
fn element_eq() {
    let e1 = Element::new(Category::AnimeSeason, "01");
    let e2 = Element::new(Category::AnimeSeason, "01");
    let e3 = Element::new(Category::AnimeSeason, "02");
    let e4 = Element::new(Category::EpisodeNumber, "01");
    assert_eq!(e1, e2);
    assert_ne!(e1, e3);
    assert_ne!(e2, e3);
    assert_ne!(e2, e4);
}

#[test]
fn count() {
    let elements = Elements::new()
        .add(Category::AnimeSeason, "01")
        .add(Category::AnimeSeasonPrefix, "S")
        .add(Category::AnimeSeason, "02");
    assert_eq!(elements.count(Category::AnimeSeason), 2);
}

#[test]
fn find_all() {
    let vec_e = vec![
        Element::new(Category::AnimeSeason, "01"),
        Element::new(Category::AnimeSeason, "02"),
    ];
    let elements = Elements::new()
        .add(Category::AnimeSeason, "01")
        .add(Category::AnimeSeasonPrefix, "S")
        .add(Category::AnimeSeason, "02");
    let all_elements = elements.find_all(Category::AnimeSeason);
    assert!(all_elements.is_some());
    let a_e = all_elements.unwrap();
    assert_eq!(a_e, vec_e);
}

#[test]
fn find_all_not_found() {
    let element = Elements::new();
    assert!(element.find_all(Category::AnimeSeason).is_none());
}

#[test]
fn size() {
    let elements = Elements::new()
        .add(Category::AnimeSeason, "01")
        .add(Category::AnimeSeasonPrefix, "S")
        .add(Category::AnimeSeason, "02");
    assert_eq!(elements.size(), 3);
}

#[test]
fn is_empty() {
    let mut elements = Elements::new();
    assert!(elements.is_empty());
    elements = elements
        .add(Category::AnimeSeason, "01")
        .add(Category::AnimeSeasonPrefix, "S")
        .add(Category::AnimeSeason, "02");
    assert!(!elements.is_empty());
}

#[test]
fn empty_category() {
    let mut e_ = Elements::new().add(Category::AnimeSeason, "01");
    assert!(!e_.is_category_empty(Category::AnimeSeason));
    assert!(e_.is_category_empty(Category::AnimeTitle));
    e_.push(Category::AnimeTitle, "My Super Anime");
    assert!(!e_.is_category_empty(Category::AnimeTitle));
}

#[test]
fn is_equal() {
    let e_1 = Elements::new()
        .add(Category::AnimeSeason, "01")
        .add(Category::EpisodeNumber, "01");
    let e_2 = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::AnimeSeason, "01");
    let e_3 = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::AnimeSeason, "01")
        .add(Category::AnimeYear, "2009");
    assert_eq!(e_1, e_2);
    assert_ne!(e_1, e_3);
}
