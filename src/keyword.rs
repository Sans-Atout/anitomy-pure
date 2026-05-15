//! Keyword database: maps known token strings to their parsing metadata.

use std::collections::HashMap;

use crate::elements::Category;

/// Metadata associated with a recognised keyword string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Keyword {
    category: Category,
    identifiable: bool,
    searchable: bool,
    valid: bool,
}

impl Keyword {
    /// Creates a keyword for the given category with all flags set to `true`.
    pub(crate) fn new(c: Category) -> Keyword {
        Keyword {
            category: c,
            identifiable: true,
            searchable: true,
            valid: true,
        }
    }

    /// Sets whether this keyword can be identified as a standalone token.
    pub(crate) fn identifiable(&mut self, b: bool) -> Keyword {
        self.identifiable = b;
        self.to_owned()
    }

    /// Sets whether this keyword is looked up during the keyword search pass.
    pub(crate) fn searchable(&mut self, b: bool) -> Keyword {
        self.searchable = b;
        self.to_owned()
    }

    /// Sets whether a match against this keyword produces a usable element value.
    pub(crate) fn valid(&mut self, b: bool) -> Keyword {
        self.valid = b;
        self.to_owned()
    }

    /// Returns `true` if this keyword can be identified as a standalone token.
    pub(crate) fn is_identifiable(&self) -> bool {
        self.identifiable
    }

    /// Returns `true` if this keyword is looked up during the keyword search pass.
    pub(crate) fn is_searchable(&self) -> bool {
        self.searchable
    }

    /// Returns `true` if a match against this keyword produces a usable element value.
    pub(crate) fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns the category this keyword belongs to.
    pub(crate) fn get_category(&self) -> Category {
        self.category
    }
}

/// Lookup table mapping uppercase keyword strings to their [`Keyword`] metadata.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct Manager {
    /// The underlying map. Keyed by the uppercase ASCII form of the keyword.
    pub(crate) keywords: HashMap<String, Keyword>,
}

