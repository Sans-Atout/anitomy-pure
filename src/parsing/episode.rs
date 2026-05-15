//! Episode number extraction: prefix patterns, season+episode patterns, and bare digit heuristics.
use crate::{
    elements::{Category, Elements},
    split::split_type_and_ep,
    token::{main_token::Token, subtoken::SubTokenCategory},
};

use super::number::{contains_digit, is_anime_year, is_digit};

// ── helpers ──────────────────────────────────────────────────────────────────

fn all_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

/// Parse "NNNNvD" → Some(("NNNN", Some("D"))) or "NNNN" → Some(("NNNN", None)).
/// Episodes are 1-4 digits; version is a single digit.
fn parse_ep_with_version(s: &str) -> Option<(&str, Option<&str>)> {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return None;
    }
    // Check for trailing vN
    if n >= 2 && (b[n - 2] == b'v' || b[n - 2] == b'V') && b[n - 1].is_ascii_digit() {
        let digits = &s[..n - 2];
        if !digits.is_empty() && digits.len() <= 4 && all_digits(digits) {
            return Some((digits, Some(&s[n - 1..])));
        }
    }
    if s.len() <= 4 && all_digits(s) {
        return Some((s, None));
    }
    None
}

fn is_fractal(s: &str) -> bool {
    s.find('.').is_some_and(|dot| {
        let l = &s[..dot];
        let r = &s[dot + 1..];
        all_digits(l) && all_digits(r)
    })
}

// ── public matchers ───────────────────────────────────────────────────────────

/// `(\d{1,4})[vV](\d)$`
pub(crate) fn parse_single_ep(s: &str, elements: &mut Elements) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    if !(3..=6).contains(&n) {
        return false;
    }
    if !b[n - 1].is_ascii_digit() || (b[n - 2] != b'v' && b[n - 2] != b'V') {
        return false;
    }
    let ep = &s[..n - 2];
    if ep.is_empty() || ep.len() > 4 || !all_digits(ep) {
        return false;
    }
    elements.push(Category::EpisodeNumber, ep);
    elements.push(Category::ReleaseVersion, &s[n - 1..]);
    true
}

/// `(\d{1,4})(vN)?[-~&+](\d{1,4})(vN)?` — range like "01-03", "01v2-03", "01v1-03v2"
pub(crate) fn match_multiple_ep(s: &str, elements: &mut Elements) -> bool {
    let sep = s
        .bytes()
        .position(|b| matches!(b, b'-' | b'~' | b'&' | b'+'));
    let Some(sep_idx) = sep else { return false };
    let left = &s[..sep_idx];
    let right = &s[sep_idx + 1..];
    let Some((ep1, ver1)) = parse_ep_with_version(left) else {
        return false;
    };
    let Some((ep2, ver2)) = parse_ep_with_version(right) else {
        return false;
    };
    let n1: u32 = ep1.parse().unwrap_or(u32::MAX);
    let n2: u32 = ep2.parse().unwrap_or(0);
    if n1 >= n2 {
        return false;
    }
    elements.push(Category::EpisodeNumber, ep1);
    elements.push(Category::EpisodeNumber, ep2);
    if let Some(v) = ver1 {
        elements.push(Category::ReleaseVersion, v);
    }
    if let Some(v) = ver2 {
        elements.push(Category::ReleaseVersion, v);
    }
    true
}

