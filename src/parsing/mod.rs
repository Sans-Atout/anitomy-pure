//! Parsing passes that progressively extract elements from the token list.
use std::sync::LazyLock;

use crate::{
    elements::{Category, Elements},
    keyword::Manager,
    token::{main_token::Token, subtoken::SubTokenCategory},
};

pub(super) static KEYWORD_MANAGER: LazyLock<Manager> = LazyLock::new(Manager::new);

use self::{
    number::{is_anime_year, is_crc32, is_digit, is_resolution, ordinals_to_nb},
    string::parse_multiple_keyword,
};

/// Episode number extraction.
pub(crate) mod episode;
/// File extension detection.
pub(crate) mod extensions;
/// Numeric pattern recognition helpers.
pub(crate) mod number;
/// Anime title, release group and episode title extraction.
pub(crate) mod string;

/// Runs the keyword recognition pass on a single token, marking matched subtokens as `Found`.
pub(crate) fn parsing_single_token(elements: &mut Elements, token: &mut Token, manager: &Manager) {
    let inside_delimiter = token.is_inside_delimiter();
    let is_paren = token.is_paren();
    let (raw, sub_tokens) = token.raw_and_subtokens();

    for index in 0..sub_tokens.len() {
        let tested_value = sub_tokens[index].value();
        if tested_value.is_empty() || sub_tokens[index].is_category(SubTokenCategory::Found) {
            continue;
        }
        if is_digit(tested_value) && tested_value.len() != 8 {
            if index + 1 < sub_tokens.len() {
                let left = sub_tokens[index].value();
                let right = sub_tokens[index + 1].value();
                if left == "5" && right == "1" {
                    sub_tokens[index].category(SubTokenCategory::Found);
                    sub_tokens[index + 1].category(SubTokenCategory::Found);
                    elements.push(Category::AudioTerm, "5.1");
                } else if parse_multiple_keyword(elements, manager, left, right) {
                    sub_tokens[index].category(SubTokenCategory::Found);
                    sub_tokens[index + 1].category(SubTokenCategory::Found);
                } else if inside_delimiter && is_digit(right) {
                    let decimal = format!("{}.{}", left, right);
                    if raw.contains(&decimal) {
                        let preceding_is_numeric = index == 0 || {
                            let prev = &sub_tokens[index - 1];
                            prev.is_category(SubTokenCategory::Found) || is_digit(prev.value())
                        };
                        if preceding_is_numeric {
                            sub_tokens[index].category(SubTokenCategory::Found);
                            sub_tokens[index + 1].category(SubTokenCategory::Found);
                        }
                    }
                }
            }
            continue;
        }
        if is_crc32(tested_value) && elements.is_category_empty(Category::FileChecksum) {
            elements.push(Category::FileChecksum, tested_value);
            sub_tokens[index].category(SubTokenCategory::Found);
            continue;
        }
        if is_resolution(tested_value) {
            elements.push(Category::VideoResolution, tested_value);
            sub_tokens[index].category(SubTokenCategory::Found);
            continue;
        }
        if let Some(keyword) = manager.find(tested_value) {
            let c = keyword.get_category();
            if (!c.is_searchable())
                || (c.is_singular() && !elements.is_category_empty(c))
                || elements.contains(c, tested_value)
            {
                continue;
            }
            if c == Category::Language && !keyword.is_identifiable() && !inside_delimiter {
                continue;
            }

            if tested_value.to_uppercase() == "E" && index + 2 < sub_tokens.len() {
                let supiscious_keyword = format!(
                    "{}-{}-{}",
                    tested_value,
                    sub_tokens[index + 1].value(),
                    sub_tokens[index + 2].value()
                );
                if supiscious_keyword.to_uppercase() == "E-AC-3" {
                    sub_tokens[index].category(SubTokenCategory::Found);
                    sub_tokens[index + 1].category(SubTokenCategory::Found);
                    sub_tokens[index + 2].category(SubTokenCategory::Found);
                    elements.push(Category::AudioTerm, &supiscious_keyword);
                    continue;
                }
            }
            if tested_value.to_uppercase() == "DTS" && index + 1 < sub_tokens.len() {
                let supiscious_keyword =
                    format!("{}-{}", tested_value, sub_tokens[index + 1].value());
                if supiscious_keyword.to_uppercase() == "DTS-ES" {
                    sub_tokens[index].category(SubTokenCategory::Found);
                    sub_tokens[index + 1].category(SubTokenCategory::Found);
                    elements.push(Category::AudioTerm, &supiscious_keyword);
                    continue;
                }
            }
            if c == Category::AnimeSeasonPrefix {
                elements.push(c, tested_value);
                sub_tokens[index].category(SubTokenCategory::Found);
                if (index as i32 - 1) >= 0 {
                    let previous = sub_tokens[index - 1].value();
                    let p_saeson = ordinals_to_nb(previous);
                    if !p_saeson.is_empty() {
                        elements.push(Category::AnimeSeason, p_saeson);
                        sub_tokens[index - 1].category(SubTokenCategory::Found);
                        continue;
                    }
                }
                if index + 1 < sub_tokens.len() {
                    let next = sub_tokens[index + 1].value();
                    if is_digit(next) {
                        elements.push(Category::AnimeSeason, next);
                        sub_tokens[index + 1].category(SubTokenCategory::Found);
                    }
                }
                continue;
            }

            if c == Category::EpisodePrefix {
                let is_valid = keyword.is_valid();
                if index + 1 < sub_tokens.len() {
                    let next = sub_tokens[index + 1].value();
                    if is_digit(next) {
                        let left = next.parse::<i32>().unwrap_or(0);
                        elements.push(Category::EpisodeNumber, next);
                        sub_tokens[index + 1].category(SubTokenCategory::Found);
                        if index + 2 < sub_tokens.len() {
                            let after = sub_tokens[index + 2].value();
                            if is_digit(after) {
                                let right = after.parse::<i32>().unwrap_or(0);
                                if left < right {
                                    elements.push(Category::EpisodeNumber, after);
                                    sub_tokens[index + 2].category(SubTokenCategory::Found);
                                }
                            }
                        }
                        if is_valid {
                            elements.push(c, sub_tokens[index].value());
                        }
                        sub_tokens[index].category(SubTokenCategory::Found);
                        continue;
                    }
                }
                if is_valid {
                    elements.push(c, sub_tokens[index].value());
                    sub_tokens[index].category(SubTokenCategory::Found);
                }
                continue;
            }

            if c == Category::ReleaseVersion {
                let release = sub_tokens[index].value().to_lowercase().replace('v', "");
                elements.push(Category::ReleaseVersion, &release);
                sub_tokens[index].category(SubTokenCategory::Found);
                continue;
            }
            if c == Category::VolumePrefix {
                if index + 1 < sub_tokens.len() {
                    let next = sub_tokens[index + 1].value();
                    if is_digit(next) {
                        if !keyword.is_identifiable() {
                            elements.push(c, tested_value);
                        }
                        elements.push(Category::VolumeNumber, next);
                        sub_tokens[index + 1].category(SubTokenCategory::Found);
                        sub_tokens[index].category(SubTokenCategory::Found);
                        continue;
                    }
                    let leading_digits: String =
                        next.chars().take_while(|ch| ch.is_ascii_digit()).collect();
                    if !leading_digits.is_empty() {
                        let remainder = &next[leading_digits.len()..];
                        if (remainder.starts_with('v') || remainder.starts_with('V'))
                            && is_digit(&remainder[1..])
                        {
                            if !keyword.is_identifiable() {
                                elements.push(c, tested_value);
                            }
                            elements.push(Category::VolumeNumber, &leading_digits);
                            elements.push(Category::ReleaseVersion, &remainder[1..]);
                            sub_tokens[index + 1].category(SubTokenCategory::Found);
                            sub_tokens[index].category(SubTokenCategory::Found);
                            continue;
                        }
                    }
                }
                if !keyword.is_identifiable() {
                    elements.push(Category::AnimeType, tested_value);
                } else {
                    elements.push(c, tested_value);
                    sub_tokens[index].category(SubTokenCategory::Found);
                }
                continue;
            }
            if c == Category::AnimeType {
                if inside_delimiter {
                    let has_truly_unknown = sub_tokens.iter().enumerate().any(|(i, s)| {
                        if i == index || !s.is_category(SubTokenCategory::Unknow) {
                            return false;
                        }
                        let v = s.value();
                        !is_resolution(v)
                            && !is_digit(v)
                            && !is_crc32(v)
                            && manager.find(v).is_none()
                    });
                    if has_truly_unknown {
                        continue;
                    }
                }
                elements.push(c, tested_value);
                if !keyword.is_searchable() {
                    sub_tokens[index].category(SubTokenCategory::Found);
                }
                continue;
            }
            if c == Category::Language {
                if inside_delimiter && is_paren {
                    let has_non_keyword_unknown = sub_tokens.iter().enumerate().any(|(i, s)| {
                        if i == index || !s.is_category(SubTokenCategory::Unknow) {
                            return false;
                        }
                        manager.find(s.value()).is_none()
                    });
                    if has_non_keyword_unknown {
                        continue;
                    }
                }
                let mut language_found = false;
                if let Some(languages) = elements.find_all(Category::Language) {
                    for language in languages {
                        if language.value.to_uppercase() == tested_value.to_uppercase() {
                            language_found = true;
                            break;
                        }
                    }
                }
                if language_found {
                    continue;
                }
                elements.push(c, tested_value);
                sub_tokens[index].category(SubTokenCategory::Found);
                continue;
            }
            if c == Category::ReleaseGroup && inside_delimiter {
                let has_other_unknowns = sub_tokens
                    .iter()
                    .enumerate()
                    .any(|(i, s)| i != index && s.is_category(SubTokenCategory::Unknow));
                if has_other_unknowns {
                    continue;
                }
            }
            if c == Category::Subtitles
                && !inside_delimiter
                && elements.is_category_empty(Category::Language)
            {
                sub_tokens[index].category(SubTokenCategory::Found);
                continue;
            }
            if c != Category::Unknown {
                if c == Category::ReleaseInformation && !keyword.is_identifiable() {
                    let has_prior_found =
                        (0..index).any(|i| sub_tokens[i].is_category(SubTokenCategory::Found));
                    if has_prior_found {
                        sub_tokens[index].category(SubTokenCategory::Found);
                        continue;
                    }
                }
                if c == Category::ReleaseInformation
                    && inside_delimiter
                    && !is_paren
                    && index > 0
                    && sub_tokens[0].is_category(SubTokenCategory::Unknow)
                {
                    continue;
                }
                elements.push(c, tested_value);
                sub_tokens[index].category(SubTokenCategory::Found);
                continue;
            }
        }

        if index + 1 < sub_tokens.len() {
            let left = sub_tokens[index].value();
            let right = sub_tokens[index + 1].value();
            if parse_multiple_keyword(elements, manager, left, right) {
                sub_tokens[index].category(SubTokenCategory::Found);
                sub_tokens[index + 1].category(SubTokenCategory::Found);
            }
        }
    }
}