impl Manager {
    /// Creates a manager pre-populated with the full anime filename keyword vocabulary.
    pub(crate) fn new() -> Manager {
        Manager::default()
            .add(
                "S",
                Keyword::new(Category::AnimeSeasonPrefix).identifiable(false),
            )
            .add(
                "SAISON",
                Keyword::new(Category::AnimeSeasonPrefix).identifiable(false),
            )
            .add(
                "SEASON",
                Keyword::new(Category::AnimeSeasonPrefix).identifiable(false),
            )
            .add(
                "GEKIJOUBAN",
                Keyword::new(Category::AnimeType).identifiable(false),
            )
            .add(
                "MOVIE",
                Keyword::new(Category::VolumePrefix).identifiable(false),
            )
            .add("OAD", Keyword::new(Category::AnimeType).identifiable(false))
            .add("OAV", Keyword::new(Category::AnimeType).identifiable(false))
            .add("ONA", Keyword::new(Category::AnimeType).identifiable(false))
            .add("OVA", Keyword::new(Category::AnimeType).identifiable(false))
            .add(
                "SPECIAL",
                Keyword::new(Category::AnimeType).identifiable(false),
            )
            .add(
                "SPECIALS",
                Keyword::new(Category::AnimeType).identifiable(false),
            )
            .add("TV", Keyword::new(Category::AnimeType).identifiable(false))
            .add(
                "SP",
                Keyword::new(Category::AnimeType)
                    .identifiable(false)
                    .searchable(false),
            )
            .add(
                "ED",
                Keyword::new(Category::AnimeType)
                    .valid(false)
                    .searchable(false),
            )
            .add(
                "ENDING",
                Keyword::new(Category::AnimeType)
                    .valid(false)
                    .searchable(false),
            )
            .add(
                "NCED",
                Keyword::new(Category::AnimeType)
                    .valid(false)
                    .searchable(false),
            )
            .add(
                "NCOP",
                Keyword::new(Category::AnimeType)
                    .valid(false)
                    .searchable(false),
            )
            .add(
                "OP",
                Keyword::new(Category::AnimeType)
                    .valid(false)
                    .searchable(false),
            )
            .add(
                "OPENING",
                Keyword::new(Category::AnimeType)
                    .valid(false)
                    .searchable(false),
            )
            .add(
                "PREVIEW",
                Keyword::new(Category::AnimeType)
                    .valid(false)
                    .searchable(false),
            )
            .add(
                "PV",
                Keyword::new(Category::AnimeType)
                    .valid(false)
                    .searchable(false),
            )
            .add("DD2.0", Keyword::new(Category::AudioTerm))
            .add("DD5.1", Keyword::new(Category::AudioTerm))
            .add("2.0CH", Keyword::new(Category::AudioTerm))
            .add("2CH", Keyword::new(Category::AudioTerm))
            .add("5.1CH", Keyword::new(Category::AudioTerm))
            .add("HE-AAC", Keyword::new(Category::AudioTerm))
            .add("DTS", Keyword::new(Category::AudioTerm))
            .add("DTS-ES", Keyword::new(Category::AudioTerm))
            .add("DTS5.1", Keyword::new(Category::AudioTerm))
            .add("TRUEHD5.1", Keyword::new(Category::AudioTerm))
            .add("AAC", Keyword::new(Category::AudioTerm))
            .add("AACX2", Keyword::new(Category::AudioTerm))
            .add("AACX3", Keyword::new(Category::AudioTerm))
            .add("AACX4", Keyword::new(Category::AudioTerm))
            .add("AC3", Keyword::new(Category::AudioTerm))
            .add("EAC3", Keyword::new(Category::AudioTerm))
            .add("E-AC-3", Keyword::new(Category::AudioTerm))
            .add("FLAC", Keyword::new(Category::AudioTerm))
            .add("FLACX2", Keyword::new(Category::AudioTerm))
            .add("FLACX3", Keyword::new(Category::AudioTerm))
            .add("FLACX4", Keyword::new(Category::AudioTerm))
            .add("LOSSLESS", Keyword::new(Category::AudioTerm))
            .add("MP3", Keyword::new(Category::AudioTerm))
            .add("OGG", Keyword::new(Category::AudioTerm))
            .add("VORBIS", Keyword::new(Category::AudioTerm))
            .add("DUALAUDIO", Keyword::new(Category::AudioTerm))
            .add("DUAL AUDIO", Keyword::new(Category::AudioTerm))
            .add("DUAL-AUDIO", Keyword::new(Category::AudioTerm))
            .add("MULTIAUDIO", Keyword::new(Category::AudioTerm))
            .add("MULTI AUDIO", Keyword::new(Category::AudioTerm))
            .add("MULTI-AUDIO", Keyword::new(Category::AudioTerm))
            .add("IPAD3", Keyword::new(Category::DeviceCompatibility))
            .add("IPHONE5", Keyword::new(Category::DeviceCompatibility))
            .add("IPOD", Keyword::new(Category::DeviceCompatibility))
            .add("PS3", Keyword::new(Category::DeviceCompatibility))
            .add("XBOX", Keyword::new(Category::DeviceCompatibility))
            .add("XBOX360", Keyword::new(Category::DeviceCompatibility))
            .add(
                "ANDROID",
                Keyword::new(Category::DeviceCompatibility).identifiable(false),
            )
            .add("EP", Keyword::new(Category::EpisodePrefix))
            .add("EP.", Keyword::new(Category::EpisodePrefix))
            .add("EPS", Keyword::new(Category::EpisodePrefix))
            .add("EPS.", Keyword::new(Category::EpisodePrefix))
            .add("EPISODE", Keyword::new(Category::EpisodePrefix))
            .add("EPISODE.", Keyword::new(Category::EpisodePrefix))
            .add("EPISODES", Keyword::new(Category::EpisodePrefix))
            .add("CAPITULO", Keyword::new(Category::EpisodePrefix))
            .add("EPISODIO", Keyword::new(Category::EpisodePrefix))
            .add("FOLGE", Keyword::new(Category::EpisodePrefix))
            .add("E", Keyword::new(Category::EpisodePrefix).valid(false))
            .add("EX", Keyword::new(Category::EpisodePrefix).valid(false))
            .add("\x7B2C", Keyword::new(Category::EpisodePrefix).valid(false))
            .add("ENG", Keyword::new(Category::Language))
            .add("ENGLISH", Keyword::new(Category::Language))
            .add("ESPANOL", Keyword::new(Category::Language))
            .add("JAP", Keyword::new(Category::Language))
            .add("JPN", Keyword::new(Category::Language))
            .add("PT-BR", Keyword::new(Category::Language))
            .add("SPANISH", Keyword::new(Category::Language))
            .add("VOSTFR", Keyword::new(Category::Language))
            .add("ESP", Keyword::new(Category::Language).identifiable(false))
            .add("ITA", Keyword::new(Category::Language).identifiable(false))
            .add("FR", Keyword::new(Category::Language).identifiable(false))
            .add("JP", Keyword::new(Category::Language).identifiable(false))
            .add("RU", Keyword::new(Category::Language).identifiable(false))
            .add("REMASTER", Keyword::new(Category::Other))
            .add("REMASTERED", Keyword::new(Category::Other))
            .add("UNCENSORED", Keyword::new(Category::Other))
            .add("UNCUT", Keyword::new(Category::Other))
            .add("TS", Keyword::new(Category::Other))
            .add("VFR", Keyword::new(Category::Other))
            .add("WIDESCREEN", Keyword::new(Category::Other))
            .add("WS", Keyword::new(Category::Other))
            .add("THORA", Keyword::new(Category::ReleaseGroup))
            .add("BATCH", Keyword::new(Category::ReleaseInformation))
            .add("COMPLETE", Keyword::new(Category::ReleaseInformation))
            .add("VERSION", Keyword::new(Category::ReleaseInformation).identifiable(false))
            .add("PATCH", Keyword::new(Category::ReleaseInformation))
            .add("REMUX", Keyword::new(Category::ReleaseInformation))
            .add(
                "END",
                Keyword::new(Category::ReleaseInformation).identifiable(false),
            )
            .add(
                "FINAL",
                Keyword::new(Category::ReleaseInformation).identifiable(false),
            )
            .add("V0", Keyword::new(Category::ReleaseVersion))
            .add("V1", Keyword::new(Category::ReleaseVersion))
            .add("V2", Keyword::new(Category::ReleaseVersion))
            .add("V3", Keyword::new(Category::ReleaseVersion))
            .add("V4", Keyword::new(Category::ReleaseVersion))
            .add("23.976FPS", Keyword::new(Category::VideoTerm))
            .add("24FPS", Keyword::new(Category::VideoTerm))
            .add("29.97FPS", Keyword::new(Category::VideoTerm))
            .add("30FPS", Keyword::new(Category::VideoTerm))
            .add("60FPS", Keyword::new(Category::VideoTerm))
            .add("120FPS", Keyword::new(Category::VideoTerm))
            .add("8BIT", Keyword::new(Category::VideoTerm))
            .add("8-BIT", Keyword::new(Category::VideoTerm))
            .add("10BIT", Keyword::new(Category::VideoTerm))
            .add("10BITS", Keyword::new(Category::VideoTerm))
            .add("10-BIT", Keyword::new(Category::VideoTerm))
            .add("10-BITS", Keyword::new(Category::VideoTerm))
            .add("HI10", Keyword::new(Category::VideoTerm))
            .add("HI10P", Keyword::new(Category::VideoTerm))
            .add("HI444", Keyword::new(Category::VideoTerm))
            .add("HI444P", Keyword::new(Category::VideoTerm))
            .add("HI444PP", Keyword::new(Category::VideoTerm))
            .add("H264", Keyword::new(Category::VideoTerm))
            .add("H265", Keyword::new(Category::VideoTerm))
            .add("H.264", Keyword::new(Category::VideoTerm))
            .add("H.265", Keyword::new(Category::VideoTerm))
            .add("X264", Keyword::new(Category::VideoTerm))
            .add("X265", Keyword::new(Category::VideoTerm))
            .add("X.264", Keyword::new(Category::VideoTerm))
            .add("AVC", Keyword::new(Category::VideoTerm))
            .add("HEVC", Keyword::new(Category::VideoTerm))
            .add("HEVC2", Keyword::new(Category::VideoTerm))
            .add("DIVX", Keyword::new(Category::VideoTerm))
            .add("DIVX5", Keyword::new(Category::VideoTerm))
            .add("DIVX6", Keyword::new(Category::VideoTerm))
            .add("XVID", Keyword::new(Category::VideoTerm))
            .add("AVI", Keyword::new(Category::VideoTerm))
            .add("MP4", Keyword::new(Category::VideoTerm))
            .add("RMVB", Keyword::new(Category::VideoTerm))
            .add("WMV", Keyword::new(Category::VideoTerm))
            .add("WMV3", Keyword::new(Category::VideoTerm))
            .add("WMV9", Keyword::new(Category::VideoTerm))
            .add("HQ", Keyword::new(Category::VideoTerm))
            .add("LQ", Keyword::new(Category::VideoTerm))
            .add("HD", Keyword::new(Category::VideoTerm))
            .add("SD", Keyword::new(Category::VideoTerm))
            .add("VOL", Keyword::new(Category::VolumePrefix))
            .add("VOL.", Keyword::new(Category::VolumePrefix))
            .add("VOLUME", Keyword::new(Category::VolumePrefix))
            .add("ASS", Keyword::new(Category::Subtitles))
            .add("BIG5", Keyword::new(Category::Subtitles))
            .add("DUB", Keyword::new(Category::Subtitles))
            .add("DUBBED", Keyword::new(Category::Subtitles))
            .add("HARDSUB", Keyword::new(Category::Subtitles))
            .add("HARDSUBS", Keyword::new(Category::Subtitles))
            .add("RAW", Keyword::new(Category::Subtitles))
            .add("SOFTSUBS", Keyword::new(Category::Subtitles))
            .add("SUBBED", Keyword::new(Category::Subtitles))
            .add("SUBTITLED", Keyword::new(Category::Subtitles))
            .add("MULTIPLE SUBTITLE", Keyword::new(Category::Subtitles))
            .add("MULTI SUBS", Keyword::new(Category::Subtitles))
            .add("MULTI-SUBS", Keyword::new(Category::Subtitles))
            .add("SOFTSUB", Keyword::new(Category::Subtitles))
            .add("SUB", Keyword::new(Category::Subtitles))
            .add("BD", Keyword::new(Category::Source))
            .add("BDRIP", Keyword::new(Category::Source))
            .add("BLURAY", Keyword::new(Category::Source))
            .add("BLU-RAY", Keyword::new(Category::Source))
            .add("DVD", Keyword::new(Category::Source))
            .add("DVD5", Keyword::new(Category::Source))
            .add("DVD9", Keyword::new(Category::Source))
            .add("DVD-R2J", Keyword::new(Category::Source))
            .add("DVDRIP", Keyword::new(Category::Source))
            .add("DVD-RIP", Keyword::new(Category::Source))
            .add("R2DVD", Keyword::new(Category::Source))
            .add("R2J", Keyword::new(Category::Source))
            .add("R2JDVD", Keyword::new(Category::Source))
            .add("R2JDVDRIP", Keyword::new(Category::Source))
            .add("HDTV", Keyword::new(Category::Source))
            .add("HDTVRIP", Keyword::new(Category::Source))
            .add("TVRIP", Keyword::new(Category::Source))
            .add("TV-RIP", Keyword::new(Category::Source))
            .add("WEBCAST", Keyword::new(Category::Source))
            .add("WEBRIP", Keyword::new(Category::Source))
    }

