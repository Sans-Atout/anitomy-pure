use anitomy_rust::{elements::Category, Parser};

fn main() {
    let filenames = [
        "[Judas] Attack on Titan - S04E01-E04 [1080p].mkv",
        "[SubsPlease] One Piece - 1000-1001 [720p].mkv",
        "[HorribleSubs] Sword Art Online - 01 [1080p].mkv",
    ];

    for filename in &filenames {
        let Ok(result) = Parser::new(filename).parse() else {
            continue;
        };

        print!("{filename}");

        let title = result.find(Category::AnimeTitle).map(|e| e.value).unwrap_or_default();
        let season = result.find(Category::AnimeSeason).map(|e| e.value).unwrap_or_default();
        print!("\n  title  : {title}");
        if !season.is_empty() {
            print!("\n  season : {season}");
        }

        match result.find_all(Category::EpisodeNumber) {
            Some(episodes) if episodes.len() > 1 => {
                let nums: Vec<&str> = episodes.iter().map(|e| e.value.as_str()).collect();
                println!("\n  episodes: {} (range of {})", nums.join(", "), nums.len());
            }
            Some(episodes) => {
                println!("\n  episode : {}", episodes[0].value);
            }
            None => {
                println!("\n  episode : (none)");
            }
        }

        println!();
    }
}
