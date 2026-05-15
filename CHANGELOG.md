# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] — 2026-05-15

### Added

- `Parser` builder API — construct with `Parser::new(filename)`, configure with builder methods, then call `.parse()`
- Builder options: `ep_number`, `ep_title`, `file_extension`, `release_group` (all default `true`)
- Builder options: `allowed_delimiters` (default `[' ', '_', '.', '-', '&', '+', ',', '|']`) and `ignored_string` for pre-stripping substrings
- `Category` enum covering all extractable fields: `AnimeTitle`, `EpisodeNumber`, `EpisodeNumberAlt`, `EpisodePrefix`, `EpisodeTitle`, `AnimeSeason`, `AnimeSeasonPrefix`, `AnimeType`, `AnimeYear`, `AudioTerm`, `DeviceCompatibility`, `FileChecksum`, `FileExtension`, `FileName`, `Language`, `Other`, `ReleaseGroup`, `ReleaseInformation`, `ReleaseVersion`, `Source`, `Subtitles`, `VideoResolution`, `VideoTerm`, `VolumeNumber`, `VolumePrefix`
- `Element` struct with `category` and `value` fields
- `Elements` collection returned by `Parser::parse`, with:
  - `find(Category)` — returns `Option<Element>` for the first match
  - `find_all(Category)` — returns `Option<Vec<Element>>` for repeated categories (e.g. multi-episode ranges)
  - `count(Category)` — number of elements for a given category
  - `is_category_empty(Category)` — predicate for absence of a category
  - `contains(Category, &str)` — checks for a specific value in a category
  - `is_empty()` — true if no elements were extracted at all
  - `size()` — total element count across all categories
- Full support for Japanese episode counters (`第01話`), multi-episode ranges (`01-03`), and season+episode patterns (`S2E04`)
- Non-bracketed release group detection via fallback scan for `codec-GROUP` patterns (e.g. `x264-ESiR`)
- `EX` episode prefix keyword for filler/special episode markers
- `SAISON` season prefix keyword for French-language filenames
- Graceful handling of incomplete filenames (unclosed brackets): returns `FileName` and `FileExtension` only

### Changed

- Parsing engine uses hand-written O(n) byte scanners throughout — no regular expressions
- `parse()` returns `Result<Elements, ParsingError>` using `std::result::Result`
- Zero production dependencies

### Performance

- ~121 µs for 15 files on a representative benchmark
- Faster than the original C++ library the project draws inspiration from

[Unreleased]: https://github.com/Sans-Atout/anitomy-pure/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Sans-Atout/anitomy-pure/releases/tag/v0.1.0