/// `S?(\d{1,2})(-S?\d{1,2})?([xE])(\d{1,4})(-E?\d{1,4})?([vV]\d)?`
pub(crate) fn match_season_ep_patern(s: &str, elements: &mut Elements) -> bool {
    let b = s.as_bytes();
    let len = b.len();
    let mut pos = 0;

    // optional 'S'
    if pos < len && (b[pos] == b'S' || b[pos] == b's') {
        pos += 1;
    }

    // 1–2 digits → season1
    let s1_start = pos;
    while pos < len && b[pos].is_ascii_digit() && pos - s1_start < 2 {
        pos += 1;
    }
    if pos == s1_start {
        return false;
    }
    let season1 = &s[s1_start..pos];
    if season1.parse::<u32>().unwrap_or(0) == 0 {
        return false;
    }

    // optional season range: '-' + optional 'S' + 1–2 digits
    let season2 = if pos < len && b[pos] == b'-' {
        let save = pos;
        pos += 1;
        if pos < len && (b[pos] == b'S' || b[pos] == b's') {
            pos += 1;
        }
        let s2_start = pos;
        while pos < len && b[pos].is_ascii_digit() && pos - s2_start < 2 {
            pos += 1;
        }
        if pos == s2_start {
            pos = save; // backtrack
            None
        } else {
            Some(&s[s2_start..pos])
        }
    } else {
        None
    };

    // separator: 'x'/'X' or optional[' '.'_''-'] + 'E'/'e'
    if pos >= len {
        return false;
    }
    if b[pos] == b'x' || b[pos] == b'X' {
        pos += 1;
    } else {
        if matches!(b[pos], b' ' | b'.' | b'_' | b'-') {
            pos += 1;
        }
        if pos >= len || (b[pos] != b'E' && b[pos] != b'e') {
            return false;
        }
        pos += 1;
    }

    // 1–4 digits → ep1
    let ep1_start = pos;
    while pos < len && b[pos].is_ascii_digit() && pos - ep1_start < 4 {
        pos += 1;
    }
    if pos == ep1_start {
        return false;
    }
    let ep1 = &s[ep1_start..pos];

    // optional ep range: '-' + optional 'E'/'e' + 1–4 digits
    let ep2 = if pos < len && b[pos] == b'-' {
        let save = pos;
        pos += 1;
        if pos < len && (b[pos] == b'E' || b[pos] == b'e') {
            pos += 1;
        }
        let ep2_start = pos;
        while pos < len && b[pos].is_ascii_digit() && pos - ep2_start < 4 {
            pos += 1;
        }
        if pos == ep2_start {
            pos = save;
            None
        } else {
            Some(&s[ep2_start..pos])
        }
    } else {
        None
    };

    // optional version: 'v'/'V' + single digit
    let version =
        if pos + 1 < len && (b[pos] == b'v' || b[pos] == b'V') && b[pos + 1].is_ascii_digit() {
            pos += 2;
            Some(&s[pos - 1..pos])
        } else {
            None
        };

    if pos != len {
        return false;
    }

    elements.push(Category::AnimeSeason, season1);
    if let Some(s2) = season2 {
        elements.push(Category::AnimeSeason, s2);
    }
    elements.push(Category::EpisodeNumber, ep1);
    if let Some(e2) = ep2 {
        elements.push(Category::EpisodeNumber, e2);
    }
    if let Some(v) = version {
        elements.push(Category::ReleaseVersion, v);
    }
    true
}

/// `\d+\.\d+` (fractal episode like "1.5", "11.1")
pub(crate) fn match_fractal_episode(s: &str, elements: &mut Elements) -> bool {
    if is_fractal(s) {
        elements.push(Category::EpisodeNumber, s);
        return true;
    }
    false
}

/// `(\d+)話$`
pub(crate) fn match_japanese_counter(s: &str, elements: &mut Elements) -> bool {
    if !s.ends_with('話') {
        return false;
    }
    let num = &s[..s.len() - '話'.len_utf8()];
    if all_digits(num) {
        elements.push(Category::EpisodeNumber, num);
        return true;
    }
    false
}

