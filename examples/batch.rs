use anitomy_pure::{Parser, elements::Category};

fn main() {
    let filenames = [
        "[SubsPlease] Dungeon Meshi - 01 [1080p].mkv",
        "[Erai-raws] Frieren - Beyond Journey's End - 28 [720p].mkv",
        "Violet.Evergarden.S01E01.1080p.BluRay.FLAC.x264-YURASUKA.mkv",
        "[HorribleSubs] Toradora! - 07v2 [480p].mkv",
        "Fullmetal.Alchemist.Brotherhood.OVA.1080p.BluRay.x264.mkv",
    ];

    println!(
        "{:<50} {:<30} {:<8} {:<6}",
        "Title", "Group", "Episode", "Res"
    );
    println!("{}", "-".repeat(100));

    for filename in &filenames {
        let Ok(result) = Parser::new(filename).parse() else {
            println!("  (parse error: {filename})");
            continue;
        };

        let title = result
            .find(Category::AnimeTitle)
            .map(|e| e.value)
            .unwrap_or_default();
        let group = result
            .find(Category::ReleaseGroup)
            .map(|e| e.value)
            .unwrap_or_default();
        let episode = result
            .find(Category::EpisodeNumber)
            .map(|e| e.value)
            .unwrap_or_default();
        let res = result
            .find(Category::VideoResolution)
            .map(|e| e.value)
            .unwrap_or_default();

        println!("{title:<50} {group:<30} {episode:<8} {res:<6}");
    }
}