/// Runs the keyword recognition pass across all tokens.
pub(crate) fn parsing_keywords(elements: &mut Elements, tokens: &mut [Token]) {
    for token in tokens.iter_mut() {
        parsing_single_token(elements, token, &KEYWORD_MANAGER);
        if token.contains_unknow() && token.is_isolated_number() {
            let value = token.sub_tokens()[0].value().to_string();
            if is_anime_year(&value) {
                elements.push(Category::AnimeYear, &value);
                token.sub_tokens()[0].category(SubTokenCategory::Found);
            }
            if (value == "480" || value == "720" || value == "1080")
                && elements.is_category_empty(Category::VideoResolution)
            {
                elements.push(Category::VideoResolution, &value);
                token.sub_tokens()[0].category(SubTokenCategory::Found);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::episode::{
        match_fractal_episode, match_japanese_counter, match_multiple_ep,
        match_partial_episode_pattern, match_season_ep_patern, match_type_episode, parse_single_ep,
        parse_single_subtoken,
    };
    use super::number::{
        contains_digit, is_anime_year, is_crc32, is_digit, is_hexa, is_resolution, ordinals_to_nb,
    };
    use super::string::{parse_anime_title, parse_episode_title, parse_release_group};
    use crate::elements::{Category, Element, Elements};
    use crate::token::{main_token::Token, subtoken::SubTokenCategory};

    #[test]
    fn crc32() {
        assert!(is_crc32("C09462E2"));
        assert!(is_crc32("8F59F2BA"));
        assert!(!is_crc32("8F5GF2BA"));
        assert!(!is_crc32("8F50F2BAA"));
        assert!(!is_crc32("8F50F2B"));
    }

    #[test]
    fn resolution() {
        assert!(is_resolution("1080p"));
        assert!(is_resolution("2K"));
        assert!(is_resolution("1920x1080"));
        assert!(is_resolution("720p"));
        assert!(!is_resolution("Hello world"));
        assert!(is_resolution("1080×720"));
    }

    #[test]
    fn ordinal() {
        assert_eq!(ordinals_to_nb("First"), "1");
        assert_eq!(ordinals_to_nb("second"), "2");
        assert_eq!(ordinals_to_nb("5th"), "5");
        assert_eq!(ordinals_to_nb("5the"), "");
    }

    #[test]
    fn anime_year() {
        assert!(is_anime_year("2009"));
        assert!(is_anime_year("1920"));
        assert!(!is_anime_year("1400"));
        assert!(is_anime_year("2050"));
        assert!(!is_anime_year("2500"));
        assert!(!is_anime_year("gdyuighjdhgj"));
    }

    #[test]
    fn test_contains_number() {
        assert!(contains_digit("E01S02"));
        assert!(contains_digit("032"));
        assert!(!contains_digit("A"));
        assert!(!contains_digit("Hello World"));
    }

    #[test]
    fn test_hexa() {
        assert!(is_hexa("028934"));
        assert!(is_hexa("FFF"));
        assert!(is_hexa("0123456789ABCDEF"));
        assert!(!is_hexa("G015021"));
        assert!(is_hexa("acbdef"));
        assert!(!is_hexa("00000fg"))
    }

    #[test]
    fn test_isdigit() {
        assert!(!is_digit("FFF"));
        assert!(is_digit("256"));
        assert!(is_digit("-120"));
        assert!(!is_digit("Hello World"));
    }

    #[test]
    fn match_single_ep_patern() {
        let mut tmp_e = Elements::new();
        let wanted_element = Elements::new()
            .add(Category::EpisodeNumber, "01")
            .add(Category::ReleaseVersion, "2");
        assert!(parse_single_ep("01v2", &mut tmp_e));
        assert_eq!(tmp_e, wanted_element);
    }

    #[test]
    fn match_multiple_ep_patern_01() {
        let mut tmp_e = Elements::new();
        let wanted_element = Elements::new()
            .add(Category::EpisodeNumber, "01")
            .add(Category::EpisodeNumber, "03")
            .add(Category::ReleaseVersion, "2");
        assert!(match_multiple_ep("01v2-03", &mut tmp_e));
        assert_eq!(tmp_e, wanted_element);
    }

    #[test]
    fn match_multiple_ep_patern_02() {
        let mut tmp_e = Elements::new();
        let wanted_element = Elements::new()
            .add(Category::EpisodeNumber, "01")
            .add(Category::EpisodeNumber, "03")
            .add(Category::ReleaseVersion, "2");
        assert!(match_multiple_ep("01-03v2", &mut tmp_e));
        assert_eq!(tmp_e, wanted_element);
    }

    #[test]
    fn match_multiple_ep_patern_03() {
        let mut tmp_e = Elements::new();
        let wanted_element = Elements::new()
            .add(Category::EpisodeNumber, "01")
            .add(Category::EpisodeNumber, "03")
            .add(Category::ReleaseVersion, "1")
            .add(Category::ReleaseVersion, "2");
        assert!(match_multiple_ep("01v1-03v2", &mut tmp_e));
        assert_eq!(tmp_e, wanted_element);
    }

    #[test]
    fn match_multiple_ep_patern_04() {
        let mut tmp_e = Elements::new();
        let wanted_element = Elements::new()
            .add(Category::EpisodeNumber, "01")
            .add(Category::EpisodeNumber, "03");
        assert!(match_multiple_ep("01-03", &mut tmp_e));
        assert_eq!(tmp_e, wanted_element);
    }

    #[test]
    fn match_episode_type() {
        let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
        let mut tmp_e = Elements::new();
        let wanted_element = Elements::new()
            .add(Category::AnimeType, "ONA")
            .add(Category::EpisodeNumber, "01")
            .add(Category::ReleaseVersion, "3");
        assert!(match_type_episode("ONA01v3", &mut tmp_e, &d));
        assert_eq!(tmp_e, wanted_element);
        tmp_e = Elements::new();
        assert!(!match_type_episode("ONAFail", &mut tmp_e, &d));
        assert_eq!(tmp_e, Elements::new());
    }

    #[test]
    fn test_japanese_ep() {
        let mut tmp_e = Elements::new();
        let wanted = Elements::new().add(Category::EpisodeNumber, "125");
        assert!(!match_japanese_counter("ONAFail", &mut tmp_e));
        assert!(match_japanese_counter("125話", &mut tmp_e));
        assert_eq!(tmp_e, wanted);
    }

    #[test]
    fn test_fractal() {
        let mut tmp_e = Elements::new();
        assert!(match_fractal_episode("11.1", &mut tmp_e));

        let mut tmp_e2 = Elements::new();
        let wanted = Elements::new().add(Category::EpisodeNumber, "1.5");
        assert!(match_fractal_episode("1.5", &mut tmp_e2));
        assert_eq!(tmp_e2, wanted);
    }

    #[test]
    fn test_parse_single_subtoken() {
        let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
        let mut e = Elements::new();
        assert!(parse_single_subtoken(&d, "01v2", &mut e));
        assert!(parse_single_subtoken(&d, "1.5", &mut e));
        assert!(match_fractal_episode("1.5", &mut e));
        assert!(parse_single_subtoken(&d, "01-03", &mut e));
        assert!(parse_single_subtoken(&d, "S01E02", &mut e));
        assert!(parse_single_subtoken(&d, "ONA1.5", &mut e));
        assert!(parse_single_subtoken(&d, "125話", &mut e));
        assert!(!parse_single_subtoken(&d, "03-02", &mut e));
        assert!(parse_single_subtoken(&d, "125A", &mut e));
        assert!(parse_single_subtoken(&d, "125a", &mut e));
        assert!(parse_single_subtoken(&d, "#32v1", &mut e));
    }

    #[test]
    fn test_season_ep_patern() {
        let mut e = Elements::new();
        assert!(match_season_ep_patern("S02E01", &mut e));
        assert!(match_season_ep_patern("S02E01-03", &mut e));
        assert!(!match_season_ep_patern("SAE01", &mut e));
        assert!(match_season_ep_patern("S01-02E01", &mut e));
        assert!(match_season_ep_patern("01x02", &mut e));
    }

    #[test]
    fn test_match_partial_episode_pattern() {
        let mut e = Elements::new();
        assert!(match_partial_episode_pattern("125a", &mut e));
    }

    #[test]
    fn test_find_release_group() {
        let d: Vec<char> = vec![' ', '_', '.', '-', '&', '+', ',', '|'];
        let mut e = Elements::new();
        let mut parsing_data = vec![
            Token::new("Kira-Fansub", &d, true, false, false),
            Token::new(" Uchuu no Stellvia ep 14 ", &d, false, false, false),
            Token::new("BD 1280x960 24fps AAC", &d, true, false, false),
            Token::new("06EE7355", &d, true, true, false),
        ];
        parse_release_group(&mut parsing_data, &mut e, &d);
        let tested = e.find(Category::ReleaseGroup).unwrap();
        let wanted = Element::new(Category::ReleaseGroup, "Kira-Fansub");
        assert_eq!(tested, wanted)
    }

    #[test]
    fn test_find_anime_title_001() {
        let d: Vec<char> = vec![' ', '_', '.', '-', '&', '+', ',', '|'];
        let mut e = Elements::new();
        let mut anime_title_subtoken: Token =
            Token::new(" Uchuu no Stellvia ep 14 ", &d, false, false, false);
        let subtoken = anime_title_subtoken.sub_tokens();
        subtoken[3].category(SubTokenCategory::Found);
        subtoken[4].category(SubTokenCategory::Found);
        let mut parsing_data = vec![
            Token::new("Kira-Fansub", &d, true, false, false),
            anime_title_subtoken,
            Token::new("BD 1280x960 24fps AAC", &d, true, true, false),
            Token::new("06EE7355", &d, true, false, false),
        ];

        parse_anime_title(&mut parsing_data, &mut e, &d);
        let tested = e.find(Category::AnimeTitle).unwrap();
        let wanted = Element::new(Category::AnimeTitle, "Uchuu no Stellvia");
        assert_eq!(tested, wanted)
    }

    #[test]
    fn test_find_anime_title_002() {
        let d: Vec<char> = vec![' ', '_', '.', '-', '&', '+', ',', '|'];
        let mut e = Elements::new();
        let mut anime_title_subtoken: Token =
            Token::new(" Uchuu no Stellvia ep 14 ", &d, true, false, false);
        let subtoken = anime_title_subtoken.sub_tokens();
        subtoken[3].category(SubTokenCategory::Found);
        subtoken[4].category(SubTokenCategory::Found);
        let mut parsing_data = vec![
            Token::new("Kira-Fansub", &d, true, false, false),
            anime_title_subtoken,
            Token::new("BD 1280x960 24fps AAC", &d, true, true, false),
            Token::new("06EE7355", &d, true, false, false),
        ];

        parse_anime_title(&mut parsing_data, &mut e, &d);
        let tested = e.find(Category::AnimeTitle).unwrap();
        let wanted = Element::new(Category::AnimeTitle, "Uchuu no Stellvia");
        assert_eq!(tested, wanted)
    }

    #[test]
    fn test_find_episode_title() {
        let d: Vec<char> = vec![' ', '_', '.', '-', '&', '+', ',', '|'];
        let mut e = Elements::new();
        let mut anime_title_subtoken: Token = Token::new(
            " Uchuu no Stellvia ep 14 My Super Episode title",
            &d,
            false,
            false,
            false,
        );
        let subtoken = anime_title_subtoken.sub_tokens();
        subtoken[3].category(SubTokenCategory::Found);
        subtoken[4].category(SubTokenCategory::Found);
        let mut parsing_data = vec![
            Token::new("Kira-Fansub", &d, true, false, false),
            anime_title_subtoken,
            Token::new("BD 1280x960 24fps AAC", &d, true, true, false),
            Token::new("06EE7355", &d, true, false, false),
        ];

        parse_episode_title(&mut parsing_data, &mut e, &d);
        let tested = e.find(Category::EpisodeTitle).unwrap();
        let wanted = Element::new(Category::EpisodeTitle, "Uchuu no Stellvia");
        assert_eq!(tested, wanted)
    }
}