/// `^#(\d{1,4})([-~&+](\d{1,4}))?([vV](\d))?$`
pub(crate) fn match_number_sign_patern(s: &str, elements: &mut Elements) -> bool {
    if !s.starts_with('#') {
        return false;
    }
    let rest = &s[1..];
    let b = rest.as_bytes();
    let mut pos = 0;

    // ep1: 1–4 digits
    let ep1_start = 0;
    while pos < b.len() && b[pos].is_ascii_digit() && pos < 4 {
        pos += 1;
    }
    if pos == ep1_start {
        return false;
    }
    let ep1 = &rest[..pos];

    // optional range: separator + 1–4 digits
    let ep2 = if pos < b.len() && matches!(b[pos], b'-' | b'~' | b'&' | b'+') {
        pos += 1;
        let ep2_start = pos;
        while pos < b.len() && b[pos].is_ascii_digit() && pos - ep2_start < 4 {
            pos += 1;
        }
        if pos == ep2_start {
            return false;
        }
        Some(&rest[ep2_start..pos])
    } else {
        None
    };

    // optional version
    let version =
        if pos + 1 < b.len() && (b[pos] == b'v' || b[pos] == b'V') && b[pos + 1].is_ascii_digit() {
            pos += 2;
            Some(&rest[pos - 1..pos])
        } else {
            None
        };

    if pos != b.len() {
        return false;
    }

    elements.push(Category::EpisodeNumber, ep1);
    if let Some(e2) = ep2 {
        elements.push(Category::EpisodeNumber, e2);
    }
    if let Some(v) = version {
        elements.push(Category::ReleaseVersion, v);
    }
    true
}

/// Digits followed by a single A/B/C suffix — e.g. "125a", "07B"
pub(crate) fn match_partial_episode_pattern(s: &str, elements: &mut Elements) -> bool {
    let b = s.as_bytes();
    if b.is_empty() {
        return false;
    }
    let last = b[b.len() - 1].to_ascii_uppercase();
    if !matches!(last, b'A' | b'B' | b'C') {
        return false;
    }
    let prefix = &s[..s.len() - 1];
    if prefix.is_empty() || !all_digits(prefix) {
        return false;
    }
    elements.push(Category::EpisodeNumber, s);
    true
}

// ── parse_single_subtoken ────────────────────────────────────────────────────

/// Tries every episode pattern on `string_to_parse`; returns `true` if one matched.
pub(crate) fn parse_single_subtoken(
    delimiter: &[char],
    string_to_parse: &str,
    found_elements: &mut Elements,
) -> bool {
    if match_number_sign_patern(string_to_parse, found_elements) {
        return true;
    }
    if match_multiple_ep(string_to_parse, found_elements) {
        return true;
    }
    if match_season_ep_patern(string_to_parse, found_elements) {
        return true;
    }
    if match_type_episode(string_to_parse, found_elements, delimiter) {
        return true;
    }
    if parse_single_ep(string_to_parse, found_elements) {
        return true;
    }
    if match_fractal_episode(string_to_parse, found_elements) {
        return true;
    }
    if match_partial_episode_pattern(string_to_parse, found_elements) {
        return true;
    }
    if match_japanese_counter(string_to_parse, found_elements) {
        return true;
    }
    false
}

/// Matches an explicit episode-prefix pattern (`EP01`, `Episode 7`, `第01話`, …); returns `true` on match.
pub(crate) fn match_type_episode(
    tested_string: &str,
    found_elements: &mut Elements,
    delimiter: &[char],
) -> bool {
    let (potential_keyword, data_to_parse) = split_type_and_ep(tested_string);
    let trim_keyword = potential_keyword.trim_matches(delimiter);
    if let Some(keyword) = super::KEYWORD_MANAGER.find(trim_keyword) {
        let tmp_c = keyword.get_category();
        let suppress_keyword =
            tmp_c == Category::EpisodePrefix && (!keyword.is_valid() || is_fractal(data_to_parse));
        if !suppress_keyword {
            found_elements.push(tmp_c, trim_keyword);
        }
        if is_digit(data_to_parse) {
            if tmp_c == Category::AnimeSeasonPrefix {
                found_elements.push(Category::AnimeSeason, data_to_parse);
            }
            if tmp_c == Category::EpisodePrefix || tmp_c == Category::AnimeType {
                found_elements.push(Category::EpisodeNumber, data_to_parse);
            }
            if tmp_c == Category::VolumePrefix {
                found_elements.push(Category::VolumeNumber, data_to_parse);
            }
            return true;
        }
        parse_single_subtoken(delimiter, data_to_parse, found_elements);
        return true;
    }
    false
}

