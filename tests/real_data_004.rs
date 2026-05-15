use anitomy_pure::Parser;
use anitomy_pure::elements::{Category, Elements};
//use pretty_assertions::assert_eq;
#[test]
fn test_a052317da36f9200dfa4f62608842fdd105537ff() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Tegami Bachi (REVERSE) - Letter Bee")
        .add(Category::EpisodeNumber, "04")
        .add(Category::EpisodeNumberAlt, "29")
        .add(Category::FileChecksum, "5203142B")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Deep] Tegami Bachi (REVERSE) - Letter Bee - 29 (04) [5203142B].mkv",
        )
        .add(Category::ReleaseGroup, "Deep");
    let parser_result =
        Parser::new("[Deep] Tegami Bachi (REVERSE) - Letter Bee - 29 (04) [5203142B].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_be514fb76873f1df44849b58e89027850c896e53() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Love Live! The School Idol Movie")
        .add(Category::AnimeType, "Movie")
        .add(Category::AnimeType, "PV")
        .add(Category::FileChecksum, "D1A15D2C")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[FFF] Love Live! The School Idol Movie - PV [D1A15D2C].mkv",
        )
        .add(Category::ReleaseGroup, "FFF");
    let parser_result = Parser::new("[FFF] Love Live! The School Idol Movie - PV [D1A15D2C].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_0297c2beb0509d55a17a4cc6cb3594783b8027b1() {
    let wanted = Elements::new()
        .add(
            Category::AnimeTitle,
            "Tamayura ~graduation photo~ Movie Part",
        )
        .add(Category::AnimeType, "Movie")
        .add(Category::EpisodeNumber, "1")
        .add(Category::FileChecksum, "98965607")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Nishi-Taku] Tamayura ~graduation photo~ Movie Part 1 [BD][720p][98965607].mkv",
        )
        .add(Category::ReleaseGroup, "Nishi-Taku")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "720p");
    let parser_result = Parser::new(
        "[Nishi-Taku] Tamayura ~graduation photo~ Movie Part 1 [BD][720p][98965607].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_8904673658f4b5a8257bb301f0b9bb94997517c4() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "1001 Nights")
        .add(Category::AnimeYear, "1998")
        .add(Category::FileName, "[LRL] 1001 Nights (1998) [DVD]")
        .add(Category::ReleaseGroup, "LRL")
        .add(Category::Source, "DVD");
    let parser_result = Parser::new("[LRL] 1001 Nights (1998) [DVD]")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_05911bf50c8a02a7554d31a67ed1ab929ca17ee3() {
    let wanted = Elements::new()
        .add(Category::EpisodeNumber, "0")
        .add(Category::FileExtension, "mkv")
        .add(Category::FileName, "[TardRaws] 0 [640x360].mkv")
        .add(Category::ReleaseGroup, "TardRaws")
        .add(Category::VideoResolution, "640x360");
    let parser_result = Parser::new("[TardRaws] 0 [640x360].mkv").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_a5b837698dfc604502763b7ff1772ad51438e38e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"Crayon Shin-Chan")
        .add(Category::EpisodeTitle, "The Secret of Buri Buri Kingdom")
        .add(Category::VolumePrefix,"Movie")
        .add(Category::VolumeNumber, "2")
        .add(Category::AnimeYear,"1994")
        .add(Category::AudioTerm,"AC3")
        .add(Category::FileExtension,"avi")
        .add(Category::FileName,"[FB] Crayon Shin-Chan Movie 2 The Secret of Buri Buri Kingdom [DivX5 AC3] 1994 [852X480] V2.avi")
        .add(Category::ReleaseGroup,"FB")
        .add(Category::ReleaseVersion,"2")
        .add(Category::VideoResolution,"852X480")
        .add(Category::VideoTerm,"DivX5")
    ;
    let parser_result = Parser::new("[FB] Crayon Shin-Chan Movie 2 The Secret of Buri Buri Kingdom [DivX5 AC3] 1994 [852X480] V2.avi").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_63caede1c1c5e4c049d150da86958bacb369b1cb() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Fairy Tail 2")
        .add(Category::EpisodeNumber, "52")
        .add(Category::EpisodeNumberAlt, "227")
        .add(Category::FileChecksum, "9DF6B8D5")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5].mkv",
        )
        .add(Category::ReleaseGroup, "Hatsuyuki-Kaitou")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "10bit");
    let parser_result =
        Parser::new("[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_571e7cd32866646b8625f7e545bbfc2ae45bac0b() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Baby Princess 3D Paradise Love")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "457CC066")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[FBI] Baby Princess 3D Paradise Love 01v0 [BD][720p-AAC][457CC066].mkv",
        )
        .add(Category::ReleaseGroup, "FBI")
        .add(Category::ReleaseVersion, "0")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "720p");
    let parser_result =
        Parser::new("[FBI] Baby Princess 3D Paradise Love 01v0 [BD][720p-AAC][457CC066].mkv")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_047035220958b93d081a7869cd879a03370f2712() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Macross Frontier")
        .add(Category::EpisodeNumber, "01b")
        .add(Category::FileChecksum, "4D5EC315")
        .add(Category::FileExtension, "avi")
        .add(
            Category::FileName,
            "[Shinsen-Subs]_Macross_Frontier_-_01b_[4D5EC315].avi",
        )
        .add(Category::ReleaseGroup, "Shinsen-Subs");
    let parser_result = Parser::new("[Shinsen-Subs]_Macross_Frontier_-_01b_[4D5EC315].avi")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_744de7408a5a51cd4881620b04407aace23637e3() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Hidamari Sketch x365")
        .add(Category::EpisodeNumber, "09a")
        .add(Category::FileChecksum, "49874745")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[NamaeNai] Hidamari Sketch x365 - 09a (DVD) [49874745].mkv",
        )
        .add(Category::ReleaseGroup, "NamaeNai")
        .add(Category::Source, "DVD");
    let parser_result = Parser::new("[NamaeNai] Hidamari Sketch x365 - 09a (DVD) [49874745].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_74192e1d2337888f4a0a13abcbe2d67432d5f425() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "D Gray man")
        .add(Category::EpisodeNumber, "04")
        .add(Category::FileExtension, "avi")
        .add(Category::FileName, "[KLF]_D.Gray-man_04V2.avi")
        .add(Category::ReleaseGroup, "KLF")
        .add(Category::ReleaseVersion, "2");
    let parser_result = Parser::new("[KLF]_D.Gray-man_04V2.avi").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_8da82c367bcc4de81e57e2b7262ecb15d84992f0() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Bakuman")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[FaggotryRaws] Bakuman - 01 (NHK E 848x480).mkv",
        )
        .add(Category::ReleaseGroup, "FaggotryRaws")
        .add(Category::VideoResolution, "848x480");
    let parser_result = Parser::new("[FaggotryRaws] Bakuman - 01 (NHK E 848x480).mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_7d1deb16f93a6a7cbad80ca41d765354941f2331() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "RWBY")
        .add(Category::EpisodeNumber, "14")
        .add(Category::EpisodeTitle, "Forever Fall Part 2")
        .add(Category::FileExtension, "mp4")
        .add(
            Category::FileName,
            "[5F] RWBY 14 Forever Fall Part 2 pt-BR.mp4",
        )
        .add(Category::Language, "pt-BR")
        .add(Category::ReleaseGroup, "5F");
    let parser_result = Parser::new("[5F] RWBY 14 Forever Fall Part 2 pt-BR.mp4")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_ef9de4db86ad82ea17af4e5007e4bbd13dc6e981() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Dragon Ball KAI")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "Dragon.Ball.KAI.-.01.-.1080p.BluRay.x264.DHD.mkv",
        )
        .add(Category::Source, "BluRay")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "x264");
    let parser_result = Parser::new("Dragon.Ball.KAI.-.01.-.1080p.BluRay.x264.DHD.mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_393505df506e9a7e1424e656f067ad2de44aed87() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Ushio to Tora (TV)")
        .add(Category::AnimeType, "TV")
        .add(Category::EpisodeNumber, "02")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[AnimeRG] Ushio to Tora (TV) - 02 [720p] [Xcelent].mkv",
        )
        .add(Category::ReleaseGroup, "AnimeRG")
        .add(Category::VideoResolution, "720p");
    let parser_result = Parser::new("[AnimeRG] Ushio to Tora (TV) - 02 [720p] [Xcelent].mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_194ed5b692a696b3066323650413f20aa10d2624() {
    let wanted = Elements::new().add(Category::FileName, "[Anime");
    let parser_result = Parser::new("[Anime").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_4b87f615a23bfc39596d0e9dd64f2daeabfd97b8() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Gekkan Shoujo Nozaki kun")
        .add(
            Category::FileName,
            "Gekkan Shoujo Nozaki-kun [HorribleSubs] (1080p)",
        )
        .add(Category::ReleaseGroup, "HorribleSubs")
        .add(Category::VideoResolution, "1080p");
    let parser_result = Parser::new("Gekkan Shoujo Nozaki-kun [HorribleSubs] (1080p)")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_53c90fce976a05dc06cf49e8336d2e85395bbb32() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Toradora!")
        .add(Category::AnimeType, "Opening")
        .add(Category::EpisodeNumber, "07")
        .add(Category::EpisodeTitle, "Pool")
        .add(Category::FileChecksum, "8F59F2BA")
        .add(
            Category::FileName,
            "[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]",
        )
        .add(Category::ReleaseGroup, "BM&T")
        .add(Category::ReleaseVersion, "2")
        .add(Category::Source, "BD")
        .add(Category::VideoTerm, "Hi10")
        .add(Category::VideoResolution, "720p");
    let parser_result =
        Parser::new("[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_80f887c7ec6eed193d3a00e43bf1e4f00b648f77() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle,"AKB0048")
        .add(Category::AudioTerm,"FLAC")
        .add(Category::EpisodeTitle,"Making of Kibou ni Tsuite Music Video")
        .add(Category::FileChecksum,"C09462E2")
        .add(Category::FileName,"[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]")
        .add(Category::ReleaseGroup,"EveTaku")
        .add(Category::Source,"BDRip")
        .add(Category::VideoTerm,"Hi10P")
        .add(Category::VideoTerm, "H.264")
        .add(Category::VideoResolution,"1080i")
        .add(Category::VolumeNumber,"03")
    ;
    let parser_result = Parser::new("[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]").parse().unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_2b468f3ed665f23351127bbc45bbe3b6bfc58578() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Magi - The Labyrinth Of Magic")
        .add(
            Category::FileName,
            "[DmonHiro] Magi - The Labyrinth Of Magic - Vol.1v2 (BD, 720p)",
        )
        .add(Category::ReleaseGroup, "DmonHiro")
        .add(Category::ReleaseVersion, "2")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "720p")
        .add(Category::VolumeNumber, "1");
    let parser_result =
        Parser::new("[DmonHiro] Magi - The Labyrinth Of Magic - Vol.1v2 (BD, 720p)")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_869fc8470c197aa75e4add6233e446ba74265c9e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Natsume Yuujinchou Shi")
        .add(Category::AudioTerm, "AAC")
        .add(
            Category::FileName,
            "[tlacatlc6] Natsume Yuujinchou Shi Vol. 1v2 & Vol. 2 (BD 1280x720 x264 AAC)",
        )
        .add(Category::ReleaseGroup, "tlacatlc6")
        .add(Category::ReleaseVersion, "2")
        .add(Category::Source, "BD")
        .add(Category::VideoTerm, "x264")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VolumeNumber, "1")
        .add(Category::VolumeNumber, "2");
    let parser_result =
        Parser::new("[tlacatlc6] Natsume Yuujinchou Shi Vol. 1v2 & Vol. 2 (BD 1280x720 x264 AAC)")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_752a48ce114bde101265c8aa514792042b46824f() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Hyouka")
        .add(Category::AudioTerm, "FLAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumber, "04")
        .add(
            Category::FileName,
            "[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]",
        )
        .add(Category::ReleaseGroup, "Tsundere")
        .add(Category::ReleaseVersion, "2")
        .add(Category::Source, "BDRip")
        .add(Category::VideoResolution, "1920x1080")
        .add(Category::VideoTerm, "10bit")
        .add(Category::VideoTerm, "h264");
    let parser_result =
        Parser::new("[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_7689f2662c28095b903fc45e774c52083ca7a3b2() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Nogizaka Haruka no Himitsu - Purezza")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumber, "03")
        .add(
            Category::FileName,
            "[Doki] Nogizaka Haruka no Himitsu - Purezza - 01v2-03v2 (1280x720 h264 AAC)",
        )
        .add(Category::ReleaseGroup, "Doki")
        .add(Category::ReleaseVersion, "2")
        .add(Category::ReleaseVersion, "2")
        .add(Category::VideoResolution, "1280x720")
        .add(Category::VideoTerm, "h264");
    let parser_result =
        Parser::new("[Doki] Nogizaka Haruka no Himitsu - Purezza - 01v2-03v2 (1280x720 h264 AAC)")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_c0d73d22f0e3a4ef23b7801a46a11c1afb1fbc3d() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "06")
        .add(Category::AnimeTitle, "Fairy Tail")
        .add(Category::EpisodeNumber, "32")
        .add(Category::EpisodeNumberAlt, "83")
        .add(
            Category::EpisodeTitle,
            "Tartaros Arc Iron Fist of the Fire Dragon",
        )
        .add(
            Category::FileName,
            "Fairy Tail - S06E32 - Tartaros Arc Iron Fist of the Fire Dragon [Episode 83]",
        );
    let parser_result =
        Parser::new("Fairy Tail - S06E32 - Tartaros Arc Iron Fist of the Fire Dragon [Episode 83]")
            .parse()
            .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_01fe2ec7cbede675a642ddf913b113e528b34760() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "02")
        .add(Category::AnimeTitle, "Noragami")
        .add(Category::EpisodeNumber, "6")
        .add(Category::EpisodeTitle, "What Must Be Done")
        .add(
            Category::FileName,
            "Noragami - S02E06 - What Must Be Done [Episode 6]",
        );
    let parser_result = Parser::new("Noragami - S02E06 - What Must Be Done [Episode 6]")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_13da0a391501fde8df41d3e380f70576c12f552e() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Classroom Crisis")
        .add(Category::AudioTerm, "AAC")
        .add(
            Category::FileName,
            "[Harunatsu] Classroom Crisis - Vol.1 [BD 720p-AAC]",
        )
        .add(Category::ReleaseGroup, "Harunatsu")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "720p")
        .add(Category::VolumeNumber, "1");
    let parser_result = Parser::new("[Harunatsu] Classroom Crisis - Vol.1 [BD 720p-AAC]")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_4a4d0d71e2e6768e875df4401692534d8eb30987() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Classroom Crisis")
        .add(Category::AudioTerm, "FLAC")
        .add(Category::EpisodeNumber, "1")
        .add(Category::EpisodeNumber, "2")
        .add(
            Category::FileName,
            "[GS] Classroom Crisis Vol.1&2 (BD 1080p 10bit FLAC)",
        )
        .add(Category::ReleaseGroup, "GS")
        .add(Category::Source, "BD")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "10bit")
        .add(Category::VolumeNumber, "1");
    let parser_result = Parser::new("[GS] Classroom Crisis Vol.1&2 (BD 1080p 10bit FLAC)")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_2389d02ca2bfc0c3719dced24ddfd749b9d5ef4d() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Norn9 - Norn + Nonetto")
        .add(Category::EpisodeNumber, "12")
        .add(
            Category::FileName,
            "[Infantjedi] Norn9 - Norn + Nonetto - 12",
        )
        .add(Category::ReleaseGroup, "Infantjedi");
    let parser_result = Parser::new("[Infantjedi] Norn9 - Norn + Nonetto - 12")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_7c352d5386fc30a7b7f52fac535b7ff94f72243d() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Dragon Ball Z Movies 8")
        .add(Category::AudioTerm, "DTS")
        .add(Category::EpisodeNumber, "10")
        .add(
            Category::FileName,
            "Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA",
        )
        .add(Category::ReleaseGroup, "THORA")
        .add(Category::Source, "BluRay")
        .add(Category::VideoResolution, "720p")
        .add(Category::VideoTerm, "x264");
    let parser_result = Parser::new("Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_2bb89edeb1391bb59026cbdc9dbb6ee68790c8d4() {
    let wanted = Elements::new()
        .add(Category::AnimeTitle, "Momokuri")
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumber, "02")
        .add(Category::FileName, "[HorribleSubs] Momokuri - 01+02 [720p]")
        .add(Category::ReleaseGroup, "HorribleSubs")
        .add(Category::VideoResolution, "720p");
    let parser_result = Parser::new("[HorribleSubs] Momokuri - 01+02 [720p]")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_e41fd52dc12187726654600a632d0af08b4aac4d() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "23")
        .add(Category::AnimeTitle, "Nintama Rantarou")
        .add(Category::EpisodeNumber, "1821")
        .add(Category::EpisodeTitle, "Buddhist Priest sama is a Ninja")
        .add(
            Category::FileName,
            "[(´• ω •`)] Nintama Rantarou - S23E1821 - Buddhist Priest-sama is a Ninja.mkv",
        )
        .add(Category::ReleaseGroup, "´• ω •`")
        .add(Category::FileExtension, "mkv");
    let parser_result = Parser::new(
        "[(´• ω •`)] Nintama Rantarou - S23E1821 - Buddhist Priest-sama is a Ninja.mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_05c2eb9192081502c995894e59ec261977bd8ac9() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "01")
        .add(Category::AnimeTitle, "Aharen san wa Hakarenai")
        .add(Category::EpisodeNumber, "06")
        .add(Category::FileExtension, "mkv")
        .add(
            Category::FileName,
            "[Judas] Aharen-san wa Hakarenai - S01E06v2.mkv",
        )
        .add(Category::ReleaseGroup, "Judas")
        .add(Category::ReleaseVersion, "2");
    let parser_result = Parser::new("[Judas] Aharen-san wa Hakarenai - S01E06v2.mkv")
        .parse()
        .unwrap();
    assert_eq!(wanted, parser_result);
}
#[test]
fn test_070392de4fbbc26af0c45ac611bb6b5cdb940a81() {
    let wanted = Elements::new()
        .add(Category::AnimeSeason, "01")
        .add(Category::AnimeTitle, "Somali and the Forest Spirit")
        .add(Category::AudioTerm, "AAC")
        .add(Category::EpisodeNumber, "01")
        .add(Category::FileChecksum, "BB7C6531")
        .add(
            Category::FileName,
            "[0x539] Somali and the Forest Spirit - S01E01 (WEB 1080p Hi10P AAC) [BB7C6531].mkv",
        )
        .add(Category::ReleaseGroup, "0x539")
        .add(Category::VideoResolution, "1080p")
        .add(Category::VideoTerm, "Hi10P")
        .add(Category::FileExtension, "mkv");
    let parser_result = Parser::new(
        "[0x539] Somali and the Forest Spirit - S01E01 (WEB 1080p Hi10P AAC) [BB7C6531].mkv",
    )
    .parse()
    .unwrap();
    assert_eq!(wanted, parser_result);
}
