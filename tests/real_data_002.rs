use anitomy_rust::elements::{Category, Elements};
///*
use anitomy_rust::Parser;
//use pretty_assertions::assert_eq;
#[test]
fn test_646e0468d2cb36fd7dfd257c932f8736a4573f90() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Sakura Taisen New York NY")
        .add(Category::EpisodeNumber, "2")
        .add(Category::FileChecksum, "1590D378")
        .add(Category::FileExtension, "avi")
        .add(Category::EpisodePrefix, "Ep")
        .add(
            Category::FileName,
            "[RNA]_Sakura_Taisen_New_York_NY_Ep_2_[1590D378].avi",
        )
        .add(Category::ReleaseGroup, "RNA");
    let parser_result = Parser::new("[RNA]_Sakura_Taisen_New_York_NY_Ep_2_[1590D378].avi")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_426e3d72c9494417190da9978cca880f0909ab99() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "2")
        .add(Category::AnimeSeasonPrefix, "Season")
        .add(Category::AnimeTitle, "Hayate no Gotoku")
        .add(Category::EpisodeNumber, "24")
        .add(
            Category::FileName,
            "Hayate no Gotoku 2nd Season 24 (Blu-Ray 1080p) [Chihiro]",
        )
        .add(Category::ReleaseGroup, "Chihiro")
        .add(Category::Source, "Blu-Ray")
        .add(Category::VideoResolution, "1080p");
    let parser_result = Parser::new("Hayate no Gotoku 2nd Season 24 (Blu-Ray 1080p) [Chihiro]")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b430f4c2b397ad9a2c5618d3b17df6dbfc3f2be0() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Blue Submarine No.6")
        .add(Category::AudioTerm, "Dual Audio")
        .add(
            Category::FileName,
            "[BluDragon] Blue Submarine No.6 (DVD, R2, Dual Audio) V3",
        )
        .add(Category::ReleaseGroup, "BluDragon")
        .add(Category::ReleaseVersion, "3")
        .add(Category::Source, "DVD");
    let parser_result = Parser::new("[BluDragon] Blue Submarine No.6 (DVD, R2, Dual Audio) V3")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_00b852a7673455f5642075312a194ec812977da7() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Chrono Crusade")
        .add(Category::EpisodeNumber, "1")
        .add(Category::EpisodeNumber, "5")
        .add(Category::EpisodePrefix, "ep")
        .add(Category::FileName, "Chrono Crusade ep. 1-5");
    let parser_result = Parser::new("Chrono Crusade ep. 1-5").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_f2c142999cc50d0db46a8701036477846d3cdbc2() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "2")
        .add(Category::AnimeSeasonPrefix, "Season")
        .add(Category::AnimeTitle, "Kimi ni Todoke")
        .add(Category::EpisodeNumber, "00")
        .add(Category::FileChecksum, "BF735BC4")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[gg]_Kimi_ni_Todoke_2nd_Season_-_00_[BF735BC4].mkv",
        )
        .add(Category::ReleaseGroup, "gg");
    let parser_result = Parser::new("[gg]_Kimi_ni_Todoke_2nd_Season_-_00_[BF735BC4].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_d69c48e65b272b8787e28c28dcf67b3bf2130ce2() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "K ON!")
        .add(Category::EpisodeNumber, "03")
        .add(Category::EpisodeTitle, "Training!")
        .add(Category::FileExtension, "mkv")
        .add(Category::EpisodePrefix, "Ep")
        .add(
            Category::FileName,
            "K-ON!_Ep03_Training!_[1080p,BluRay,x264]_-_THORA.mkv",
        )
        .add(Category::ReleaseGroup, "THORA")
        .add(Category::Source, "BluRay")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "x264");
    let parser_result = Parser::new("K-ON!_Ep03_Training!_[1080p,BluRay,x264]_-_THORA.mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_69705e61293253f162956cd27e03ea1fa1c606cb() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "K ON!!")
        .add(Category::EpisodeNumber, "08")
        .add(Category::EpisodeTitle, "Career Plan!")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "K-ON!!_Ep08_Career_Plan!_[1080p,BluRay,x264]_-_THORA.mkv",
        )
        .add(Category::ReleaseGroup, "THORA")
        .add(Category::Source, "BluRay")
        .add(Category::EpisodePrefix, "Ep")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "x264");
    let parser_result = Parser::new("K-ON!!_Ep08_Career_Plan!_[1080p,BluRay,x264]_-_THORA.mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_f54a871ee9cb15cf1eae7f35f9cf2e22866dade5() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Queen's Blade")
        .add(Category::AnimeSeason, "2")
        .add(Category::AnimeSeasonPrefix, "S")
        .add(Category::FileName, "[SFW]_Queen's_Blade_S2")
        .add(Category::ReleaseGroup, "SFW");
    let parser_result = Parser::new("[SFW]_Queen's_Blade_S2").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_fdcbd04dbc375cce2464df0d54776440f8535da2() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Evangelion 1.0 You Are [Not] Alone")
        .add(
            Category::FileName,
            "Evangelion_1.0_You_Are_[Not]_Alone_(1080p)_[@Home]",
        )
        .add(Category::ReleaseGroup, "@Home")
        .add(Category::VideoResolution, "1080p");
    let parser_result = Parser::new("Evangelion_1.0_You_Are_[Not]_Alone_(1080p)_[@Home]")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_86b6b1c4809a472b4a4a7b766c0b89f261447b98() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Infinite Stratos - IS")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "29675B71")
        .add(Category::FileExtension, "avi")
        .add(
            Category::FileName,
            "[Ayako]_Infinite_Stratos_-_IS_-_01v2_[XVID][400p][29675B71].avi",
        )
        .add(Category::ReleaseGroup, "Ayako")
        .add(Category::ReleaseVersion, "2")
        .add(Category::VideoResolution, "400p")
        .add(Category::VideoTerm, "XVID");
    let parser_result =
        Parser::new("[Ayako]_Infinite_Stratos_-_IS_-_01v2_[XVID][400p][29675B71].avi")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_cd72195ce00e32e469228957fdb239be8b4d2dbe() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Kore wa Zombie desu ka")
        .add(Category::AnimeType, "TV")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "03")
        .add(Category::FileChecksum, "888E4991")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[E-HARO Raws] Kore wa Zombie desu ka - 03 (TV 1280x720 h264 AAC) [888E4991].mkv",
        )
        .add(Category::ReleaseGroup, "E-HARO Raws")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "h264");
    let parser_result = Parser::new(
        "[E-HARO Raws] Kore wa Zombie desu ka - 03 (TV 1280x720 h264 AAC) [888E4991].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_016f9c52da3573e7b866a070c66149925993fab9() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Kore wa Zombie desu ka")
        .add(Category::EpisodeNumber, "2")
        .add(Category::FileExtension, "mkv")
        .add(Category::EpisodePrefix, "Episode")
        .add(
            Category::FileName,
            "[Edomae Subs] Kore wa Zombie desu ka  Episode 2.mkv",
        )
        .add(Category::ReleaseGroup, "Edomae Subs");
    let parser_result = Parser::new("[Edomae Subs] Kore wa Zombie desu ka  Episode 2.mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_3fe99b24e38265608a8d13a19e128a384301fdc3() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Juuni Kokki")
        .add(Category::EpisodeNumber, "5")
        .add(Category::FileExtension, "avi")
        .add(Category::EpisodePrefix, "Ep")
        .add(Category::FileName, "Juuni.Kokki.Ep.5.avi");
    let parser_result = Parser::new("Juuni.Kokki.Ep.5.avi").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_f667845cbce26cea677eb3233d0bea87adab9f1c() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Juuni Kokki")
        .add(Category::EpisodeNumber, "5")
        .add(Category::FileExtension, "avi")
        .add(Category::EpisodePrefix, "Ep")
        .add(Category::FileName, "Juuni Kokki Ep.5.avi");
    let parser_result = Parser::new("Juuni Kokki Ep.5.avi").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
