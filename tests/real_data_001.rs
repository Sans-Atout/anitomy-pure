use anitomy_rust::elements::{Category, Elements};
use anitomy_rust::Parser;
use pretty_assertions::assert_eq;
#[test]
fn test_7c4169a80d0f50c05618431ecd403006d78a1c99() {
    let wanted = Elements::new()
		.add(Category::AnimeTitle,"Toradora!")
		.add(Category::AnimeYear,"2008")
		.add(Category::AudioTerm,"FLAC")
		.add(Category::EpisodeNumber,"01")
		.add(Category::EpisodeTitle,"Tiger and Dragon")
		.add(Category::FileChecksum,"1234ABCD")
		.add(Category::FileExtension,"mkv")
		.add(Category::FileName,"[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv")
		.add(Category::ReleaseGroup,"TaigaSubs")
		.add(Category::ReleaseVersion,"2")
		.add(Category::VideoResolution,"1280x720")
		.add(Category::VideoTerm,"H.264")
	;
    let parser_result = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_d641f27e8153dad93bdeda4e59ac82b5988942a8() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Princess Lover!")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "2048A39A")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[ANBU]_Princess_Lover!_-_01_[2048A39A].mkv",
        )
        .add(Category::ReleaseGroup, "ANBU");
    let parser_result = Parser::new("[ANBU]_Princess_Lover!_-_01_[2048A39A].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e22aafc921053b45fb6441dc018cbac0aeaec7d5() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Canaan")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "12F00E89")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[ANBU-Menclave]_Canaan_-_01_[1024x576_H.264_AAC][12F00E89].mkv",
        )
        .add(Category::ReleaseGroup, "ANBU-Menclave")
        .add(Category::VideoResolution, "1024x576")
        .add(Category::VideoTerm, "H.264");
    let parser_result =
        Parser::new("[ANBU-Menclave]_Canaan_-_01_[1024x576_H.264_AAC][12F00E89].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_f42fe1e2a97c9eb5bf923a1f77172259ad6c1193() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Haiyoru! Nyaru Ani")
        .add(Category::FileChecksum, "596DD8E6")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[ANBU-umai]_Haiyoru!_Nyaru-Ani_[596DD8E6].mkv",
        )
        .add(Category::ReleaseGroup, "ANBU-umai");
    let parser_result = Parser::new("[ANBU-umai]_Haiyoru!_Nyaru-Ani_[596DD8E6].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
/// ! False positive
fn test_fc92da9e05a8dc196678d3ca567a98186a6825b7() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Special A")
        .add(Category::AnimeType, "Special")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "C83164B9")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[BakaWolf-m.3.3.w] Special A 01 (H.264) [C83164B9].mkv",
        )
        .add(Category::ReleaseGroup, "BakaWolf-m.3.3.w")
        .add(Category::VideoTerm, "H.264");
    let parser_result = Parser::new("[BakaWolf-m.3.3.w] Special A 01 (H.264) [C83164B9].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_171957d4b60ddbbf14fe1affa132fa7e26f9577e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Seikon no Qwaser")
        .add(Category::EpisodeNumber, "13")
        .add(Category::FileChecksum, "988DB090")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[chibi-Doki] Seikon no Qwaser - 13v0 (Uncensored Director's Cut) [988DB090].mkv",
        )
        .add(Category::Other, "Uncensored")
        .add(Category::ReleaseGroup, "chibi-Doki")
        .add(Category::ReleaseVersion, "0");
    let parser_result = Parser::new(
        "[chibi-Doki] Seikon no Qwaser - 13v0 (Uncensored Director's Cut) [988DB090].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_8073a3aedd0b60e580d9cc0cbcbb6932781be11c() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Kono Aozora ni Yakusoku Wo")
        .add(Category::EpisodeNumber, "10")
        .add(Category::FileChecksum, "C83D206B")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B].mkv",
        )
        .add(Category::ReleaseGroup, "Chihiro")
        .add(Category::ReleaseVersion, "2")
        .add(Category::Source, "DVD")
        .add(Category::VideoTerm, "h264");
    let parser_result =
        Parser::new("[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e6e29c702c64d4c7d8f4af05454c36fcc34583b5() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Toradora")
        .add(Category::AnimeType, "ED")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "2")
        .add(Category::FileChecksum, "3B65D1E6")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Coalgirls]_Toradora_ED2_(704x480_DVD_AAC)_[3B65D1E6].mkv",
        )
        .add(Category::ReleaseGroup, "Coalgirls")
        .add(Category::Source, "DVD")
        .add(Category::VideoResolution, "704x480");
    let parser_result = Parser::new("[Coalgirls]_Toradora_ED2_(704x480_DVD_AAC)_[3B65D1E6].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_630651d35f622f9f766e3f98578d3a8a614f1d33() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Mobile Suit Gundam 00")
        .add(Category::AnimeSeasonPrefix, "S")
        .add(Category::AnimeSeason, "2")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "4863FBE8")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Conclave-Mendoi]_Mobile_Suit_Gundam_00_S2_-_01v2_[1280x720_H.264_AAC][4863FBE8].mkv",
        )
        .add(Category::ReleaseGroup, "Conclave-Mendoi")
        .add(Category::ReleaseVersion, "2")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "H.264");
    let parser_result = Parser::new(
        "[Conclave-Mendoi]_Mobile_Suit_Gundam_00_S2_-_01v2_[1280x720_H.264_AAC][4863FBE8].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_3110eeefa9ddc7583ce7fd297c5f632af402c2c2() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Bleach")
        .add(Category::EpisodeNumber, "225")
        .add(Category::FileChecksum, "C63D149C")
        .add(Category::FileExtension, "avi")
        .add(Category::FileName, "[DB]_Bleach_225_[C63D149C].avi")
        .add(Category::ReleaseGroup, "DB");
    let parser_result = Parser::new("[DB]_Bleach_225_[C63D149C].avi")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_16f12bb8d06fa6d060fdfe512332ebecee10ae76() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Nodame Cantabile Finale")
        .add(Category::EpisodeNumber, "00")
        .add(Category::FileChecksum, "73AD0735")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Frostii]_Nodame_Cantabile_Finale_-_00_[73AD0735].mkv",
        )
        .add(Category::ReleaseGroup, "Frostii");
    let parser_result = Parser::new("[Frostii]_Nodame_Cantabile_Finale_-_00_[73AD0735].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_da643c5d3afccac755d62c774ddcda7f69d1f1f1() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "FullMetalAlchemist")
        .add(Category::EpisodeNumber, "09")
        .add(Category::FileExtension, "rmvb")
        .add(
            Category::FileName,
            "[Hard-Boiled FS]FullMetalAlchemist_09.rmvb",
        )
        .add(Category::ReleaseGroup, "Hard-Boiled FS");
    let parser_result = Parser::new("[Hard-Boiled FS]FullMetalAlchemist_09.rmvb")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_a2ad6f3ebba7b8c3807d4532e7ea94ffed8d481f() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Tower of Druaga - Sword of Uruk")
        .add(Category::EpisodeNumber, "04")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p].mkv",
        )
        .add(Category::ReleaseGroup, "HorribleSubs")
        .add(Category::VideoResolution, "480p");
    let parser_result =
        Parser::new("[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
/// ! Know false positive
/// Here, Juuni Kokki as "Les 12 Royaumes" is the anime name
fn test_49be603afd2b77129cc736f3f062e8e3cb48f918() {
    let wanted = Elements::new()
        .add(Category::ReleaseGroup, "Juuni.Kokki")
        .add(Category::AnimeTitle, "Les 12 Royaumes")
        .add(Category::AudioTerm, "OGG")
        .add(Category::EpisodeNumber, "24")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Juuni.Kokki]-(Les.12.Royaumes)-[Ep.24]-[x264+OGG]-[JAP+FR+Sub.FR]-[Chap]-[AzF].mkv",
        )
        .add(Category::Language, "JAP")
        .add(Category::Language, "FR")
        .add(Category::Subtitles, "Sub")
        .add(Category::VideoTerm, "x264")
        .add(Category::EpisodePrefix, "Ep");
    let parser_result = Parser::new(
        "[Juuni.Kokki]-(Les.12.Royaumes)-[Ep.24]-[x264+OGG]-[JAP+FR+Sub.FR]-[Chap]-[AzF].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_581972780a0f66afb7363c403a40dac3cb09675b() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "One Piece")
        .add(Category::VolumePrefix, "Movie")
        .add(Category::VolumeNumber, "9")
        .add(Category::FileExtension, "avi")
        .add(
            Category::FileName,
            "[KAF-TEAM]_One_Piece_Movie_9_vostfr_HD.avi",
        )
        .add(Category::Language, "vostfr")
        .add(Category::ReleaseGroup, "KAF-TEAM")
        .add(Category::VideoTerm, "HD");
    let parser_result = Parser::new("[KAF-TEAM]_One_Piece_Movie_9_vostfr_HD.avi")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_989b47b818d1abebd279af59793bdc22b8525ce9() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Nazca")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[kito].Nazca.episode.01.DVDRip.[x264.He-aac.{Jpn}+Sub{Fr}].mkv",
        )
        .add(Category::AudioTerm, "He-aac")
        .add(Category::Subtitles, "Sub")
        .add(Category::Language, "Fr")
        .add(Category::Language, "Jpn")
        .add(Category::EpisodePrefix, "episode")
        .add(Category::ReleaseGroup, "kito")
        .add(Category::Source, "DVDRip")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("[kito].Nazca.episode.01.DVDRip.[x264.He-aac.{Jpn}+Sub{Fr}].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_450d85cd820eba74358122537139fc8ec59df9dc() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Umineko no Naku Koro ni")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "11")
        .add(Category::FileChecksum, "943106AD")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Lambda-Delta]_Umineko_no_Naku_Koro_ni_-_11_[848x480_H.264_AAC][943106AD].mkv",
        )
        .add(Category::ReleaseGroup, "Lambda-Delta")
        .add(Category::VideoResolution, "848x480")
        .add(Category::VideoTerm, "H.264");
    let parser_result = Parser::new(
        "[Lambda-Delta]_Umineko_no_Naku_Koro_ni_-_11_[848x480_H.264_AAC][943106AD].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_881258acd47a36a5c14beaf45a4301f9700be437() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Kemono no Souja Erin")
        .add(Category::EpisodeNumber, "12")
        .add(Category::FileChecksum, "0F5F884F")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[SS]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F].mkv",
        )
        .add(Category::ReleaseGroup, "SS")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "h264");
    let parser_result =
        Parser::new("[SS]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_f8bcf31f8721eff0cfd32a67502c177e96e60bc9() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Fullmetal Alchemist")
        .add(Category::AnimeYear, "2009")
        .add(Category::EpisodeNumber, "04")
        .add(Category::FileChecksum, "40F2A957")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4",
        )
        .add(Category::ReleaseGroup, "Taka")
        .add(Category::VideoResolution, "720p");
    let parser_result = Parser::new("[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_1a71b7659bb43123c50a11aafdda0d3f18ae212c() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Summer Wars")
        .add(Category::AudioTerm, "TrueHD5.1")
        .add(Category::FileChecksum, "9F311DAB")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[UTW-TMD]_Summer_Wars_[BD][h264-720p][TrueHD5.1][9F311DAB].mkv",
        )
        .add(Category::ReleaseGroup, "UTW-TMD")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "h264");
    let parser_result =
        Parser::new("[UTW-TMD]_Summer_Wars_[BD][h264-720p][TrueHD5.1][9F311DAB].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_7df6249dfedf81bc8413af928891b82e46d185d4() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "First Squad The Morment Of Truth")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[ValdikSS]_First_Squad_The_Morment_Of_Truth_[720x576_h264_dvdscr_eng_hardsub].mkv",
        )
        .add(Category::Language, "eng")
        .add(Category::ReleaseGroup, "ValdikSS")
        .add(Category::Subtitles, "hardsub")
        .add(Category::VideoResolution, "720x576")
        .add(Category::VideoTerm, "h264");
    let parser_result = Parser::new(
        "[ValdikSS]_First_Squad_The_Morment_Of_Truth_[720x576_h264_dvdscr_eng_hardsub].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_62738764d5a9b418fd8ad7e56aadba512c814899() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Code Geass R2 TV")
        .add(Category::AnimeType, "TV")
        .add(Category::EpisodeNumber, "20")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Code_Geass_R2_TV_[20_of_25]_[ru_jp]_[HDTV]_[Varies_&_Cuba77_&_AnimeReactor_RU].mkv",
        )
        .add(Category::Language, "ru")
        .add(Category::Language, "jp")
        .add(Category::ReleaseGroup, "Varies_&_Cuba77_&_AnimeReactor_RU")
        .add(Category::Source, "HDTV");
    let parser_result = Parser::new(
        "Code_Geass_R2_TV_[20_of_25]_[ru_jp]_[HDTV]_[Varies_&_Cuba77_&_AnimeReactor_RU].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_d978b59458a1dcb40afc9dcafacfea3b127dc0a9() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Evangelion 1.11 You Are (Not) Alone")
        .add(Category::AnimeYear, "2009")
        .add(Category::AudioTerm, "DTS-ES")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Evangelion_1.11_You_Are_(Not)_Alone_(2009)_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv",
        )
        .add(Category::ReleaseGroup, "THORA")
        .add(Category::Source, "BluRay")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "x264");
    let parser_result = Parser::new(
        "Evangelion_1.11_You_Are_(Not)_Alone_(2009)_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_8994bb0ae96c838f7aa298fbab1ba2a09229448e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Evangelion 1.11 You Are (Not) Alone")
        .add(Category::AudioTerm, "DTS-ES")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Evangelion_1.11_You_Are_(Not)_Alone_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv",
        )
        .add(Category::ReleaseGroup, "THORA")
        .add(Category::Source, "BluRay")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("Evangelion_1.11_You_Are_(Not)_Alone_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_d21a741053d2d2f5e61959455467a4c72a6f57e2() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Eve no Jikan")
        .add(Category::EpisodeNumber, "2")
        .add(Category::FileChecksum, "88F4F7F0")
        .add(Category::FileExtension, "mkv")
        .add(Category::FileName, "Eve no Jikan 2 [88F4F7F0].mkv");
    let parser_result = Parser::new("Eve no Jikan 2 [88F4F7F0].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_01ab88b4b5b636f1f7a70b761a290ebe5df47c44() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Gin'iro no Kami no Agito")
        .add(Category::AnimeYear, "2006")
        .add(Category::AudioTerm, "DTS")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Gin'iro_no_Kami_no_Agito_(2006)_[1080p,BluRay,x264,DTS]_-_THORA.mkv",
        )
        .add(Category::ReleaseGroup, "THORA")
        .add(Category::Source, "BluRay")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("Gin'iro_no_Kami_no_Agito_(2006)_[1080p,BluRay,x264,DTS]_-_THORA.mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_1d7c1727d1d029a092adabcaf88cc3a2ca5effe4() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Magical Girl Lyrical Nanoha A's")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "7A8A7769")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Magical Girl Lyrical Nanoha A's - 01.DVD[H264.AAC][DGz][7A8A7769].mkv",
        )
        .add(Category::ReleaseGroup, "DGz")
        .add(Category::Source, "DVD")
        .add(Category::VideoTerm, "H264");
    let parser_result =
        Parser::new("Magical Girl Lyrical Nanoha A's - 01.DVD[H264.AAC][DGz][7A8A7769].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_ec8455cb47f00c300b1de6530c75861486ac0580() {
    let wanted = Elements::new()
		.add(Category::AnimeSeason,"2")
		.add(Category::AnimeTitle,"Mobile Suit Gundam 00")
		.add(Category::EpisodeNumber,"07")
		.add(Category::EpisodeTitle,"A Reunion and a Parting")
		.add(Category::FileExtension,"mkv")
		.add(Category::FileName,"Mobile_Suit_Gundam_00_Season_2_Ep07_A_Reunion_and_a_Parting_[1080p,BluRay,x264]_-_THORA.mkv")
		.add(Category::ReleaseGroup,"THORA")
		.add(Category::Source,"BluRay")
		.add(Category::AnimeSeasonPrefix, "Season")
		.add(Category::VideoResolution,"1080p")
		.add(Category::VideoTerm,"x264")
		.add(Category::EpisodePrefix, "Ep")
	;
    let parser_result = Parser::new("Mobile_Suit_Gundam_00_Season_2_Ep07_A_Reunion_and_a_Parting_[1080p,BluRay,x264]_-_THORA.mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_2d32536c96b199939261e9ea80efd360ef9c900e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Noein")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Noein_[01_of_24]_[ru_jp]_[bodlerov_&_torrents_ru].mkv",
        )
        .add(Category::Language, "ru")
        .add(Category::Language, "jp")
        .add(Category::ReleaseGroup, "bodlerov_&_torrents_ru");
    let parser_result = Parser::new("Noein_[01_of_24]_[ru_jp]_[bodlerov_&_torrents_ru].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b7887137eabaffbaaf79a33ad3c039cfa2ebfbf7() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "ponyo on the cliff by the sea")
        .add(Category::AudioTerm, "dts")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "ponyo_on_the_cliff_by_the_sea[h264.dts][niizk].mkv",
        )
        .add(Category::ReleaseGroup, "niizk")
        .add(Category::VideoTerm, "h264");
    let parser_result = Parser::new("ponyo_on_the_cliff_by_the_sea[h264.dts][niizk].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_a5e04d9602cdb80ac6aa7577e1f04a37bc2fc523() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "AIKa ZERO OVA")
        .add(Category::AnimeType, "OVA")
        .add(Category::AudioTerm, "Flac")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "6730D40A")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A].mkv",
        )
        .add(Category::ReleaseGroup, "Seto_Otaku")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1920x1080")
        .add(Category::VideoTerm, "H264");
    let parser_result =
        Parser::new("[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_fc804ba86c18adea4d719a04f7408cd8493d5f4f() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "R.O.D the TV")
        .add(Category::AnimeType, "TV")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mkv")
        .add(Category::FileName, "[a4e]R.O.D_the_TV_01[divx5.2.1].mkv")
        .add(Category::VideoTerm, "divx5")
        .add(Category::ReleaseGroup, "a4e");
    let parser_result = Parser::new("[a4e]R.O.D_the_TV_01[divx5.2.1].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_f9fb85697cb82fb6350b3558a9ab44a2b772cece() {
    let wanted = Elements::new()
		.add(Category::AnimeTitle,"Ghost in the Shell Stand Alone Complex 2nd GIG")
		.add(Category::AudioTerm,"AAC")
		.add(Category::AudioTerm,"5.1")
		.add(Category::EpisodeNumber,"05")
		.add(Category::EpisodeTitle,"EXCAVATION")
		.add(Category::FileExtension,"mkv")
		.add(Category::EpisodePrefix,"Ep")
		.add(Category::FileName,"Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv")
		.add(Category::ReleaseGroup,"THORA")
		.add(Category::ReleaseVersion,"2")
		.add(Category::Source,"HDTV")
		.add(Category::VideoResolution,"720p")
		.add(Category::VideoTerm,"x264")
	;
    let parser_result = Parser::new("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_a5b250cb566804f842474b3382892aa091bd1d70() {
    let wanted = Elements::new()
		.add(Category::AnimeTitle,"Ghost in the Shell Stand Alone Complex 2nd GIG")
		.add(Category::AudioTerm,"AAC") 
		.add(Category::AudioTerm,"5.1") 
		.add(Category::EpisodeNumber,"06")
		.add(Category::EpisodeTitle,"Pu239")
		.add(Category::FileExtension,"mkv")
		.add(Category::FileName,"Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv")
		.add(Category::ReleaseGroup,"THORA")
		.add(Category::Source,"HDTV")
		.add(Category::VideoResolution,"720p")
		.add(Category::VideoTerm,"x264")
		.add(Category::EpisodePrefix, "Ep")
	;
    let parser_result = Parser::new("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_09f2bd48e05e8990e3a17b64458e2c3a8060f19f() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Fate Stay Night")
        .add(Category::EpisodePrefix, "Ep")
        .add(Category::EpisodeNumber, "05")
        .add(Category::EpisodeTitle, "The Two Magi Part1")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Fate_Stay_Night_Ep05_The_Two_Magi_Part1_[720p,BluRay,x264]_-_THORA.mkv",
        )
        .add(Category::ReleaseGroup, "THORA")
        .add(Category::Source, "BluRay")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("Fate_Stay_Night_Ep05_The_Two_Magi_Part1_[720p,BluRay,x264]_-_THORA.mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_1457cb4bd6716f7ef714d74c1e8fd099c85324a6() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Mezzo(DSA)")
        .add(Category::AudioTerm, "ogg")
        .add(Category::EpisodeNumber, "05")
        .add(Category::FileChecksum, "585d9971")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[RaX]Mezzo(DSA)_-_05_-_[x264_ogg]_[585d9971].mkv",
        )
        .add(Category::ReleaseGroup, "RaX")
        .add(Category::VideoTerm, "x264");
    let parser_result = Parser::new("[RaX]Mezzo(DSA)_-_05_-_[x264_ogg]_[585d9971].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_a602e00b321e6bd0298d2e048a5557f161a5dc8b() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Fullmetal Alchemist")
        .add(Category::AnimeYear, "2009")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "02")
        .add(Category::FileChecksum, "7B2C5E8B")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[AKH-SWE]_Fullmetal_Alchemist_(2009)_02v2_[H.264.AAC][7B2C5E8B].mkv",
        )
        .add(Category::ReleaseGroup, "AKH-SWE")
        .add(Category::ReleaseVersion, "2")
        .add(Category::VideoTerm, "H.264");
    let parser_result =
        Parser::new("[AKH-SWE]_Fullmetal_Alchemist_(2009)_02v2_[H.264.AAC][7B2C5E8B].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e7595b174ba425636fd1fdedd12e9a6af409f209() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Sayonara Zetsubou Sensei")
        .add(Category::AudioTerm, "AC3")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[FuktLogik][Sayonara_Zetsubou_Sensei][01][DVDRip][x264_AC3].mkv",
        )
        .add(Category::ReleaseGroup, "FuktLogik")
        .add(Category::Source, "DVDRip")
        .add(Category::VideoTerm, "x264");
    let parser_result =
        Parser::new("[FuktLogik][Sayonara_Zetsubou_Sensei][01][DVDRip][x264_AC3].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
/// ! False positive
/// ! Anime Title can not be "Kiddy Grade 2" so I'm must add Episode number category just to pass the test
// TODO Try to fixe that after fixe everything else
fn test_82711b7c5c58eeacefd19eae7b11e4c46fdd4ea1() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Kiddy Grade")
        .add(Category::AudioTerm, "AC3")
        .add(Category::EpisodeTitle, "Pilot")
        .add(Category::FileChecksum, "650B731B")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Ayu]_Kiddy_Grade_2_-_Pilot_[H264_AC3][650B731B].mkv",
        )
        .add(Category::EpisodeNumber, "2")
        .add(Category::ReleaseGroup, "Ayu")
        .add(Category::VideoTerm, "H264");
    let parser_result = Parser::new("[Ayu]_Kiddy_Grade_2_-_Pilot_[H264_AC3][650B731B].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_39548491a05d25c19c275fed437c9a8bd76d2c5e() {
    let wanted = Elements::new()
        .add(
            Category::AnimeTitle,
            "Tatakau Shisho - The Book of Bantorra",
        )
        .add(Category::AudioTerm, "MP3")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Darksoul-Subs] Tatakau Shisho - The Book of Bantorra [848x480 XVID_MP3].mkv",
        )
        .add(Category::ReleaseGroup, "Darksoul-Subs")
        .add(Category::VideoResolution, "848x480")
        .add(Category::VideoTerm, "XVID");
    let parser_result =
        Parser::new("[Darksoul-Subs] Tatakau Shisho - The Book of Bantorra [848x480 XVID_MP3].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_f8c2936ee3918dea84016b140627797c57aa8a88() {
    let wanted = Elements::new()
		.add(Category::AnimeTitle,"Neon Genesis Evangelion - Platinum")
		.add(Category::EpisodeNumber,"06")
		.add(Category::EpisodeTitle,"Showdown in Tokyo 3")
		.add(Category::FileChecksum,"CBDB8577")
		.add(Category::FileExtension,"mkv")
		.add(Category::FileName,"[ACX]Neon_Genesis_Evangelion_-_Platinum_-_06_-_Showdown_in_Tokyo_3_[SaintDeath]_[CBDB8577].mkv")
		.add(Category::ReleaseGroup,"ACX")
	;
    let parser_result = Parser::new("[ACX]Neon_Genesis_Evangelion_-_Platinum_-_06_-_Showdown_in_Tokyo_3_[SaintDeath]_[CBDB8577].mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_b3517e241415fc72103b83ac6fe11e9bc4757177() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Sora no Woto")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "E83AD672")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Himatsubushi]_Sora_no_Woto_-_01_-_H264_-_720p_-_E83AD672.mkv",
        )
        .add(Category::ReleaseGroup, "Himatsubushi")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "H264");
    let parser_result =
        Parser::new("[Himatsubushi]_Sora_no_Woto_-_01_-_H264_-_720p_-_E83AD672.mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e2fcb66e127b09279605cc6e9db81cce5aa21e62() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Nurse Witch Komugi chan Magikarte")
        .add(Category::EpisodeNumber, "02.5")
        .add(Category::FileChecksum, "902BB314")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[EroGaKi-Team]_Nurse_Witch_Komugi-chan_Magikarte_02.5_[902BB314].mkv",
        )
        .add(Category::ReleaseGroup, "EroGaKi-Team");
    let parser_result =
        Parser::new("[EroGaKi-Team]_Nurse_Witch_Komugi-chan_Magikarte_02.5_[902BB314].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_5e414360884314b781ba9a96ce5d8e0d71445379() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Ookiku Furikabutte")
        .add(Category::AnimeSeasonPrefix, "S")
        .add(Category::AnimeSeason, "2")
        .add(Category::EpisodeNumber, "09")
        .add(Category::FileChecksum, "BD841253")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Ookiku Furikabutte S2 - 09 (Central Anime) [BD841253].mkv",
        )
        .add(Category::ReleaseGroup, "Central Anime");
    let parser_result = Parser::new("Ookiku Furikabutte S2 - 09 (Central Anime) [BD841253].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_081697b971edf42d4f410af86cef3cd48eb5537a() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "HEROMAN")
        .add(Category::EpisodeNumber, "10")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[HorribleSubs] HEROMAN - 10_(XviD_AnimeSenshi).mkv",
        )
        .add(Category::ReleaseGroup, "HorribleSubs")
        .add(Category::VideoTerm, "XviD");
    let parser_result = Parser::new("[HorribleSubs] HEROMAN - 10_(XviD_AnimeSenshi).mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_67ca1214df71d9aa87f3de217038a41761472c41() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Detective Conan")
        .add(Category::EpisodeNumber, "316")
        .add(Category::EpisodeNumber, "317")
        .add(Category::FileChecksum, "2411959B")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Detective Conan - 316-317 [DCTP][2411959B].mkv",
        )
        .add(Category::ReleaseGroup, "DCTP");
    let parser_result = Parser::new("Detective Conan - 316-317 [DCTP][2411959B].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_69510b2230455e6689cfd4cf655e3d3ceec1b626() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Angel Beats")
        .add(Category::EpisodeNumber, "9")
        .add(Category::FileExtension, "mkv")
        .add(Category::FileName, "[N LogN Fansubs] Angel Beats (9).mkv")
        .add(Category::ReleaseGroup, "N LogN Fansubs");
    let parser_result = Parser::new("[N LogN Fansubs] Angel Beats (9).mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_3c64ceb13d5e14d17097334bb6f80c1dcef5e9a2() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "To Aru Kagaku no Railgun")
        .add(Category::EpisodeNumber, "13")
        .add(Category::EpisodeNumber, "15")
        .add(
            Category::FileName,
            "To_Aru_Kagaku_no_Railgun_13-15_[BD_1080p][AtsA]",
        )
        .add(Category::ReleaseGroup, "AtsA")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1080p");
    let parser_result = Parser::new("To_Aru_Kagaku_no_Railgun_13-15_[BD_1080p][AtsA]")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_377b8c576bd6d7c1f715a4c3be7ffa1aca2820db() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Juuousei")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "803DA487")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Juuousei_-_01_[Black_Sheep][HDTV_H264_AAC][803DA487].mkv",
        )
        .add(Category::ReleaseGroup, "Black_Sheep")
        .add(Category::Source, "HDTV")
        .add(Category::VideoTerm, "H264");
    let parser_result = Parser::new("Juuousei_-_01_[Black_Sheep][HDTV_H264_AAC][803DA487].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
