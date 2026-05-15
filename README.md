# anitomy-pure

A fast, zero-dependency Rust library for parsing anime video filenames.

[![CI](https://github.com/Sans-Atout/anitomy-pure/actions/workflows/build.yml/badge.svg)](https://github.com/Sans-Atout/anitomy-pure/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/anitomy-pure.svg)](https://crates.io/crates/anitomy-pure)
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-blue.svg)](LICENSE.md)

## What it does

Parses filenames like:

```
[HorribleSubs] Boku no Hero Academia - 73 [1080p].mkv
```

Into structured data:

| Category | Value |
|----------|-------|
| ReleaseGroup | HorribleSubs |
| AnimeTitle | Boku no Hero Academia |
| EpisodeNumber | 73 |
| VideoResolution | 1080p |
| FileExtension | mkv |

It handles the wide variety of naming conventions used in fansub releases: brackets, underscores, dots, version suffixes (`v2`), multi-episode ranges (`01-03`), season patterns (`S2E04`), Japanese counters (`第01話`), and more.

## Why anitomy-pure?

- **Faster than the C++ original.** The library it is based on ([anitomy](https://github.com/erengy/anitomy)) goes through a C FFI bridge; anitomy-pure beats it in pure throughput without leaving Rust.
- **Zero production dependencies.** No `regex`, no `unicode-normalization`, no utility crates — only `std`. Compile times stay short and the dependency tree stays auditable.
- **No heap allocations in hot paths.** Parsing reuses stack buffers and `&str` slices; `String` allocation only happens when a result is returned.
- **Compiler-enforced safety.** `#![deny(unsafe_code)]` is set at the crate root — no `unsafe` block can be introduced without a build failure. `#![deny(missing_docs)]` ensures every public item stays documented.

## Performance

Benchmarked on rustc 1.95.0 across 15 real-world filenames (Criterion, 100 samples):

| Parser | Version | Time (15 files) | Per file |
|--------|---------|-----------------|----------|
| **anitomy-pure** | **0.0.4** | **~113 µs** | **~7.6 µs** |
| anitomy (C++ via FFI) | 0.2.0 | ~130 µs | ~8.7 µs |
| zantetsu | 0.2.0 | ~1.08 ms | ~71.8 µs |
| hunch | 2.0.2 | ~2.48 ms | ~165 µs |

Numbers will vary by machine. To reproduce:

```sh
cargo bench
```

Results land in `target/criterion/`.

## Installation

```toml
[dependencies]
anitomy-pure = "0.0.4"
```

## Usage

```rust
use anitomy_rust::{Parser, elements::Category};

fn main() {
    let result = Parser::new("[HorribleSubs] Boku no Hero Academia - 73 [1080p].mkv")
        .parse()
        .unwrap();

    println!("{}", result.find(Category::AnimeTitle).unwrap().value);    // "Boku no Hero Academia"
    println!("{}", result.find(Category::EpisodeNumber).unwrap().value); // "73"
    println!("{}", result.find(Category::VideoResolution).unwrap().value); // "1080p"
}
```

### Builder options

```rust
Parser::new("filename.mkv")
    .ep_number(true)           // parse episode numbers (default: true)
    .ep_title(true)            // parse episode titles (default: true)
    .file_extension(true)      // parse file extension (default: true)
    .release_group(true)       // parse release group (default: true)
    .allowed_delimiters(vec![' ', '_', '.', '-'])
    .ignored_string(vec!["[SubGroup]"])
    .parse()
```

### Working with results

```rust
let result = Parser::new("...").parse()?;

// Find the first element of a category
if let Some(elem) = result.find(Category::EpisodeNumber) {
    println!("Episode: {}", elem.value);
}

// Find all elements of a category (e.g. multi-episode ranges)
if let Some(episodes) = result.find_all(Category::EpisodeNumber) {
    for ep in episodes {
        println!("Episode: {}", ep.value);
    }
}

// Check presence
if !result.is_category_empty(Category::AnimeSeason) {
    // ...
}
```

### Error handling

`parse()` returns `Result<Elements, ParsingError>`. The only failure case is an empty filename string.

```rust
match Parser::new("").parse() {
    Ok(_) => unreachable!(),
    Err(e) => eprintln!("Parse error: {e}"),
}
```

## Categories

| Category | Description |
|----------|-------------|
| `AnimeTitle` | Main title of the series |
| `EpisodeNumber` | Episode number(s) |
| `EpisodeNumberAlt` | Alternative episode number (e.g. absolute vs. season) |
| `EpisodeTitle` | Title of the specific episode |
| `EpisodePrefix` | Keyword before episode number (`EP`, `Episode`, etc.) |
| `AnimeSeason` | Season number |
| `AnimeSeasonPrefix` | Keyword before season number (`S`, `Season`, etc.) |
| `AnimeType` | Type of release (`OVA`, `ONA`, `Movie`, etc.) |
| `AnimeYear` | Year of the series |
| `AudioTerm` | Audio information (`FLAC`, `AAC`, `5.1`, etc.) |
| `VideoTerm` | Video information (`H.264`, `10bit`, etc.) |
| `VideoResolution` | Resolution (`1080p`, `720p`, `1920x1080`, etc.) |
| `Source` | Source media (`Blu-ray`, `WEB-DL`, etc.) |
| `ReleaseGroup` | Fansub/release group name |
| `ReleaseVersion` | Release version (`v2`, `v3`) |
| `ReleaseInformation` | Additional release info (`REPACK`, `PROPER`, etc.) |
| `FileExtension` | File extension (`mkv`, `mp4`, etc.) |
| `FileChecksum` | CRC32 checksum (`[1234ABCD]`) |
| `Language` | Audio/subtitle language |
| `Subtitles` | Subtitle type (`VOSTFR`, `Sub`, etc.) |
| `VolumeNumber` | Volume number (for manga-based content) |
| `VolumePrefix` | Volume keyword (`Vol.`, `Volume`) |
| `DeviceCompatibility` | Device tags (`PS3`, `Xbox`) |

## Examples

Runnable examples live in [`examples/`](examples/):

| Example | What it shows |
|---------|--------------|
| `basic` | Parse a single filename and print each field |
| `batch` | Parse a list of filenames into a summary table |
| `multi_episode` | Multi-episode ranges with `find_all` |
| `builder_options` | Selective parsing and `ignored_string` |

```sh
cargo run --example basic
cargo run --example batch
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Mozilla Public License 2.0](LICENSE.md)