/// ! False positive
fn test_de94454023357e98fbca89236fa9e9f371235be5() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Keroro")
        .add(Category::AudioTerm, "mp3")
        .add(Category::EpisodeNumber, "148")
        .add(Category::FileChecksum, "FE68D5F1")
        .add(Category::FileExtension, "avi")
        .add(Category::FileName, "[Keroro].148.[Xvid.mp3].[FE68D5F1].avi")
        .add(Category::VideoTerm, "Xvid");
    let parser_result = Parser::new("[Keroro].148.[Xvid.mp3].[FE68D5F1].avi")
        .parse()
        .unwrap();
    assert_ne!(wanted, parser_result);
}
#[test]
fn test_2e5b0092da76cb1c46a1691b4b998041fddbe337() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "5 centimeters per second")
        .add(Category::AudioTerm, "flac")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "5_centimeters_per_second[1904x1072.h264.flac][niizk].mkv",
        )
        .add(Category::ReleaseGroup, "niizk")
        .add(Category::VideoResolution, "1904x1072")
        .add(Category::VideoTerm, "h264");
    let parser_result = Parser::new("5_centimeters_per_second[1904x1072.h264.flac][niizk].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_3186236f32ef68fabfc387320a2b1903a1e07910() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "009 1")
        .add(Category::EpisodeNumber, "02")
        .add(Category::FileChecksum, "36D2444D")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Yoroshiku]_009-1_-_02_(H264)_[36D2444D].mkv",
        )
        .add(Category::ReleaseGroup, "Yoroshiku")
        .add(Category::VideoTerm, "H264");
    let parser_result = Parser::new("[Yoroshiku]_009-1_-_02_(H264)_[36D2444D].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_db9c34ba38bdd65c4611e0a36b9901d4c3e4bfd2() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "1")
        .add(Category::AnimeTitle, "After War Gundam X")
        .add(Category::EpisodeNumber, "03")
        .add(Category::EpisodeTitle, "My Mount is Fierce!")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "After War Gundam X - 1x03 - My Mount is Fierce!.mkv",
        );
    let parser_result = Parser::new("After War Gundam X - 1x03 - My Mount is Fierce!.mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_d2b08ea827ac14bf677a50367179b6f664155090() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "The World God Only Knows 2")
        .add(Category::EpisodeNumber, "03")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[HorribleSubs] The World God Only Knows 2 - 03 [720p].mkv",
        )
        .add(Category::ReleaseGroup, "HorribleSubs")
        .add(Category::VideoResolution, "720p");
    let parser_result = Parser::new("[HorribleSubs] The World God Only Knows 2 - 03 [720p].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_dd2920c477af6235e52c650943ef6a15275ddfbb() {
    let wanted = Elements::new()
        .add(
            Category::AnimeTitle,
            "Macross Frontier - Sayonara no Tsubasa",
        )
        .add(Category::FileChecksum, "46B35E25")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Macross Frontier - Sayonara no Tsubasa (Central Anime, 720p) [46B35E25].mkv",
        )
        .add(Category::ReleaseGroup, "Central Anime")
        .add(Category::VideoResolution, "720p");
    let parser_result =
        Parser::new("Macross Frontier - Sayonara no Tsubasa (Central Anime, 720p) [46B35E25].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_5ea8a08f4549bad82733bba167cd0a4e5f441c13() {
    let wanted = Elements::new()
        .add(Category::AnimeYear, "2012")
        .add(Category::AnimeTitle, "Space Battleship Yamato 2199")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "18")
        .add(Category::FileChecksum, "BA70BA9C")
        .add(
            Category::FileName,
            "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 8 bit AAC)[BA70BA9C]",
        )
        .add(Category::ReleaseGroup, "Nubles")
        .add(Category::VideoResolution, "720p")
        .add(Category::EpisodePrefix, "episode")
        .add(Category::VideoTerm, "8-bit");
    let parser_result = Parser::new(
        "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 8 bit AAC)[BA70BA9C]",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e2b638338c1c50f00632b0fc46356ad8b07c35b2() {
    let wanted = Elements::new()
        .add(Category::AnimeYear, "2012")
        .add(Category::AnimeTitle, "Space Battleship Yamato 2199")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "18")
        .add(Category::FileChecksum, "1F56D642")
        .add(
            Category::FileName,
            "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 10 bit AAC)[1F56D642]",
        )
        .add(Category::EpisodePrefix, "episode")
        .add(Category::ReleaseGroup, "Nubles")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "10-bit");
    let parser_result = Parser::new(
        "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 10 bit AAC)[1F56D642]",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_6f5dd135ceb0793f437f616f5683ea9d2004ef0e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Red Data Girl")
        .add(Category::EpisodeNumber, "10")
        .add(Category::FileChecksum, "29EA865B")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[FFF] Red Data Girl - 10v0 [29EA865B].mkv",
        )
        .add(Category::ReleaseGroup, "FFF")
        .add(Category::ReleaseVersion, "0");
    let parser_result = Parser::new("[FFF] Red Data Girl - 10v0 [29EA865B].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_c647d1b3e9961fbc2445d55bac57aa1ac06e623e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Magical☆Star Kanon 100% OVA")
        .add(Category::AnimeType, "OVA")
        .add(Category::FileChecksum, "E9F43685")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[CMS] Magical☆Star Kanon 100% OVA[DVD][E9F43685].mkv",
        )
        .add(Category::ReleaseGroup, "CMS")
        .add(Category::Source, "DVD");
    let parser_result = Parser::new("[CMS] Magical☆Star Kanon 100% OVA[DVD][E9F43685].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_2210de27c48b0483063791855d0b919676edd0f5() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Ro Kyu Bu! SS")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "C1B5CE5D")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Doremi].Ro-Kyu-Bu!.SS.01.[C1B5CE5D].mkv",
        )
        .add(Category::ReleaseGroup, "Doremi");
    let parser_result = Parser::new("[Doremi].Ro-Kyu-Bu!.SS.01.[C1B5CE5D].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e6b05915cee55dca0bc08b6b8c240449e8c32b79() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Persona 4 The Animation")
        .add(Category::AudioTerm,"FLAC")
        .add(Category::EpisodeNumber,"13")
        .add(Category::EpisodeTitle,"A Stormy Summer Vacation Part 1")
        .add(Category::FileChecksum,"8A45634B")
        .add(Category::FileExtension,"mkv")
        .add(Category::FileName,"[Raizel] Persona 4 The Animation Episode 13 - A Stormy Summer Vacation Part 1  [BD_1080p_Dual_Audio_FLAC_Hi10p][8A45634B].mkv")
        .add(Category::ReleaseGroup,"Raizel")
        .add(Category::Source,"BD")
        .add(Category::AudioTerm, "Dual Audio")
        .add(Category::EpisodePrefix, "Episode")
        .add(Category::VideoResolution,"1080p")
        .add(Category::VideoTerm,"Hi10p")
    ;
    let parser_result = Parser::new("[Raizel] Persona 4 The Animation Episode 13 - A Stormy Summer Vacation Part 1  [BD_1080p_Dual_Audio_FLAC_Hi10p][8A45634B].mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_3fbdbf88417a9ff0a056d8f0bb44247d06069070() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Kotoura san - Special Short Anime 'Haruka's Room'")
        .add(Category::AnimeType,"Special")
        .add(Category::AudioTerm,"AAC")
        .add(Category::EpisodeNumber,"01")
        .add(Category::FileChecksum,"6B6BE015")
        .add(Category::FileExtension,"mkv")
        .add(Category::FileName,"[Hien] Kotoura-san - Special Short Anime 'Haruka's Room' - 01 [BD 1080p H.264 10-bit AAC][6B6BE015].mkv")
        .add(Category::ReleaseGroup,"Hien")
        .add(Category::Source,"BD")
        .add(Category::VideoResolution,"1080p")
        .add(Category::VideoTerm,"H.264")
        .add(Category::VideoTerm, "10-bit")
    ;
    let parser_result = Parser::new("[Hien] Kotoura-san - Special Short Anime 'Haruka's Room' - 01 [BD 1080p H.264 10-bit AAC][6B6BE015].mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_6161f034f1c598f8985d5f224a371ce16f9c5d53() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Diebuster")
        .add(Category::AudioTerm, "AC3")
        .add(Category::EpisodeNumber, "1")
        .add(Category::FileChecksum, "82E36A36")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[R-R] Diebuster.EP1 (720p.Hi10p.AC3) [82E36A36].mkv",
        )
        .add(Category::EpisodePrefix, "EP")
        .add(Category::ReleaseGroup, "R-R")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "Hi10p");
    let parser_result = Parser::new("[R-R] Diebuster.EP1 (720p.Hi10p.AC3) [82E36A36].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_59258c2b46b31fe8843a482d1ee7a338a504a7be() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Aim For The Top! Gunbuster")
        .add(Category::AudioTerm, "FLAC")
        .add(Category::EpisodeNumber, "1")
        .add(Category::FileChecksum, "69ECCDCF")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Aim_For_The_Top!_Gunbuster-ep1.BD(H264.FLAC.10bit)[KAA][69ECCDCF].mkv",
        )
        .add(Category::ReleaseGroup, "KAA")
        .add(Category::Source, "BD")
        .add(Category::VideoTerm, "H264")
        .add(Category::VideoTerm, "10bit")
        .add(Category::EpisodePrefix, "ep");
    let parser_result =
        Parser::new("Aim_For_The_Top!_Gunbuster-ep1.BD(H264.FLAC.10bit)[KAA][69ECCDCF].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_9b67622f3fedd2237b7a20d1c6fe73158c65c6a5() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Gift ~eternal rainbow~")
        .add(Category::AudioTerm, "vorbis")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Rakuda].Gift.~eternal.rainbow~.01.dvd.h.264.vorbis.mkv",
        )
        .add(Category::ReleaseGroup, "Rakuda")
        .add(Category::Source, "dvd")
        .add(Category::VideoTerm, "h.264");
    let parser_result = Parser::new("[Rakuda].Gift.~eternal.rainbow~.01.dvd.h.264.vorbis.mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_56655a6129097936f83d59e027a5ea573816092a() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "D.C.II Da Capo II")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "a1fc58a7")
        .add(Category::FileExtension, "mkv")
        .add(Category::EpisodePrefix, "Ep")
        .add(
            Category::FileName,
            "[Jumonji-Giri]_[Shinsen-Subs][ASF]_D.C.II_Da_Capo_II_Ep01_(a1fc58a7).mkv",
        )
        .add(Category::ReleaseGroup, "Jumonji-Giri");
    let parser_result =
        Parser::new("[Jumonji-Giri]_[Shinsen-Subs][ASF]_D.C.II_Da_Capo_II_Ep01_(a1fc58a7).mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_acd883f9eccb02ab18a9e6b14fed84f66f3675ec() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Mobile Suit Gundam Seed Destiny")
        .add(Category::AudioTerm,"AAC")
        .add(Category::EpisodeNumber,"07")
        .add(Category::FileExtension,"mp4")
        .add(Category::FileName,"[Mobile Suit Gundam Seed Destiny HD REMASTER][07][Big5][720p][AVC_AAC][encoded by SEED].mp4")
        .add(Category::Other,"REMASTER")
        .add(Category::Subtitles,"Big5")
        .add(Category::ReleaseGroup,"SEED")
        .add(Category::VideoResolution,"720p")
        .add(Category::VideoTerm,"AVC")
        .add(Category::VideoTerm, "HD")
    ;
    let parser_result = Parser::new("[Mobile Suit Gundam Seed Destiny HD REMASTER][07][Big5][720p][AVC_AAC][encoded by SEED].mp4").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_9a0190850245e1cf14416686e6ca0a0e330e286f() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "「K」")
        .add(Category::AudioTerm, "AAC")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "「K」 Image Blu-ray WHITE & BLACK - Main (BD 1280x720 AVC AAC).mp4",
        )
        .add(Category::Source, "Blu-ray")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "AVC");
    let parser_result =
        Parser::new("「K」 Image Blu-ray WHITE & BLACK - Main (BD 1280x720 AVC AAC).mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e02b9c2e5a46f19a3e1183c0b40ddeccca6a67d5() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Shingeki no Kyojin")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "05")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[[Zero-Raws] Shingeki no Kyojin - 05 (MBS 1280x720 x264 AAC).mp4",
        )
        .add(Category::ReleaseGroup, "Zero-Raws")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("[[Zero-Raws] Shingeki no Kyojin - 05 (MBS 1280x720 x264 AAC).mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_1f61ba07afb63d45c24b66203a2e20d6675ceed7() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "SlamDunk")
        .add(Category::AudioTerm, "aac")
        .add(Category::EpisodeNumber, "001")
        .add(Category::FileChecksum, "7FE2C873")
        .add(Category::FileExtension, "mkv")
        .add(Category::Language, "Jpn")
        .add(
            Category::FileName,
            "[52wy][SlamDunk][001][Jpn_Chs_Cht][x264_aac][DVDRip][7FE2C873].mkv",
        )
        .add(Category::ReleaseGroup, "52wy")
        .add(Category::Source, "DVDRip")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("[52wy][SlamDunk][001][Jpn_Chs_Cht][x264_aac][DVDRip][7FE2C873].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_cf9fddc5af8182156029518ef92baccdced8747e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Last Exile ~Fam The Silver Wing~")
        .add(Category::EpisodeNumber, "13")
        .add(Category::FileChecksum, "AFF9E530")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Commie] Last Exile ~Fam, The Silver Wing~ - 13 [AFF9E530].mkv",
        )
        .add(Category::ReleaseGroup, "Commie");
    let parser_result =
        Parser::new("[Commie] Last Exile ~Fam, The Silver Wing~ - 13 [AFF9E530].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_21371665c3198f048de7ee928301540f8018a27c() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Dragon Ball Z Battle of Gods")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[Hakugetsu&Speed&MGRT][Dragon_Ball_Z_Battle_of_Gods][BDRIP][BIG5][1280x720].mp4",
        )
        .add(Category::ReleaseGroup, "Hakugetsu&Speed&MGRT")
        .add(Category::Source, "BDRIP")
        .add(Category::Subtitles, "BIG5")
        .add(Category::VideoResolution, "1280x720");
    let parser_result = Parser::new(
        "[Hakugetsu&Speed&MGRT][Dragon_Ball_Z_Battle_of_Gods][BDRIP][BIG5][1280x720].mp4",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_77fd2d158b07ca1045530f5fda8f8b7eefdb9950() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Evangelion 3.0 You Can (Not)")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[Hakugetsu&MGRT][Evangelion 3.0 You Can (Not) Redo][480P][V0].mp4",
        )
        .add(Category::ReleaseGroup, "Hakugetsu&MGRT")
        .add(Category::ReleaseVersion, "0")
        .add(Category::VideoResolution, "480P");
    let parser_result =
        Parser::new("[Hakugetsu&MGRT][Evangelion 3.0 You Can (Not) Redo][480P][V0].mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b01c83f84abaa786b419c2b27f8c54f5102d8141() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Kidou Senshi Gundam UC Unicorn")
        .add(Category::AudioTerm, "5.1ch")
        .add(Category::AudioTerm,"AAC")
        .add(Category::EpisodeNumber,"02")
        .add(Category::FileExtension,"mp4")
        .add(Category::FileName,"[TV-J] Kidou Senshi Gundam UC Unicorn - episode.02 [BD 1920x1080 h264+AAC(5.1ch JP+EN) +Sub(JP-EN-SP-FR-CH) Chap].mp4")
        .add(Category::ReleaseGroup,"TV-J")
        .add(Category::Source,"BD")
        .add(Category::VideoResolution,"1920x1080")
        .add(Category::VideoTerm,"h264")
        .add(Category::EpisodePrefix, "episode")
    ;
    let parser_result = Parser::new("[TV-J] Kidou Senshi Gundam UC Unicorn - episode.02 [BD 1920x1080 h264+AAC(5.1ch JP+EN) +Sub(JP-EN-SP-FR-CH) Chap].mp4").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_7d8ea68cb0630f0aacf23b36efcc7b25fca5dfbe() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Fate Zero")
        .add(Category::AudioTerm, "AC3")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "02A0491D")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[UTW]_Fate_Zero_-_01_[BD][h264-720p_AC3][02A0491D].mkv",
        )
        .add(Category::ReleaseGroup, "UTW")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "h264");
    let parser_result = Parser::new("[UTW]_Fate_Zero_-_01_[BD][h264-720p_AC3][02A0491D].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_de5e8c40e1e023c81ff588d3b06695eaa4cc737d() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Evangelion 3.33 You Can (Not) Redo")
        .add(Category::AudioTerm, "flac")
        .add(Category::FileChecksum, "F2060CF5")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[UTW-THORA] Evangelion 3.33 You Can (Not) Redo [BD][1080p,x264,flac][F2060CF5].mkv",
        )
        .add(Category::ReleaseGroup, "UTW-THORA")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "x264");
    let parser_result = Parser::new(
        "[UTW-THORA] Evangelion 3.33 You Can (Not) Redo [BD][1080p,x264,flac][F2060CF5].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_3f4c5bc49dd7e9e0382950d97623dee1034c712e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Shingeki no Kyojin")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "25")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[Zero-Raws] Shingeki no Kyojin - 25 END (MBS 1280x720 x264 AAC).mp4",
        )
        .add(Category::ReleaseGroup, "Zero-Raws")
        .add(Category::ReleaseInformation, "END")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("[Zero-Raws] Shingeki no Kyojin - 25 END (MBS 1280x720 x264 AAC).mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_1cc9c4e9e73d173f18e1838bbbdc5043576fd213() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Bakemonogatari")
        .add(Category::AudioTerm, "AACx2")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "Bakemonogatari - 01 (BD 1280x720 AVC AACx2).mp4",
        )
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "AVC");
    let parser_result = Parser::new("Bakemonogatari - 01 (BD 1280x720 AVC AACx2).mp4")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_59a5b6fa28ecfdeeb782f68b00512dd58a0aa83e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Evangelion Shin Gekijouban Q")
        .add(Category::AnimeType, "Gekijouban")
        .add(Category::AudioTerm, "FLACx2")
        .add(Category::AudioTerm, "5.1ch")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Evangelion Shin Gekijouban Q (BDrip 1920x1080 x264 FLACx2 5.1ch)-ank.mkv",
        )
        .add(Category::EpisodeTitle, "ank")
        .add(Category::Source, "BDrip")
        .add(Category::VideoResolution, "1920x1080")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("Evangelion Shin Gekijouban Q (BDrip 1920x1080 x264 FLACx2 5.1ch)-ank.mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_2381689c40f6372d7e4c2ea8fa644a06a5f20001() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Evangelion The New Movie Q")
        .add(Category::AnimeType, "Movie")
        .add(Category::AudioTerm, "AACx2")
        .add(Category::AudioTerm, "5.1")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "Evangelion The New Movie Q (BD 1280x720 AVC AACx2 [5.1+2.0]).mp4",
        )
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "AVC");
    let parser_result =
        Parser::new("Evangelion The New Movie Q (BD 1280x720 AVC AACx2 [5.1+2.0]).mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_eb302b3d4af37a92584d9217e24f1efe08db24ee() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Howl's Moving Castle")
        .add(Category::AnimeYear, "2004")
        .add(Category::AudioTerm, "flac")
        .add(Category::AudioTerm, "dts")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2.mkv",
        )
        .add(Category::ReleaseGroup, "THORA")
        .add(Category::ReleaseVersion, "2")
        .add(Category::Source, "BluRay")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2.mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_0e94140173c5815fcac42545ae1fe2d3d28509bc() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Kotonoha no Niwa")
        .add(Category::AudioTerm, "AACx3")
        .add(Category::FileExtension, "mp4")
        .add(Category::AudioTerm, "5.1")
        .add(
            Category::FileName,
            "Kotonoha no Niwa (BD 1280x720 AVC AACx3 [5.1+2.0+2.0] Subx3).mp4",
        )
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "AVC");
    let parser_result =
        Parser::new("Kotonoha no Niwa (BD 1280x720 AVC AACx3 [5.1+2.0+2.0] Subx3).mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_70eac2a5caaa6550e6f36f1cfd88159e06dc19ea() {
    let wanted = Elements::new()
        .add(
            Category::AnimeTitle,
            "Queen's Blade Utsukushiki Toushi tachi - OVA",
        )
        .add(Category::AnimeType, "OVA")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "Queen's Blade Utsukushiki Toushi-tachi - OVA_01 (BD 1280x720 AVC AAC).mp4",
        )
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "AVC");
    let parser_result =
        Parser::new("Queen's Blade Utsukushiki Toushi-tachi - OVA_01 (BD 1280x720 AVC AAC).mp4")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_757776fb856dcbf79b0ff6a5e3c917aa71f26511() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Golden Time")
        .add(Category::EpisodeNumber, "24")
        .add(
            Category::FileName,
            "【MMZYSUB】★【Golden Time】[24（END）][GB][720P_MP4]",
        )
        .add(Category::ReleaseGroup, "MMZYSUB")
        .add(Category::ReleaseInformation, "END")
        .add(Category::VideoResolution, "720P")
        .add(Category::VideoTerm, "MP4");
    let parser_result = Parser::new("【MMZYSUB】★【Golden Time】[24（END）][GB][720P_MP4]")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_0098579e104b94d5985c4d96cd36122d308e547a() {
    let wanted = Elements::new()
        .add(
            Category::AnimeTitle,
            "Futsuu no Joshikousei ga [Locodol] Yatte Mita",
        )
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "BAD09C76")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[FFF] Futsuu no Joshikousei ga [Locodol] Yatte Mita. - 01 [BAD09C76].mkv",
        )
        .add(Category::ReleaseGroup, "FFF");
    let parser_result =
        Parser::new("[FFF] Futsuu no Joshikousei ga [Locodol] Yatte Mita. - 01 [BAD09C76].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
//*/
