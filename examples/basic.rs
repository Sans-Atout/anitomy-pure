use anitomy_pure::{Parser, elements::Category};

fn main() {
    let filename = "[HorribleSubs] Boku no Hero Academia - 73 [1080p].mkv";

    let result = Parser::new(filename).parse().expect("parse failed");

    println!("File      : {filename}");
    println!("---");

    if let Some(e) = result.find(Category::ReleaseGroup) {
        println!("Group     : {}", e.value);
    }
    if let Some(e) = result.find(Category::AnimeTitle) {
        println!("Title     : {}", e.value);
    }
    if let Some(e) = result.find(Category::EpisodeNumber) {
        println!("Episode   : {}", e.value);
    }
    if let Some(e) = result.find(Category::VideoResolution) {
        println!("Resolution: {}", e.value);
    }
    if let Some(e) = result.find(Category::FileExtension) {
        println!("Extension : {}", e.value);
    }
}
