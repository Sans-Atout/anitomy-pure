use anitomy_pure::{
    Parser,
    elements::{Category, Element},
};

use pretty_assertions::assert_eq;

#[test]
fn testing_anime_season() {
    let elements =
        Parser::new("Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR")
            .parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::AnimeSeason);
    assert!(tested.is_some());
    let wanted = Element::new(Category::AnimeSeason, "2");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_anime_season_prefix() {
    let elements =
        Parser::new("Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR")
            .parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::AnimeSeasonPrefix);
    assert!(tested.is_some());
    let wanted = Element::new(Category::AnimeSeasonPrefix, "saison");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_anime_type() {
    let elements = Parser::new("Ushio to Tora (TV)").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::AnimeType);
    assert!(tested.is_some());
    let wanted = Element::new(Category::AnimeType, "TV");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_anime_year() {
    let elements = Parser::new("[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::AnimeYear);
    assert!(tested.is_some());
    let wanted = Element::new(Category::AnimeYear, "2009");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_audio_term() {
    let elements = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[XBOX][1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::AudioTerm);
    assert!(tested.is_some());
    let wanted = Element::new(Category::AudioTerm, "FLAC");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_device_compatibility() {
    let elements = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[XBOX][1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::DeviceCompatibility);
    assert!(tested.is_some());
    let wanted = Element::new(Category::DeviceCompatibility, "XBOX");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_episode_number() {
    let elements =
        Parser::new("[Kira-Fansub] Uchuu no Stellvia ep 14 (BD 1280x960 24fps AAC) [06EE7355].mkv")
            .parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::EpisodeNumber);
    assert!(tested.is_some());
    let wanted = Element::new(Category::EpisodeNumber, "14");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_episode_number_alt() {
    // TODO
}
#[test]
fn testing_episode_prefix() {
    let elements =
        Parser::new("[Kira-Fansub] Uchuu no Stellvia ep 14 (BD 1280x960 24fps AAC) [06EE7355].mkv")
            .parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::EpisodePrefix);
    assert!(tested.is_some());
    let wanted = Element::new(Category::EpisodePrefix, "ep");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_episode_title() {
    // TODO
}
#[test]
fn testing_file_checksum() {
    let elements = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::FileChecksum);
    assert!(tested.is_some());
    let wanted = Element::new(Category::FileChecksum, "1234ABCD");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_file_extension() {
    let elements = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::FileExtension);
    assert!(tested.is_some());
    let wanted = Element::new(Category::FileExtension, "mkv");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_file_name() {
    let elements = Parser::new("my_test_file.mp4").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::FileName);
    assert!(tested.is_some());
    let wanted = Element::new(Category::FileName, "my_test_file.mp4");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_language() {
    let elements =
        Parser::new("Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR")
            .parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::Language);
    assert!(tested.is_some());
    let wanted = Element::new(Category::Language, "VOSTFR");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_other() {
    let elements = Parser::new("[Mobile Suit Gundam Seed Destiny HD REMASTER][07][Big5][720p][AVC_AAC][encoded by SEED].mp4").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::Other);
    assert!(tested.is_some());
    let wanted = Element::new(Category::Other, "REMASTER");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_release_group() {
    // TODO
}
#[test]
fn testing_release_information() {
    let elements =
        Parser::new("[Zero-Raws] Shingeki no Kyojin - 25 END (MBS 1280x720 x264 AAC).mp4").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::ReleaseInformation);
    assert!(tested.is_some());
    let wanted = Element::new(Category::ReleaseInformation, "END");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_release_version() {
    let elements = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01_v2_-_Tiger_and_Dragon_[XBOX][1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::ReleaseVersion);
    assert!(tested.is_some());
    let wanted = Element::new(Category::ReleaseVersion, "2");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_source() {
    let elements =
        Parser::new("[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::Source);
    assert!(tested.is_some());
    let wanted = Element::new(Category::Source, "DVD");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_subtitles() {
    let elements = Parser::new(
        "[ValdikSS]_First_Squad_The_Morment_Of_Truth_[720x576_h264_dvdscr_eng_hardsub].mkv",
    )
    .parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::Subtitles);
    assert!(tested.is_some());
    let wanted = Element::new(Category::Subtitles, "hardsub");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_video_resolution() {
    let elements = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::VideoResolution);
    assert!(tested.is_some());
    let wanted = Element::new(Category::VideoResolution, "1280x720");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_video_term() {
    let elements =
        Parser::new("[Kira-Fansub] Uchuu no Stellvia ep 14 (BD 1280x960 24fps AAC) [06EE7355].mkv")
            .parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::VideoTerm);
    assert!(tested.is_some());
    let wanted = Element::new(Category::VideoTerm, "24fps");
    assert_eq!(tested.unwrap(), wanted)
}
#[test]
fn testing_volume_number() {
    let elements = Parser::new("[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::VolumeNumber);
    assert!(tested.is_some());
    let wanted = Element::new(Category::VolumeNumber, "03");
    assert_eq!(tested.unwrap(), wanted)
}