// ── parse_episode_number ─────────────────────────────────────────────────────

/// Main episode number extraction: runs three passes (prefix, season+ep, bare digit) over all tokens.
pub(crate) fn parse_episode_number(
    delimiter: &[char],
    tokens_to_parse: &mut [Token],
    found_elements: &mut Elements,
) {
    for token in tokens_to_parse.iter_mut() {
        if !token.contains_unknow() {
            continue;
        }
        let (raw_tok, subtokens) = token.raw_and_subtokens();
        for subtoken_id in 0..subtokens.len() {
            if subtokens[subtoken_id].is_category(SubTokenCategory::Found) {
                continue;
            }
            let val = subtokens[subtoken_id].value();
            if val.is_empty() {
                subtokens[subtoken_id].category(SubTokenCategory::Found);
                continue;
            }
            if is_digit(val) || !contains_digit(val) {
                continue;
            }
            // Look-ahead: combine "EP07.5" or "EP01-04" from adjacent subtokens
            let (val_to_parse, skip_next) = {
                let base = subtokens[subtoken_id].value();
                if let Some(next_st) = subtokens.get(subtoken_id + 1) {
                    if is_digit(next_st.value()) && !next_st.is_category(SubTokenCategory::Found) {
                        let compound_dot = format!("{}.{}", base, next_st.value());
                        let compound_dash = format!("{}-{}", base, next_st.value());
                        if raw_tok.contains(&compound_dot) {
                            (compound_dot, true)
                        } else if raw_tok.contains(&compound_dash) {
                            (compound_dash, true)
                        } else {
                            (base.to_string(), false)
                        }
                    } else {
                        (base.to_string(), false)
                    }
                } else {
                    (base.to_string(), false)
                }
            };
            let ep_prefix_count_before = found_elements.count(Category::EpisodePrefix);
            if parse_single_subtoken(delimiter, &val_to_parse, found_elements) {
                let starts_with_alpha = val_to_parse
                    .bytes()
                    .next()
                    .is_some_and(|b| b.is_ascii_alphabetic());
                let ep_prefix_added =
                    found_elements.count(Category::EpisodePrefix) > ep_prefix_count_before;
                // For AnimeType/suppressed-EpisodePrefix compounds keep the alpha part
                // visible so find_anime_title can include it in the title.
                if !starts_with_alpha || ep_prefix_added {
                    subtokens[subtoken_id].category(SubTokenCategory::Found);
                }
                if skip_next {
                    subtokens[subtoken_id + 1].category(SubTokenCategory::Found);
                }
                // Handle "#N-M" range
                if !skip_next && val_to_parse.starts_with('#') {
                    let n1: i32 = val_to_parse[1..].parse().unwrap_or(-1);
                    let next1 = subtokens.get(subtoken_id + 1);
                    let (ep2_val, ep2_idx, dash_idx) = if let Some(n) = next1 {
                        if is_digit(n.value()) && !n.is_category(SubTokenCategory::Found) {
                            let rc = format!("{}-{}", val_to_parse, n.value());
                            if raw_tok.contains(&rc) {
                                (n.value().to_string(), subtoken_id + 1, None)
                            } else {
                                (String::new(), 0, None)
                            }
                        } else if n.value() == "-" && !n.is_category(SubTokenCategory::Found) {
                            if let Some(after) = subtokens.get(subtoken_id + 2) {
                                if is_digit(after.value())
                                    && !after.is_category(SubTokenCategory::Found)
                                {
                                    let rc = format!("{}-{}", val_to_parse, after.value());
                                    if raw_tok.contains(&rc) {
                                        (
                                            after.value().to_string(),
                                            subtoken_id + 2,
                                            Some(subtoken_id + 1),
                                        )
                                    } else {
                                        (String::new(), 0, None)
                                    }
                                } else {
                                    (String::new(), 0, None)
                                }
                            } else {
                                (String::new(), 0, None)
                            }
                        } else {
                            (String::new(), 0, None)
                        }
                    } else {
                        (String::new(), 0, None)
                    };
                    if !ep2_val.is_empty() {
                        let n2: i32 = ep2_val.parse().unwrap_or(-1);
                        if n1 >= 0 && n2 > n1 {
                            found_elements.push(Category::EpisodeNumber, &ep2_val);
                            subtokens[ep2_idx].category(SubTokenCategory::Found);
                            if let Some(di) = dash_idx {
                                subtokens[di].category(SubTokenCategory::Found);
                            }
                        }
                    }
                }
            }
        }
    }

    if found_elements.is_category_empty(Category::EpisodeNumber) {
        for token in tokens_to_parse.iter_mut() {
            if !token.contains_unknow() {
                continue;
            }
            let (raw_data, sub_tokens) = token.raw_and_subtokens();
            for index in 0..sub_tokens.len() {
                let tested_value = sub_tokens[index].value();
                if is_digit(tested_value) {
                    if let Some(next_value) = sub_tokens.get(index + 1) {
                        if is_digit(next_value.value()) {
                            let right = next_value.value().parse::<i32>().unwrap();
                            let left = tested_value.parse::<i32>().unwrap();
                            let fractal_pattern =
                                format!("{}.{}", tested_value, next_value.value());
                            if raw_data.contains(&fractal_pattern) {
                                let has_text_before = (0..index).any(|i| {
                                    sub_tokens[i].is_category(SubTokenCategory::Unknow)
                                        && !is_digit(sub_tokens[i].value())
                                });
                                let has_text_after =
                                    sub_tokens.get(index + 2..).is_some_and(|rest| {
                                        rest.iter().any(|s| {
                                            s.is_category(SubTokenCategory::Unknow)
                                                && !is_digit(s.value())
                                        })
                                    });
                                let preceded_by_dash = index > 0 && {
                                    let prev = &sub_tokens[index - 1];
                                    prev.is_category(SubTokenCategory::Unknow)
                                        && prev.value() == "-"
                                };
                                if !has_text_before || !has_text_after || preceded_by_dash {
                                    sub_tokens[index].category(SubTokenCategory::Found);
                                    sub_tokens[index + 1].category(SubTokenCategory::Found);
                                    found_elements.push(Category::EpisodeNumber, &fractal_pattern);
                                    return;
                                }
                                continue;
                            }
                            if left < right {
                                let lv = sub_tokens[index].value().to_string();
                                let rv = sub_tokens[index + 1].value().to_string();
                                sub_tokens[index].category(SubTokenCategory::Found);
                                sub_tokens[index + 1].category(SubTokenCategory::Found);
                                found_elements.push(Category::EpisodeNumber, &lv);
                                found_elements.push(Category::EpisodeNumber, &rv);
                                return;
                            }
                        }
                    }
                    if let Some(sub_token) = sub_tokens.get(index + 2) {
                        if is_digit(sub_token.value()) {
                            let middle = sub_tokens[index + 1].value();
                            let right = sub_token.value().parse::<i32>().unwrap();
                            let left = tested_value.parse::<i32>().unwrap();
                            let p_delimiter = middle.chars().next().unwrap();
                            if middle == "of" && left < right {
                                let lv = sub_tokens[index].value().to_string();
                                sub_tokens[index].category(SubTokenCategory::Found);
                                sub_tokens[index + 1].category(SubTokenCategory::Found);
                                sub_tokens[index + 2].category(SubTokenCategory::Found);
                                found_elements.push(Category::EpisodeNumber, &lv);
                                return;
                            }
                            if delimiter.contains(&p_delimiter) && middle.len() == 1 && left < right
                            {
                                let rv = sub_tokens[index + 2].value().to_string();
                                sub_tokens[index + 1].category(SubTokenCategory::Found);
                                sub_tokens[index + 2].category(SubTokenCategory::Found);
                                found_elements.push(Category::EpisodeNumber, &rv);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    if found_elements.is_category_empty(Category::EpisodeNumber) {
        for token_index in 0..tokens_to_parse.len() {
            if let Some(tmp_token) = tokens_to_parse.get_mut(token_index) {
                if !tmp_token.contains_unknow() {
                    continue;
                }
                let _ = tmp_token; // drop — re-read below with is_bracket first
            }
            // Re-approach: get is_bracket separately first
            let is_bracket = tokens_to_parse[token_index].is_inside_delimiter();
            let raw_token_owned = tokens_to_parse[token_index].raw_token().to_string();
            let sub_token = tokens_to_parse[token_index].sub_tokens();
            let mut subtoken_index = 0;
            while subtoken_index < sub_token.len() {
                if let Some(tested_subtoken) = sub_token.get(subtoken_index) {
                    subtoken_index += 1;
                    if tested_subtoken.is_category(SubTokenCategory::Found)
                        || !is_digit(tested_subtoken.value())
                    {
                        continue;
                    }
                    if let Some(next_token) = sub_token.get(subtoken_index) {
                        if is_digit(next_token.value())
                            && !next_token.is_category(SubTokenCategory::Found)
                        {
                            let fractal_pattern =
                                format!("{}.{}", tested_subtoken.value(), next_token.value());
                            if raw_token_owned.contains(&fractal_pattern) {
                                subtoken_index += 1;
                                continue;
                            }
                        }
                    }
                    if !is_bracket {
                        let pos = subtoken_index - 1;
                        let preceded_by_dash = pos > 0 && {
                            let prev = &sub_token[pos - 1];
                            prev.is_category(SubTokenCategory::Unknow) && prev.value() == "-"
                        };
                        if !preceded_by_dash {
                            let followed_by_dash_then_unknown =
                                sub_token.get(subtoken_index).is_some_and(|next| {
                                    next.is_category(SubTokenCategory::Unknow)
                                        && next.value() == "-"
                                        && sub_token.get(subtoken_index + 1..).is_some_and(|rest| {
                                            rest.iter().any(|s| {
                                                s.is_category(SubTokenCategory::Unknow)
                                                    && !is_digit(s.value())
                                            })
                                        })
                                });
                            if !followed_by_dash_then_unknown {
                                let has_text_before = pos > 0
                                    && sub_token[..pos].iter().any(|s| {
                                        s.is_category(SubTokenCategory::Unknow)
                                            && !is_digit(s.value())
                                    });
                                let has_text_after = sub_token[subtoken_index..].iter().any(|s| {
                                    s.is_category(SubTokenCategory::Unknow) && !is_digit(s.value())
                                });
                                if has_text_before && has_text_after {
                                    // Relax: if multiple structural keywords are found after
                                    // this number (structured dot-file), it IS an episode.
                                    // Require ≥2 to avoid false positives (e.g. "Title 3 - PV").
                                    // Also never treat anime years as episode numbers.
                                    let found_count_after = sub_token[subtoken_index..]
                                        .iter()
                                        .filter(|s| s.is_category(SubTokenCategory::Found))
                                        .count();
                                    let is_year = is_anime_year(tested_subtoken.value());
                                    if found_count_after < 2 || is_year {
                                        continue;
                                    }
                                }
                                if pos == 0 && has_text_after {
                                    continue;
                                }
                            }
                        }
                    }
                    if is_bracket {
                        let pos = subtoken_index - 1;
                        if pos > 0 {
                            let prev = &sub_token[pos - 1];
                            if !prev.is_category(SubTokenCategory::Found) {
                                let v = prev.value();
                                if v.chars().last().is_some_and(|c| c.is_alphabetic()) {
                                    continue;
                                }
                            }
                        }
                    }
                    if tested_subtoken.value().len() == 1 && subtoken_index >= 2 {
                        let prev = &sub_token[subtoken_index - 2];
                        if !is_digit(prev.value()) && prev.is_category(SubTokenCategory::Unknow) {
                            let compound = format!("{}.{}", prev.value(), tested_subtoken.value());
                            if raw_token_owned.contains(&compound) {
                                continue;
                            }
                        }
                    }
                    let ep_val = tested_subtoken.value().to_string();
                    sub_token[subtoken_index - 1].category(SubTokenCategory::Found);
                    found_elements.push(Category::EpisodeNumber, &ep_val);
                    return;
                }
            }
        }
    }
}

// ── detect_episode_number_alt ────────────────────────────────────────────────

/// Detects an alternative episode number (e.g. absolute number alongside a season number).
pub(crate) fn detect_episode_number_alt(
    tokens_to_parse: &mut [Token],
    found_elements: &mut Elements,
) {
    let ep_count = found_elements.count(Category::EpisodeNumber);
    if ep_count == 2 {
        if let Some(eps) = found_elements.find_all(Category::EpisodeNumber) {
            let v1 = eps[0].value.clone();
            let v2 = eps[1].value.clone();
            let n1: i32 = v1.parse().unwrap_or(-1);
            let n2: i32 = v2.parse().unwrap_or(-1);
            if n1 >= 0 && n2 >= 0 && n1 == n2 {
                found_elements.remove_first(Category::EpisodeNumber);
                found_elements.remove_first(Category::EpisodeNumber);
                let to_keep = if v1.len() <= v2.len() { v1 } else { v2 };
                found_elements.push(Category::EpisodeNumber, &to_keep);
                found_elements.remove_first(Category::EpisodePrefix);
            } else if n1 >= 0 && n2 >= 0 && !found_elements.is_category_empty(Category::AnimeSeason)
            {
                let (smaller, larger) = if n1 < n2 { (v1, v2) } else { (v2, v1) };
                found_elements.remove_first(Category::EpisodeNumber);
                found_elements.remove_first(Category::EpisodeNumber);
                found_elements.push(Category::EpisodeNumber, &smaller);
                found_elements.push(Category::EpisodeNumberAlt, &larger);
                found_elements.remove_first(Category::EpisodePrefix);
            }
        }
        return;
    }
    if ep_count != 1 {
        return;
    }
    for token in tokens_to_parse.iter_mut() {
        if !token.is_inside_delimiter() || !token.is_paren() || !token.contains_unknow() {
            continue;
        }
        let sub_tokens = token.sub_tokens();
        if sub_tokens.len() != 1 || !sub_tokens[0].is_category(SubTokenCategory::Unknow) {
            continue;
        }
        let val = sub_tokens[0].value();
        if !is_digit(val) {
            continue;
        }
        let paren_num: i32 = val.parse().unwrap_or(0);
        if paren_num == 0 {
            continue;
        }
        if let Some(ep_elem) = found_elements.find(Category::EpisodeNumber) {
            let ep_num: i32 = ep_elem.value.parse().unwrap_or(0);
            if paren_num == ep_num {
                continue;
            }
            if paren_num < ep_num {
                if val.len() != ep_elem.value.len() {
                    continue;
                }
                let old_ep = found_elements
                    .remove_first(Category::EpisodeNumber)
                    .unwrap();
                found_elements.push(Category::EpisodeNumber, val);
                found_elements.push(Category::EpisodeNumberAlt, &old_ep);
                sub_tokens[0].category(SubTokenCategory::Found);
            } else {
                found_elements.push(Category::EpisodeNumberAlt, val);
                sub_tokens[0].category(SubTokenCategory::Found);
            }
        }
    }
}
