use anitomy_pure::elements::Category;
use anitomy_pure::{Parser, elements::Element};

use pretty_assertions::assert_eq;
#[test]
fn extract_extension() {
    let result = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").parse().unwrap();
    assert_eq!(
        result.find(Category::FileExtension).unwrap(),
        Element::new(Category::FileExtension, "mkv")
    );
    let result = Parser::new("[DB]_Bleach_225_[C63D149C].avi")
        .parse()
        .unwrap();
    assert_eq!(
        result.find(Category::FileExtension).unwrap(),
        Element::new(Category::FileExtension, "avi")
    );
    let result = Parser::new("[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4")
        .parse()
        .unwrap();
    assert_eq!(
        result.find(Category::FileExtension).unwrap(),
        Element::new(Category::FileExtension, "mp4")
    );
    let result = Parser::new("To_Aru_Kagaku_no_Railgun_13-15_[BD_1080p][AtsA]")
        .parse()
        .unwrap();
    assert!(result.find(Category::FileExtension).is_none());
}

#[test]
fn extract_all() {
    let r1 = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(r1.is_ok());
    let r2 = Parser::new(".mkv").parse();
    assert!(r2.is_ok());
    let r3 = Parser::new("").parse();
    assert!(r3.is_err());
}
