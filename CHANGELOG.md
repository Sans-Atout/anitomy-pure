# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Anime title extraction for compound alpha+digit tokens: `EP07.5` no longer includes "EP" in the title, `SP01` no longer includes "SP", and `S01E01` stops the title at "S" correctly
- `OVA3.5`, `EX01` and similar searchable/invalid-prefix tokens are correctly included in the title when the episode number has already been found
- Non-bracketed release groups in `codec-GROUP` patterns (e.g. `x264-ESiR`) are now detected via a fallback scan when no bracketed group is present
- Added `EX` as an `EpisodePrefix` keyword (valid=false) to handle filler/special episode markers
- Added `SAISON` as an `AnimeSeasonPrefix` keyword for French-language filenames

### Added

- `Elements::find_all()` returns `Option<Vec<Element>>` for multi-episode ranges and other repeated categories
- `Elements::is_category_empty()` convenience predicate
- `Elements::count()` returns how many elements exist for a given category
- Full support for Japanese episode counters (`第01話`), multi-episode ranges (`01-03`), and season patterns (`S2E04`)

### Changed

- **Breaking**: `Elements::find()` now returns `Option<Element>` instead of `Result<Element, CategoryNotFound>`
- **Breaking**: `Elements::find_all()` now returns `Option<Vec<Element>>` instead of `Result<Vec<Element>, CategoryNotFound>`
- Complete rewrite of the parsing engine — all parsing now uses hand-written O(n) byte scanners instead of regular expressions
- Zero production dependencies: `regex`, `unicode-normalization`, and `error-stack` have been removed
- `parse()` returns `Result<Elements, ParsingError>` using `std::result::Result` (no longer `error_stack::Result`)
- Hot paths no longer allocate: title building uses `push_str`, extension lookup uses a static `LazyLock<HashSet>`

### Removed

- `CategoryNotFound` error type — callers should match on `None` instead
- `normalize()` utility function (unused)
- All production dependencies (`regex`, `unicode-normalization`, `error-stack`)

### Performance

- ~18.7× faster than the previous implementation (2.27 ms → ~121 µs for 50 files)
- Now faster than the original C++ library it is based on (~147 µs)

## [0.0.4] — 2024-01-01

### Added

- Initial public release
- Core parsing for episode number, anime title, release group, video/audio terms, file extension, checksum, language, subtitles, season, volume, and more
- Builder API (`Parser::new().ep_number(true).parse()`)
- `allowed_delimiters` and `ignored_string` builder options

[Unreleased]: https://github.com/Sans-Atout/anitomy-pure/compare/v0.0.4...HEAD
[0.0.4]: https://github.com/Sans-Atout/anitomy-pure/releases/tag/v0.0.4
