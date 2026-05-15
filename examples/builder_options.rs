use anitomy_rust::{elements::Category, Parser};

fn main() {
    let filename = "[Commie] Steins;Gate - 01 [BD 1080p AAC] [A1B2C3D4].mkv";

    // Default: parse everything.
    let full = Parser::new(filename).parse().unwrap();
    println!("=== Full parse ===");
    print_summary(&full);

    // Title + episode number only — skip release group, episode title.
    let minimal = Parser::new(filename)
        .ep_title(false)
        .release_group(false)
        .parse()
        .unwrap();
    println!("\n=== Minimal (title + episode only) ===");
    print_summary(&minimal);

    // Ignore a known sub-group tag before parsing.
    let filename2 = "[Commie][BD] Steins;Gate - 01 [1080p].mkv";
    let ignored = Parser::new(filename2)
        .ignored_string(vec!["[BD]"])
        .parse()
        .unwrap();
    println!("\n=== With ignored string [BD] ===");
    print_summary(&ignored);
}

fn print_summary(result: &anitomy_rust::elements::Elements) {
    for cat in [
        Category::ReleaseGroup,
        Category::AnimeTitle,
        Category::EpisodeNumber,
        Category::AudioTerm,
        Category::VideoResolution,
        Category::FileChecksum,
        Category::FileExtension,
    ] {
        if let Some(e) = result.find(cat) {
            println!("  {:<22} {}", format!("{cat:?}"), e.value);
        }
    }
}
