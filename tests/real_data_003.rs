use anitomy_rust::elements::{Category, Elements};
use anitomy_rust::Parser;
use pretty_assertions::assert_eq;
#[test]
fn test_1c2d69ad8440899540711617251eab598fcb7c74() {
    let wanted = Elements::new()
        .add(
            Category::AnimeTitle,
            "\u{6f06}\u{9ed1}\u{7684}\u{5b50}\u{5f48}",
        )
        .add(Category::EpisodeNumber, "11")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[異域字幕組][漆黑的子彈][Black Bullet][11][1280x720][繁体].mp4",
        )
        .add(Category::ReleaseGroup, "異域字幕組")
        .add(Category::VideoResolution, "1280x720");
    let parser_result =
        Parser::new("[異域字幕組][漆黑的子彈][Black Bullet][11][1280x720][繁体].mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_dbd1a64e01bf0d25fa7ac9c7c93eb6e6af391ee4() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Mangaka san to Assistant san to the Animation")
        .add(Category::AudioTerm,"AAC")
        .add(Category::EpisodeNumber,"02")
        .add(Category::FileExtension,"mp4")
        .add(Category::FileName,"[AoJiaoZero][Mangaka-san to Assistant-san to the Animation] 02 [BIG][X264_AAC][720P].mp4")
        .add(Category::ReleaseGroup,"AoJiaoZero")
        .add(Category::VideoResolution,"720P")
        .add(Category::VideoTerm,"X264")
    ;
    let parser_result = Parser::new(
        "[AoJiaoZero][Mangaka-san to Assistant-san to the Animation] 02 [BIG][X264_AAC][720P].mp4",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_bcbe37f0c9e13c7ba2dc43b4b6123e47931bb697() {
    let wanted = Elements::new()
        .add(Category::FileName, "Vol.01")
        .add(Category::VolumeNumber, "01");
    let parser_result = Parser::new("Vol.01").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b59cd09da11e1614cf1bd4ea1b738c96da6e866c() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Rozen Maiden 3")
        .add(Category::AnimeType, "PV")
        .add(Category::FileChecksum, "CA57F300")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Asenshi] Rozen Maiden 3 - PV [CA57F300].mkv",
        )
        .add(Category::ReleaseGroup, "Asenshi");
    let parser_result = Parser::new("[Asenshi] Rozen Maiden 3 - PV [CA57F300].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_41bd35e56b82b1b8a02b89e3382c01c990fbbb5b() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Mary Bell")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mkv")
        .add(Category::FileName, "Mary Bell (DVD) - 01v2 [h-b].mkv")
        .add(Category::ReleaseGroup, "h-b")
        .add(Category::ReleaseVersion, "2")
        .add(Category::Source, "DVD");
    let parser_result = Parser::new("Mary Bell (DVD) - 01v2 [h-b].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_1758eba9ed69770eb69f6d4d2c1a432ebaa50adb() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Mary Bell")
        .add(Category::EpisodeNumber, "02")
        .add(Category::FileExtension, "mkv")
        .add(Category::FileName, "Mary Bell (DVD) - 02 [h-b].mkv")
        .add(Category::ReleaseGroup, "h-b")
        .add(Category::Source, "DVD");
    let parser_result = Parser::new("Mary Bell (DVD) - 02 [h-b].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_8ea5cd0983fdc6fa7a686892d47cfcd2eb19c7d6() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Attack on Titan")
        .add(Category::EpisodeNumber, "3")
        .add(Category::EpisodePrefix, "Episode")
        .add(
            Category::EpisodeTitle,
            "A Dim Light Amid Despair / Humanity's Comeback Part 1",
        )
        .add(
            Category::FileName,
            "Attack on Titan - Episode 3 - A Dim Light Amid Despair / Humanity's Comeback, Part 1",
        );
    let parser_result = Parser::new(
        "Attack on Titan - Episode 3 - A Dim Light Amid Despair / Humanity's Comeback, Part 1",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_1e66287abd3757f53f05f99382d85d9de6c00660() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "01")
        .add(Category::AnimeTitle, "The Irregular at Magic High School")
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeTitle, "Enrollment Part I")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "The Irregular at Magic High School - S01E01- Enrollment Part I.mkv",
        );
    let parser_result =
        Parser::new("The Irregular at Magic High School - S01E01- Enrollment Part I.mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_4fccebbb474efc742e9f5820520b58b174afb680() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Aikatsu!")
        .add(Category::EpisodeNumber, "100")
        .add(Category::FileChecksum, "D035A39F")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Mezashite] Aikatsu! ‒ 100 [D035A39F].mkv",
        )
        .add(Category::ReleaseGroup, "Mezashite");
    let parser_result = Parser::new("[Mezashite] Aikatsu! ‒ 100 [D035A39F].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_9615aae5db2459075868837842a9fce4c5f6c960() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "DRAMAtical Murder")
        .add(Category::EpisodeNumber, "1")
        .add(Category::EpisodePrefix, "Episode")
        .add(Category::EpisodeTitle, "Data_01_Login")
        .add(
            Category::FileName,
            "DRAMAtical Murder Episode 1 - Data_01_Login",
        );
    let parser_result = Parser::new("DRAMAtical Murder Episode 1 - Data_01_Login")
        .allowed_delimiters(vec![' '])
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_c61a3ca5107b6dc66f6a31fa804ad69145488aac() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Today in Class 5-2")
        .add(Category::EpisodeNumber, "04")
        .add(Category::FileExtension, "avi")
        .add(Category::FileName, "[Triad]_Today_in_Class_5-2_-_04.avi")
        .add(Category::ReleaseGroup, "Triad");
    let parser_result = Parser::new("[Triad]_Today_in_Class_5-2_-_04.avi")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b539723e80f010856ace07c168c08d36bee5566b() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "BLUE DROP")
        .add(Category::EpisodeNumber, "10")
        .add(Category::FileExtension, "avi")
        .add(Category::FileName, "__BLUE DROP 10 (1).avi");
    let parser_result = Parser::new("__BLUE DROP 10 (1).avi").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_080ec26742dd4a6dc2db6d7bbeda583517afd6cd() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Death Note")
        .add(Category::EpisodeNumber, "37")
        .add(Category::FileChecksum, "6FA7D273")
        .add(Category::FileExtension, "avi")
        .add(
            Category::FileName,
            "37 [Ruberia]_Death_Note_-_37v2_[FINAL]_[XviD][6FA7D273].avi",
        )
        .add(Category::ReleaseGroup, "Ruberia")
        .add(Category::ReleaseInformation, "FINAL")
        .add(Category::ReleaseVersion, "2")
        .add(Category::VideoTerm, "XviD");
    let parser_result = Parser::new("37 [Ruberia]_Death_Note_-_37v2_[FINAL]_[XviD][6FA7D273].avi")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_72345aaa3bba6ed497c93d58d3b51fc3dcfbc110() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Accel World - EX")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "3E56EE18")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[UTW]_Accel_World_-_EX01_[BD][h264-720p_AAC][3E56EE18].mkv",
        )
        .add(Category::ReleaseGroup, "UTW")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "h264");
    let parser_result = Parser::new("[UTW]_Accel_World_-_EX01_[BD][h264-720p_AAC][3E56EE18].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_f5b45292dcabd0f55da5cc563c965c2ccf88c948() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Tsukimonogatari")
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumber, "04")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[HorribleSubs] Tsukimonogatari - (01-04) [1080p].mkv",
        )
        .add(Category::ReleaseGroup, "HorribleSubs")
        .add(Category::VideoResolution, "1080p");
    let parser_result = Parser::new("[HorribleSubs] Tsukimonogatari - (01-04) [1080p].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_aa55f1e24d075068347866a32ad23c006b622abf() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Bokura Ga Ita")
        .add(Category::AudioTerm, "AC3")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "BFCE1627")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Urusai]_Bokura_Ga_Ita_01_[DVD_h264_AC3]_[BFCE1627][Fixed].mkv",
        )
        .add(Category::ReleaseGroup, "Urusai")
        .add(Category::Source, "DVD")
        .add(Category::VideoTerm, "h264");
    let parser_result =
        Parser::new("[Urusai]_Bokura_Ga_Ita_01_[DVD_h264_AC3]_[BFCE1627][Fixed].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_46bb883936b8aa6c31cb1819a7b55615d40df9ec() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "White Album")
        .add(Category::AudioTerm, "FLAC")
        .add(Category::EpisodeNumber, "1")
        .add(Category::EpisodeNumber, "13")
        .add(
            Category::FileName,
            "[Coalgirls]_White_Album_1-13_(1280×720_Blu-Ray_FLAC)",
        )
        .add(Category::ReleaseGroup, "Coalgirls")
        .add(Category::Source, "Blu-Ray")
        .add(Category::VideoResolution, "1280×720");
    let parser_result = Parser::new("[Coalgirls]_White_Album_1-13_(1280×720_Blu-Ray_FLAC)")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_481d789af2995b078496bad168d54aca5f2642f2() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Bakemonogatari")
        .add(Category::AnimeType, "OP")
        .add(Category::AudioTerm, "FLAC")
        .add(Category::EpisodeNumber, "4a")
        .add(Category::FileChecksum, "327A2375")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Coalgirls]_Bakemonogatari_OP4a_(1280x720_Blu-Ray_FLAC)_[327A2375].mkv",
        )
        .add(Category::ReleaseGroup, "Coalgirls")
        .add(Category::Source, "Blu-Ray")
        .add(Category::VideoResolution, "1280x720");
    let parser_result =
        Parser::new("[Coalgirls]_Bakemonogatari_OP4a_(1280x720_Blu-Ray_FLAC)_[327A2375].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_933d2317dc58dfe798a882a7b5dca5e2cba313a5() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "No.6")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "A956075C")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Ruri]No.6 01 [720p][H264][A956075C].mkv",
        )
        .add(Category::ReleaseGroup, "Ruri")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "H264");
    let parser_result = Parser::new("[Ruri]No.6 01 [720p][H264][A956075C].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_55c7d19d958f5cad821211544892b8c46b22d2a1() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Sword Art Online Extra Edition")
        .add(Category::AudioTerm, "Dual Audio")
        .add(Category::AudioTerm, "Vorbis")
        .add(
            Category::FileName,
            "[CH] Sword Art Online Extra Edition Dual Audio [BD 480p][10bitH.264+Vorbis]",
        )
        .add(Category::ReleaseGroup, "CH")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "480p")
        .add(Category::VideoTerm, "H.264")
        .add(Category::VideoTerm, "10bits");
    let parser_result =
        Parser::new("[CH] Sword Art Online Extra Edition Dual Audio [BD 480p][10bitH.264+Vorbis]")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_0b0663990f07915ab586b8978bba8b8c8ea9db76() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Akuma no Riddle")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "69A307A2")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv",
        )
        .add(Category::ReleaseGroup, "Watakushi")
        .add(Category::ReleaseVersion, "2")
        .add(Category::VideoResolution, "720p");
    let parser_result =
        Parser::new("EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv")
            .ignored_string(vec!["['EvoBot.']"])
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
/// ! False positive
fn test_fefdb51f699cb95f7862cc3ea0915725c92dc17a() {
    let wanted = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeTitle, "Land of Visible Pain")
        .add(Category::FileExtension, "mkv")
        .add(Category::FileName, "01 - Land of Visible Pain.mkv");
    let parser_result = Parser::new("01 - Land of Visible Pain.mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_657e0f71f42e3a9253d72474aadc28792721df99() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "2")
        .add(Category::AnimeSeasonPrefix, "saison")
        .add(Category::EpisodePrefix, "Episode")
        .add(Category::EpisodeNumber, "14")
        .add(
            Category::EpisodeTitle,
            "Ore no Imouto ga Konnani Kawaii Wake ga Nai",
        )
        .add(
            Category::FileName,
            "Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR",
        )
        .add(Category::Language, "VOSTFR");
    let parser_result =
        Parser::new("Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_cd1315f3d554f90f657686190d76465599677d8f() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Machine Doll wa Kizutsukanai")
        .add(Category::EpisodeNumber,"01")
        .add(Category::EpisodeTitle,"Facing ''Cannibal Candy'' I")
        .add(Category::FileChecksum,"B99C8DED")
        .add(Category::FileExtension,"mkv")
        .add(Category::FileName,"[Zom-B] Machine-Doll wa Kizutsukanai - 01 - Facing ''Cannibal Candy'' I (kuroi, FFF remux) [B99C8DED].mkv")
        .add(Category::ReleaseInformation,"remux")
        .add(Category::ReleaseGroup,"Zom-B")
    ;
    let parser_result = Parser::new("[Zom-B] Machine-Doll wa Kizutsukanai - 01 - Facing ''Cannibal Candy'' I (kuroi, FFF remux) [B99C8DED].mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b8184bb937a5a533f12d140435a206b8f41e0325() {
    let wanted = Elements::new()
        .add(
            Category::AnimeTitle,
            "The iDOLM@STER 765 Pro to Iu Monogatari",
        )
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "The iDOLM@STER 765 Pro to Iu Monogatari.mkv",
        );
    let parser_result = Parser::new("The iDOLM@STER 765 Pro to Iu Monogatari.mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_8b58a2b068063ed4e2dc03741ccd44de82020236() {
    let wanted = Elements::new()
        .add(
            Category::AnimeTitle,
            "Darker than Black - Gemini of the Meteor",
        )
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "65274FDE")
        .add(Category::FileExtension, "7z")
        .add(
            Category::FileName,
            "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].patch.7z",
        )
        .add(Category::ReleaseInformation, "patch")
        .add(Category::ReleaseGroup, "Yuurisan-Subs")
        .add(Category::ReleaseVersion, "2");
    let parser_result = Parser::new(
        "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].patch.7z",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b908c1ca507d6e053ecd67fa19929eb157035084() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Fate Zero OVA")
        .add(Category::AnimeType, "OVA")
        .add(Category::AudioTerm, "FLAC")
        .add(Category::EpisodeNumber, "3.5")
        .add(Category::FileChecksum, "5F5AD026")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Coalgirls]_Fate_Zero_OVA3.5_(1280x720_Blu-ray_FLAC)_[5F5AD026].mkv",
        )
        .add(Category::ReleaseGroup, "Coalgirls")
        .add(Category::Source, "Blu-ray")
        .add(Category::VideoResolution, "1280x720");
    let parser_result =
        Parser::new("[Coalgirls]_Fate_Zero_OVA3.5_(1280x720_Blu-ray_FLAC)_[5F5AD026].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_75f7dea0b16f866a769d5e3bfd7266215a9a4340() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Koi Kaze")
        .add(Category::AudioTerm, "Dual Audio")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "c13cefe0")
        .add(Category::EpisodePrefix, "Ep")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[GrimRipper] Koi Kaze [Dual Audio] Ep01 (c13cefe0).mkv",
        )
        .add(Category::ReleaseGroup, "GrimRipper");
    let parser_result = Parser::new("[GrimRipper] Koi Kaze [Dual Audio] Ep01 (c13cefe0).mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_8934652d8642b1dc86d5b0a539714d83f92f46db() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Gintama")
        .add(Category::EpisodeNumber, "111C")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[HorribleSubs] Gintama - 111C [1080p].mkv",
        )
        .add(Category::ReleaseGroup, "HorribleSubs")
        .add(Category::VideoResolution, "1080p");
    let parser_result = Parser::new("[HorribleSubs] Gintama - 111C [1080p].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_f2a8540b58fd335f5178bc387c42beaf0f49e6b6() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Hidamari Sketch x365")
        .add(Category::EpisodeNumber, "04.1")
        .add(Category::FileChecksum, "B6CE8458")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[SpoonSubs]_Hidamari_Sketch_x365_-_04.1_(DVD)[B6CE8458].mkv",
        )
        .add(Category::ReleaseGroup, "SpoonSubs")
        .add(Category::Source, "DVD");
    let parser_result = Parser::new("[SpoonSubs]_Hidamari_Sketch_x365_-_04.1_(DVD)[B6CE8458].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b17f5b9e8b46b7a6ba43135de21ae868701f5a4e() {
    let wanted = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodePrefix, "Ep")
        .add(Category::EpisodeTitle, "The Boy in the Iceberg")
        .add(Category::FileName, "Ep. 01 - The Boy in the Iceberg");
    let parser_result = Parser::new("Ep. 01 - The Boy in the Iceberg")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e43b8bb57b59841f1607b68cf610a1990379a980() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Kuroko no Basuke")
        .add(Category::AnimeSeasonPrefix, "S")
        .add(Category::AnimeSeason, "3")
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumberAlt, "51")
        .add(Category::FileChecksum, "619C57A0")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Hatsuyuki]_Kuroko_no_Basuke_S3_-_01_(51)_[720p][10bit][619C57A0].mkv",
        )
        .add(Category::ReleaseGroup, "Hatsuyuki")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "10bit");
    let parser_result =
        Parser::new("[Hatsuyuki]_Kuroko_no_Basuke_S3_-_01_(51)_[720p][10bit][619C57A0].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_23032f3b6ea9ea827e1b62eb2816dc424892ba85() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Sora no Woto")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "07.5")
        .add(Category::FileChecksum, "C37580F8")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Elysium]Sora.no.Woto.EP07.5(BD.720p.AAC)[C37580F8].mkv",
        )
        .add(Category::ReleaseGroup, "Elysium")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "720p");
    let parser_result = Parser::new("[Elysium]Sora.no.Woto.EP07.5(BD.720p.AAC)[C37580F8].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_5eced9ee3b11134397cae55cb35b525281f9dc6a() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Sora no Woto")
        .add(Category::AudioTerm,"AAC")
        .add(Category::EpisodeNumber,"07.5")
        .add(Category::EpisodeTitle,"Drinking Party - Fortress Battle")
        .add(Category::FileChecksum,"F7DF16F7")
        .add(Category::FileExtension,"mkv")
        .add(Category::FileName,"[Zurako] Sora no Woto - 07.5 - Drinking Party - Fortress Battle (BD 1080p AAC) [F7DF16F7].mkv")
        .add(Category::ReleaseGroup,"Zurako")
        .add(Category::Source,"BD")
        .add(Category::VideoResolution,"1080p")
    ;
    let parser_result = Parser::new("[Zurako] Sora no Woto - 07.5 - Drinking Party - Fortress Battle (BD 1080p AAC) [F7DF16F7].mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_cc41c6072101ae24a8c6d864610346a8f42c10aa() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Maji de Watashi ni Koi Shinasai!!")
        .add(Category::EpisodeNumber, "02")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Hiryuu] Maji de Watashi ni Koi Shinasai!! - 02 [720].mkv",
        )
        .add(Category::ReleaseGroup, "Hiryuu")
        .add(Category::VideoResolution, "720");
    let parser_result = Parser::new("[Hiryuu] Maji de Watashi ni Koi Shinasai!! - 02 [720].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_238a3c09689868da2099674fed0f2ec911760a02() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Uchuu no Stellvia")
        .add(Category::EpisodePrefix, "ep")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "14")
        .add(Category::FileChecksum, "06EE7355")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Kira-Fansub] Uchuu no Stellvia ep 14 (BD H264 1280x960 24fps AAC) [06EE7355].mkv",
        )
        .add(Category::ReleaseGroup, "Kira-Fansub")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1280x960")
        .add(Category::VideoTerm, "H264")
        .add(Category::VideoTerm, "24fps");
    let parser_result = Parser::new(
        "[Kira-Fansub] Uchuu no Stellvia ep 14 (BD H264 1280x960 24fps AAC) [06EE7355].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_cb06b789a4d1e156cdbf4c0d90d61dda74741d02() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Yosuga no Sora")
        .add(Category::EpisodePrefix, "Ep")
        .add(Category::AnimeType, "Preview")
        .add(Category::AudioTerm, "FLAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[ANE] Yosuga no Sora - Ep01 Preview (Yorihime ver) [BDRip 1080p x264 FLAC].mkv",
        )
        .add(Category::ReleaseGroup, "ANE")
        .add(Category::Source, "BDRip")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "x264");
    let parser_result = Parser::new(
        "[ANE] Yosuga no Sora - Ep01 Preview (Yorihime ver) [BDRip 1080p x264 FLAC].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_3bbe6167e0edcf33bf4d42dc9735cf7fc8c670b2() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Myself Yourself")
        .add(Category::AudioTerm, "DD2.0")
        .add(Category::EpisodeNumber, "12")
        .add(Category::FileChecksum, "CB2B37F1")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[B-G_&_m.3.3.w]_Myself_Yourself_12.DVD(H.264_DD2.0)_[CB2B37F1].mkv",
        )
        .add(Category::ReleaseGroup, "B-G_&_m.3.3.w")
        .add(Category::Source, "DVD")
        .add(Category::VideoTerm, "H.264");
    let parser_result =
        Parser::new("[B-G_&_m.3.3.w]_Myself_Yourself_12.DVD(H.264_DD2.0)_[CB2B37F1].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b5154c81317da4e81999799325c0346ed754975f() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "The Animatrix")
        .add(Category::AudioTerm, "DTS")
        .add(Category::EpisodeNumber, "08")
        .add(Category::EpisodeTitle, "A Detective Story")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR.mkv",
        )
        .add(Category::ReleaseGroup, "ESiR")
        .add(Category::Source, "BluRay")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR.mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_fde577beca5dea3a1bc9677fe032baa93c4d8ceb() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Oreshura")
        .add(Category::EpisodeNumber,"01")
        .add(Category::EpisodeTitle,"The Start Of High School Life Is A War Zone")
        .add(Category::FileExtension,"mkv")
        .add(Category::FileChecksum,"211375E6")
        .add(Category::FileName,"[DmonHiro] Oreshura #01v2 - The Start Of High School Life Is A War Zone [BD, 720p] [211375E6].mkv")
        .add(Category::ReleaseGroup,"DmonHiro")
        .add(Category::ReleaseVersion,"2")
        .add(Category::Source,"BD")
        .add(Category::VideoResolution,"720p")
    ;
    let parser_result = Parser::new("[DmonHiro] Oreshura #01v2 - The Start Of High School Life Is A War Zone [BD, 720p] [211375E6].mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_95a640a4c04e7318eb50814536d6e6913124ad0a() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Abarenbou Rikishi!! Matsutarou")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "04")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[모에-Raws] Abarenbou Rikishi!! Matsutarou #04 (ABC 1280x720 x264 AAC).mp4",
        )
        .add(Category::ReleaseGroup, "모에-Raws")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("[모에-Raws] Abarenbou Rikishi!! Matsutarou #04 (ABC 1280x720 x264 AAC).mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_2c12ba4895acc45e1ff89a8d4817bd5e1b0b58d0() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Nekomonogatari (Black)")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "1")
        .add(Category::EpisodeNumber, "4")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[바카-Raws] Nekomonogatari (Black) #1-4 (BS11 1280x720 x264 AAC).mp4",
        )
        .add(Category::ReleaseGroup, "바카-Raws")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("[바카-Raws] Nekomonogatari (Black) #1-4 (BS11 1280x720 x264 AAC).mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_0c921bfd33358072d4bda0ea9f6005b60b9b31eb() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Tiger & Bunny")
        .add(Category::AudioTerm,"AAC")
        .add(Category::EpisodeNumber,"01")
        .add(Category::EpisodeTitle,"All's well that ends well")
        .add(Category::FileChecksum,"4A9AB85F")
        .add(Category::FileExtension,"mkv")
        .add(Category::FileName,"[NinjaPanda] Tiger & Bunny #01 All's well that ends well. (v3, 1080p Hi10P, DA AAC) [4A9AB85F].mkv")
        .add(Category::ReleaseGroup,"NinjaPanda")
        .add(Category::ReleaseVersion,"3")
        .add(Category::VideoResolution,"1080p")
        .add(Category::VideoTerm,"Hi10P")
    ;
    let parser_result = Parser::new("[NinjaPanda] Tiger & Bunny #01 All's well that ends well. (v3, 1080p Hi10P, DA AAC) [4A9AB85F].mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_9167d505c372aec51dfbdb563a42dd3182bedab1() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Neko no Ongaeshi")
        .add(Category::AudioTerm, "DualAudio")
        .add(Category::FileChecksum, "0CDC2145")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Neko no Ongaeshi - [HQR.remux-DualAudio][NTV.1280x692.h264](0CDC2145).mkv",
        )
        .add(Category::VideoResolution, "1280x692")
        .add(Category::VideoTerm, "h264");
    let parser_result =
        Parser::new("Neko no Ongaeshi - [HQR.remux-DualAudio][NTV.1280x692.h264](0CDC2145).mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_8f28318fffa8372035c3d6cd204eb45beb91d6b1() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Memories Off 3.5")
        .add(Category::EpisodeNumber, "04")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[ReDone] Memories Off 3.5 - 04 (DVD 10-bit).mkv",
        )
        .add(Category::ReleaseGroup, "ReDone")
        .add(Category::Source, "DVD")
        .add(Category::VideoTerm, "10-bit");
    let parser_result = Parser::new("[ReDone] Memories Off 3.5 - 04 (DVD 10-bit).mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_477fa0065b9cf25444c1e77a9c971932a24d24a4() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Seirei Tsukai no Blade Dance")
        .add(Category::AnimeType, "SP")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "F1FF8588")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588].mkv",
        )
        .add(Category::ReleaseGroup, "FFF")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "720p");
    let parser_result =
        Parser::new("[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_a79804e0e0533f41906f64e4fec1478df71810c6() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Byousoku 5 Centimeter")
        .add(Category::AudioTerm, "AAC")
        .add(Category::AudioTerm, "2.0ch")
        .add(
            Category::FileName,
            "Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]",
        )
        .add(Category::Source, "Blu-Ray")
        .add(Category::Subtitles, "SOFTSUBS")
        .add(Category::VideoResolution, "1920x1080")
        .add(Category::VideoTerm, "H.264");
    let parser_result =
        Parser::new("Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e5d1d4214000f8ad313c1dae513abd30c58f39a9() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Swing out Sisters")
        .add(Category::AudioTerm, "AC3")
        .add(Category::FileChecksum, "3ABD57E6")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[SubDESU-H] Swing out Sisters Complete Version (720p x264 8bit AC3) [3ABD57E6].mp4",
        )
        .add(Category::ReleaseGroup, "SubDESU-H")
        .add(Category::ReleaseInformation, "Complete")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "8bit")
        .add(Category::VideoTerm, "x264");
    let parser_result = Parser::new(
        "[SubDESU-H] Swing out Sisters Complete Version (720p x264 8bit AC3) [3ABD57E6].mp4",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_de77fa00d1ef97a29247ea3f15f0aaf796746496() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Cyborg 009")
        .add(Category::AnimeYear, "1968")
        .add(Category::EpisodeNumber, "06")
        .add(Category::FileChecksum, "30C15D62")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "Cyborg 009 (1968) [TSHS] episode 06 [30C15D62].mp4",
        )
        .add(Category::EpisodePrefix, "episode")
        .add(Category::ReleaseGroup, "TSHS");
    let parser_result = Parser::new("Cyborg 009 (1968) [TSHS] episode 06 [30C15D62].mp4")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_88fb987ab2d0c4c3f1e0fbb78ccf87ae8dade9c7() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Dragon Ball Kai")
        .add(Category::AnimeYear, "2014")
        .add(Category::EpisodeNumber, "002")
        .add(Category::EpisodeNumberAlt, "100")
        .add(Category::FileChecksum, "DD66AFB7")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7].mkv",
        )
        .add(Category::ReleaseGroup, "Hatsuyuki")
        .add(Category::VideoResolution, "1280x720");
    let parser_result =
        Parser::new("[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