    fn add(&mut self, keyword: &str, k: Keyword) -> Manager {
        self.keywords.insert(keyword.to_string(), k);
        self.to_owned()
    }

    /// Looks up a keyword by string, case-insensitively. Returns `None` if not found.
    pub(crate) fn find(&self, s: &str) -> Option<&Keyword> {
        let bytes = s.as_bytes();
        if bytes.len() <= 64 && s.is_ascii() {
            let mut buf = [0u8; 64];
            buf[..bytes.len()].copy_from_slice(bytes);
            buf[..bytes.len()].make_ascii_uppercase();
            // safe: buf contains only ASCII bytes (valid UTF-8)
            let upper = std::str::from_utf8(&buf[..bytes.len()]).unwrap();
            return self.keywords.get(upper);
        }
        self.keywords.get(&s.to_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::{Keyword, Manager};
    use crate::elements::Category;

    #[test]
    fn keyword_001() {
        let keyword = Keyword::new(Category::AnimeSeason)
            .identifiable(false)
            .valid(false)
            .searchable(false);
        assert!(!keyword.is_identifiable());
        assert!(!keyword.is_searchable());
        assert!(!keyword.is_valid());
    }

    #[test]
    fn keyword_002() {
        let keyword = Keyword::new(Category::AnimeSeason)
            .identifiable(false)
            .searchable(false);
        assert!(!keyword.is_identifiable());
        assert!(!keyword.is_searchable());
        assert!(keyword.is_valid());
    }

    #[test]
    fn keyword_003() {
        let keyword = Keyword::new(Category::AnimeSeason)
            .identifiable(false)
            .valid(false);
        assert!(!keyword.is_identifiable());
        assert!(keyword.is_searchable());
        assert!(!keyword.is_valid());
    }

    #[test]
    fn keyword_004() {
        let keyword = Keyword::new(Category::AnimeSeason)
            .searchable(false)
            .valid(false);
        assert!(keyword.is_identifiable());
        assert!(!keyword.is_searchable());
        assert!(!keyword.is_valid());
    }

    #[test]
    fn keyword_005() {
        let keyword = Keyword::new(Category::AnimeSeason).valid(false);
        assert!(keyword.is_identifiable());
        assert!(keyword.is_searchable());
        assert!(!keyword.is_valid());
    }

    #[test]
    fn keyword_006() {
        let keyword = Keyword::new(Category::AnimeSeason).searchable(false);
        assert!(keyword.is_identifiable());
        assert!(!keyword.is_searchable());
        assert!(keyword.is_valid());
    }

    #[test]
    fn keyword_007() {
        let keyword = Keyword::new(Category::AnimeSeason).identifiable(false);
        assert!(!keyword.is_identifiable());
        assert!(keyword.is_searchable());
        assert!(keyword.is_valid());
    }

    #[test]
    fn keyword_008() {
        let keyword = Keyword::new(Category::AnimeSeason);
        assert!(keyword.is_identifiable());
        assert!(keyword.is_searchable());
        assert!(keyword.is_valid());
    }

    #[test]
    fn manager_find() {
        let manager = Manager::new();
        assert!(manager.find("VOLUME").is_some());
        assert!(manager.find("My CUSTOM KEYWORD").is_none());
    }
}
