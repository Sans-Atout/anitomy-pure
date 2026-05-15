/// Category of a parsed element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    /// Season number (e.g. `2` in `S2E04`).
    AnimeSeason,
    /// Keyword preceding the season number (e.g. `S`, `Season`).
    AnimeSeasonPrefix,
    /// Main title of the series.
    AnimeTitle,
    /// Release type (e.g. `OVA`, `ONA`, `Movie`).
    AnimeType,
    /// Production year of the series.
    AnimeYear,
    /// Audio encoding or format (e.g. `FLAC`, `AAC`, `5.1`).
    AudioTerm,
    /// Device compatibility tag (e.g. `PS3`, `Xbox`).
    DeviceCompatibility,
    /// Episode number(s).
    EpisodeNumber,
    /// Alternative episode number (e.g. absolute number alongside season episode).
    EpisodeNumberAlt,
    /// Keyword preceding the episode number (e.g. `EP`, `Episode`).
    EpisodePrefix,
    /// Title of the individual episode.
    EpisodeTitle,
    /// CRC32 checksum (e.g. `[1234ABCD]`).
    FileChecksum,
    /// File extension (e.g. `mkv`, `mp4`).
    FileExtension,
    /// Full original filename, always present in the result.
    FileName,
    /// Audio or subtitle language (e.g. `ENG`, `JPN`).
    Language,
    /// Miscellaneous tag not covered by other categories.
    Other,
    /// Fansub or release group name.
    ReleaseGroup,
    /// Additional release information (e.g. `REPACK`, `BATCH`).
    ReleaseInformation,
    /// Release version suffix (e.g. `v2`, `v3`).
    ReleaseVersion,
    /// Source media (e.g. `Blu-ray`, `WEB-DL`, `HDTV`).
    Source,
    /// Subtitle type or language (e.g. `Sub`, `VOSTFR`, `Dual Audio`).
    Subtitles,
    /// Video resolution (e.g. `1080p`, `720p`, `1920x1080`).
    VideoResolution,
    /// Video encoding or format (e.g. `H.264`, `10bit`, `HEVC`).
    VideoTerm,
    /// Volume number (for manga-based or multi-volume releases).
    VolumeNumber,
    /// Keyword preceding the volume number (e.g. `Vol.`, `Volume`).
    VolumePrefix,
    /// Unrecognised token; not normally present in final results.
    Unknown,
}

impl Category {
    /// Returns `true` if this category can hold only one value (e.g. `AnimeTitle`, `EpisodeNumber`).
    pub fn is_singular(&self) -> bool {
        !matches!(
            self,
            Category::AnimeSeason
                | Category::AnimeType
                | Category::AudioTerm
                | Category::DeviceCompatibility
                | Category::EpisodeNumber
                | Category::Language
                | Category::Other
                | Category::ReleaseInformation
                | Category::Source
                | Category::VideoTerm
        )
    }

    /// Returns `true` if this category is looked up in the keyword manager during parsing.
    pub fn is_searchable(&self) -> bool {
        matches!(
            self,
            Category::AnimeSeasonPrefix
                | Category::AnimeType
                | Category::AudioTerm
                | Category::DeviceCompatibility
                | Category::EpisodePrefix
                | Category::FileChecksum
                | Category::Language
                | Category::Other
                | Category::ReleaseGroup
                | Category::ReleaseInformation
                | Category::ReleaseVersion
                | Category::Source
                | Category::Subtitles
                | Category::VideoResolution
                | Category::VideoTerm
                | Category::VolumePrefix
        )
    }
}

/// A single extracted value with its associated category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    /// The category this element belongs to.
    pub category: Category,
    /// The extracted string value.
    pub value: String,
}

impl Element {
    /// Creates a new element with the given category and value.
    pub fn new(c: Category, v: &str) -> Element {
        Element {
            category: c,
            value: v.to_string(),
        }
    }
}

/// Collection of all elements extracted from a filename.
///
/// Returned by [`Parser::parse`](crate::Parser::parse).
#[derive(Debug, Eq, Clone, Default)]
pub struct Elements {
    elements: Vec<Element>,
}

impl Elements {
    /// Creates an empty collection.
    pub fn new() -> Elements {
        Elements::default()
    }

    /// Total number of elements across all categories.
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// Returns `true` if an element with the given category and exact value exists.
    pub fn contains(&self, c: Category, v: &str) -> bool {
        self.elements.iter().any(|e| e.category == c && e.value == v)
    }

    /// Appends a new element (mutable, in-place).
    pub fn push(&mut self, c: Category, v: &str) {
        self.elements.push(Element::new(c, v));
    }

    /// Appends a new element and returns `self` (builder style).
    pub fn add(mut self, c: Category, v: &str) -> Elements {
        self.elements.push(Element::new(c, v));
        self
    }

    /// Returns the first element matching the given category, or `None`.
    pub fn find(&self, c: Category) -> Option<Element> {
        self.elements.iter().find(|e| e.category == c).cloned()
    }

    /// Returns all elements matching the given category, or `None` if there are none.
    ///
    /// Useful for multi-episode ranges where several [`EpisodeNumber`](Category::EpisodeNumber)
    /// elements may be present.
    pub fn find_all(&self, c: Category) -> Option<Vec<Element>> {
        let v: Vec<Element> = self.elements.iter().filter(|e| e.category == c).cloned().collect();
        if v.is_empty() { None } else { Some(v) }
    }

    /// Returns the number of elements for the given category.
    pub fn count(&self, c: Category) -> i32 {
        let mut count = 0;
        for e in self.elements.iter() {
            if c == e.category {
                count += 1;
            }
        }
        count
    }

    /// Returns `true` if no elements have been extracted at all.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns `true` if no element of the given category was found.
    pub fn is_category_empty(&self, c: Category) -> bool {
        !self.elements.iter().any(|e| e.category == c)
    }

    /// Removes and returns the value of the first element matching the given category.
    pub fn remove_first(&mut self, c: Category) -> Option<String> {
        if let Some(pos) = self.elements.iter().position(|e| e.category == c) {
            Some(self.elements.remove(pos).value)
        } else {
            None
        }
    }
}

impl PartialEq for Elements {
    fn eq(&self, other: &Self) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }
        for e in self.elements.iter() {
            if !other.elements.contains(e) {
                return false;
            }
        }
        true
    }
}
