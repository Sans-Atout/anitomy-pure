//! Anime title, release group and episode title extraction.
use crate::{
    elements::{Category, Elements},
    keyword::Manager,
    token::{main_token::Token, subtoken::SubTokenCategory},
};

/// Extracts the anime title from the token list and pushes it into `found_elements`.
pub(crate) fn parse_anime_title(tokens: &mut [Token], found_elements: &mut Elements, d: &[char]) {
    // Pre-pass: a japanese-corner bracket (「」) is itself the complete title.
    for i in 0..tokens.len() {
        if tokens[i].is_japanese_corner() && tokens[i].is_inside_delimiter() && tokens[i].contains_unknow() {
            let title = {
                let (raw, sub_tokens) = tokens[i].raw_and_subtokens();
                let t = format!("「{}」", raw);
                for st in sub_tokens.iter_mut() {
                    st.category(SubTokenCategory::Found);
                }
                t
            };
            for j in (i + 1)..tokens.len() {
                if !tokens[j].is_inside_delimiter() {
                    for st in tokens[j].sub_tokens().iter_mut() {
                        st.category(SubTokenCategory::Found);
                    }
                }
            }
            found_elements.push(Category::AnimeTitle, &title);
            return;
        }
    }

    let mut tmp_index = 0;
    let mut anime_title = String::new();

    while tmp_index < tokens.len() {
        if tokens[tmp_index].contains_unknow() && !tokens[tmp_index].is_inside_delimiter() {
            let has_alpha = tokens[tmp_index].raw_token().chars().any(|c| c.is_alphanumeric());
            if !has_alpha {
                tmp_index += 1;
                continue;
            }
            {
                let digit_unknowns: Vec<_> = tokens[tmp_index]
                    .sub_tokens()
                    .iter()
                    .filter(|s| s.is_category(SubTokenCategory::Unknow) && !s.value().is_empty())
                    .collect();
                if digit_unknowns.len() == 1
                    && digit_unknowns[0].value().chars().all(|c| c.is_ascii_digit())
                {
                    for st in tokens[tmp_index].sub_tokens().iter_mut() {
                        if st.is_category(SubTokenCategory::Unknow) && !st.value().is_empty() {
                            st.category(SubTokenCategory::Found);
                        }
                    }
                    tmp_index += 1;
                    continue;
                }
            }
            if !found_elements.is_category_empty(Category::EpisodeNumber) {
                let first_unknown_pos = tokens[tmp_index]
                    .sub_tokens()
                    .iter()
                    .position(|s| s.is_category(SubTokenCategory::Unknow));
                if let Some(pos) = first_unknown_pos {
                    if pos > 0 {
                        tmp_index += 1;
                        continue;
                    }
                }
            }
            anime_title = find_anime_title(tokens, tmp_index, d, found_elements);
            found_elements.push(Category::AnimeTitle, &anime_title);
            return;
        }
        tmp_index += 1;
    }

    if anime_title.is_empty() {
        let mut first_idx: Option<usize> = None;
        let mut second_idx: Option<usize> = None;
        for i in 0..tokens.len() {
            if tokens[i].contains_unknow() && tokens[i].is_inside_delimiter() {
                if first_idx.is_none() {
                    first_idx = Some(i);
                } else if second_idx.is_none() {
                    second_idx = Some(i);
                    break;
                }
            }
        }
        let chosen = match (first_idx, second_idx) {
            (Some(f), Some(s)) => {
                let first_count = tokens[f]
                    .sub_tokens()
                    .iter()
                    .filter(|st| st.is_category(SubTokenCategory::Unknow))
                    .count();
                let second_count = tokens[s]
                    .sub_tokens()
                    .iter()
                    .filter(|st| st.is_category(SubTokenCategory::Unknow))
                    .count();
                if first_count > second_count { Some(f) } else { Some(s) }
            }
            _ => None,
        };
        if let Some(idx) = chosen {
            anime_title = find_anime_title(tokens, idx, d, found_elements);
            found_elements.push(Category::AnimeTitle, &anime_title);
        }
    }
}

fn find_anime_title(tokens: &mut [Token], starting_index: usize, d: &[char], found_elements: &Elements) -> String {
    let mut token_index = starting_index;
    let mut anime_title = String::new();

    // ── First token (starting_index) ──────────────────────────────────────────
    let mut unknow_start = 0;
    {
        let (raw_token, subtoken) = tokens[token_index].raw_and_subtokens();

        while subtoken[unknow_start].is_category(SubTokenCategory::Found) {
            unknow_start += 1;
        }

        let mut tmp_index = unknow_start;
        while tmp_index < subtoken.len() {
            if subtoken[tmp_index].is_category(SubTokenCategory::Found) {
                return anime_title.trim_matches(d).to_owned();
            }

            let current_val = subtoken[tmp_index].value();

            let next_is_found = tmp_index + 1 < subtoken.len()
                && subtoken[tmp_index + 1].is_category(SubTokenCategory::Found);
            if next_is_found && current_val.chars().all(|c: char| !c.is_alphanumeric()) {
                subtoken[tmp_index].category(SubTokenCategory::Found);
                tmp_index += 1;
                continue;
            }

            if d.contains(&'.') && !anime_title.is_empty() && current_val.len() <= 2 {
                let last_space_word = anime_title.trim_start().rsplit(' ').next().unwrap_or("");
                let prev_last_seg = last_space_word.rsplit('.').next().unwrap_or(last_space_word);
                if prev_last_seg.len() <= 2 && !prev_last_seg.is_empty() {
                    let mut dot_form = String::with_capacity(last_space_word.len() + 1 + current_val.len());
                    dot_form.push_str(last_space_word);
                    dot_form.push('.');
                    dot_form.push_str(current_val);
                    if raw_token.contains(dot_form.as_str()) {
                        anime_title.push('.');
                        anime_title.push_str(current_val);
                        subtoken[tmp_index].category(SubTokenCategory::Found);
                        tmp_index += 1;
                        continue;
                    }
                }
            }

            if d.contains(&'-') && !anime_title.is_empty() {
                let last_word = anime_title.trim_start().rsplit(' ').next().unwrap_or("");
                if !last_word.is_empty() {
                    let both_single_digits = last_word.len() == 1
                        && current_val.len() == 1
                        && last_word.chars().all(|c| c.is_ascii_digit())
                        && current_val.chars().all(|c| c.is_ascii_digit());
                    let next_is_found2 = tmp_index + 1 < subtoken.len()
                        && subtoken[tmp_index + 1].is_category(SubTokenCategory::Found);
                    let both_alpha_words = last_word.len() >= 2
                        && current_val.len() >= 2
                        && last_word.chars().all(|c| c.is_alphabetic())
                        && current_val.chars().all(|c| c.is_alphabetic())
                        && last_word.chars().next().map_or(false, |c| c.is_uppercase())
                        && current_val.chars().next().map_or(false, |c| c.is_uppercase())
                        && next_is_found2;
                    if both_single_digits || both_alpha_words {
                        let mut dash_form = String::with_capacity(last_word.len() + 1 + current_val.len());
                        dash_form.push_str(last_word);
                        dash_form.push('-');
                        dash_form.push_str(current_val);
                        if raw_token.contains(dash_form.as_str()) {
                            anime_title.push('-');
                            anime_title.push_str(current_val);
                            subtoken[tmp_index].category(SubTokenCategory::Found);
                            tmp_index += 1;
                            continue;
                        }
                    }
                }
            }

            // Handle alpha-prefix + digit-suffix compounds like "EX01", "OVA3.5", "ED2", "OP4a".
            // Consult the keyword manager to decide: include alpha in title, or skip entirely.
            if !current_val.is_empty() {
                let alpha_len = current_val.bytes().take_while(|b| b.is_ascii_alphabetic()).count();
                if alpha_len > 0 && alpha_len < current_val.len() {
                    let alpha_prefix = &current_val[..alpha_len];
                    let digit_suffix = &current_val[alpha_len..];
                    if let Some(kw) = super::KEYWORD_MANAGER.find(alpha_prefix) {
                        let cat = kw.get_category();
                        if (cat == Category::EpisodePrefix && !kw.is_valid())
                            || (cat == Category::AnimeType && kw.is_valid() && kw.is_searchable())
                        {
                            // Include alpha prefix in title: EX01 → "EX", OVA3.5 → "OVA".
                            let is_ep_suffix = !digit_suffix.is_empty()
                                && digit_suffix.bytes().any(|b| b.is_ascii_digit())
                                && digit_suffix.bytes().all(|b| b.is_ascii_digit() || b == b'.');
                            if is_ep_suffix
                                && !found_elements.is_category_empty(Category::EpisodeNumber)
                            {
                                let ep_matches = found_elements
                                    .find(Category::EpisodeNumber)
                                    .map_or(false, |ep| {
                                        ep.value == digit_suffix
                                            || ep.value
                                                .split('.')
                                                .next()
                                                .map_or(false, |n| n == digit_suffix)
                                    });
                                if ep_matches {
                                    let prefix_owned = alpha_prefix.to_owned();
                                    anime_title.push(' ');
                                    anime_title.push_str(&prefix_owned);
                                    subtoken[tmp_index].category(SubTokenCategory::Found);
                                    tmp_index += 1;
                                    continue;
                                }
                            }
                            // Digit doesn't match or not clean format — fall through to default.
                        } else {
                            // EP(valid)/AnimeSeasonPrefix/AnimeType(!title-worthy): end of title.
                            subtoken[tmp_index].category(SubTokenCategory::Found);
                            return anime_title.trim_matches(d).to_owned();
                        }
                    }
                }
            }

            anime_title.push(' ');
            anime_title.push_str(current_val);
            subtoken[tmp_index].category(SubTokenCategory::Found);
            tmp_index += 1;
        }
        // raw_token and subtoken borrows end here (block ends)
    }

    token_index += 1;

    // ── Subsequent tokens ─────────────────────────────────────────────────────
    while token_index < tokens.len() {
        let is_weak = tokens[token_index].is_weak();
        let is_inside = tokens[token_index].is_inside_delimiter();
        let is_embedded_bracket = is_inside
            && !is_weak
            && token_index + 1 < tokens.len()
            && !tokens[token_index + 1].is_inside_delimiter()
            && tokens[token_index + 1].contains_unknow();
        if !tokens[token_index].contains_unknow()
            || (is_inside && !is_weak && !is_embedded_bracket)
        {
            return anime_title.trim_matches(d).to_owned();
        }

        // Get last char of previous token without Vec<char> allocation
        let last_char: char = tokens[token_index - 1]
            .raw_token()
            .chars()
            .next_back()
            .unwrap_or(' ');

        let cur_token_starts_with_dash: bool = {
            let cur_raw = tokens[token_index].raw_token();
            let stripped = cur_raw.trim_start_matches(' ');
            d.contains(&'-') && stripped.starts_with("- ")
        };

        let tmp_subtokens = tokens[token_index].sub_tokens();
        if tmp_subtokens[0].is_category(SubTokenCategory::Found) {
            return anime_title.trim_matches(d).to_owned();
        }

        if is_weak {
            if d.contains(&last_char) {
                anime_title.push(' ');
                anime_title.push('(');
            } else {
                anime_title.push('(');
            }
            anime_title.push_str(tmp_subtokens[0].value());
        } else if is_embedded_bracket {
            if d.contains(&last_char) {
                anime_title.push(' ');
                anime_title.push('[');
            } else {
                anime_title.push('[');
            }
            anime_title.push_str(tmp_subtokens[0].value());
        } else if cur_token_starts_with_dash {
            anime_title.push_str(" - ");
            anime_title.push_str(tmp_subtokens[0].value());
        } else {
            anime_title.push(' ');
            anime_title.push_str(tmp_subtokens[0].value());
        }
        tmp_subtokens[0].category(SubTokenCategory::Found);

        for tmp_st in tmp_subtokens.iter_mut().skip(1) {
            if tmp_st.is_category(SubTokenCategory::Found) {
                return anime_title.trim_matches(d).to_owned();
            }
            anime_title.push(' ');
            anime_title.push_str(tmp_st.value());
            tmp_st.category(SubTokenCategory::Found);
        }

        if is_weak {
            anime_title.push(')');
        } else if is_embedded_bracket {
            anime_title.push(']');
        }
        token_index += 1;
    }

    anime_title.trim_matches(d).to_owned()
}

/// Extracts the release group from bracketed tokens, falling back to a `codec-GROUP` heuristic.
pub(crate) fn parse_release_group(tokens: &mut [Token], found_elements: &mut Elements, d: &[char]) {
    if !found_elements.is_category_empty(Category::ReleaseGroup) {
        return;
    }
    let n = tokens.len();
    let mut rg_index: Option<usize> = None;
    let mut rg_has_before = false;
    let mut rg_has_raw_alpha_before = false;
    for i in 0..n {
        if !tokens[i].is_inside_delimiter() || tokens[i].is_weak() || tokens[i].is_japanese_corner() {
            continue;
        }
        if !tokens[i].is_full_unknow() && !tokens[i].contains_unknow() {
            continue;
        }
        let has_before = (0..i).any(|j| {
            !tokens[j].is_inside_delimiter()
                && tokens[j].contains_unknow()
                && tokens[j].raw_token().chars().any(|c| c.is_alphabetic())
        });
        let has_raw_alphabetic_before = (0..i).any(|j| {
            !tokens[j].is_inside_delimiter()
                && tokens[j].raw_token().chars().any(|c| c.is_alphabetic())
        });
        let has_after = ((i + 1)..n).any(|j| !tokens[j].is_inside_delimiter() && tokens[j].contains_unknow());
        if has_before && has_after {
            continue;
        }
        rg_index = Some(i);
        rg_has_before = has_before;
        rg_has_raw_alpha_before = has_raw_alphabetic_before;
        break;
    }
    if let Some(i) = rg_index {
        if tokens[i].is_full_unknow() {
            let group = {
                let sub_tokens = tokens[i].sub_tokens();
                let by_pos = sub_tokens.iter().position(|s| s.value().to_uppercase() == "BY");
                if let Some(p) = by_pos {
                    if p > 0 && p + 1 < sub_tokens.len() {
                        let mut g = String::new();
                        for (k, s) in sub_tokens[p + 1..].iter().enumerate() {
                            if k > 0 { g.push(' '); }
                            g.push_str(s.value());
                        }
                        for st in sub_tokens.iter_mut() {
                            st.category(SubTokenCategory::Found);
                        }
                        Some(g)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            if let Some(g) = group {
                found_elements.push(Category::ReleaseGroup, &g);
                return;
            }
            // No "BY" found — use the whole raw token
            let (raw, sub_tokens) = tokens[i].raw_and_subtokens();
            let raw_owned = raw.to_string();
            for st in sub_tokens.iter_mut() {
                st.category(SubTokenCategory::Found);
            }
            found_elements.push(Category::ReleaseGroup, &raw_owned);
        } else if !rg_has_before || tokens[i].is_paren() {
            if tokens[i].is_paren() && rg_has_raw_alpha_before || !rg_has_raw_alpha_before {
                parse_particular_string_subtoken(&mut tokens[i], found_elements, Category::ReleaseGroup, d);
            }
        }
    }

    // Fallback for non-bracketed release groups: in dot-separated filenames the release
    // group is the last segment after a hyphen following metadata keywords (e.g. "x264-ESiR").
    // Conditions: last Unknown purely-alphabetic subtoken (≥2 chars) preceded by a Found
    // subtoken, nothing Unknown after it, AND the raw filename contains "-{group}" (hyphen
    // separator — distinguishes "x264-ESiR" from "x264.DHD" which is NOT a release group).
    if found_elements.is_category_empty(Category::ReleaseGroup) {
        for token in tokens.iter_mut() {
            if token.is_inside_delimiter() || token.is_weak() || token.is_japanese_corner() {
                continue;
            }
            let (raw, sub_tokens) = token.raw_and_subtokens();
            let candidate = sub_tokens.iter().enumerate().rev().find(|(_, s)| {
                s.is_category(SubTokenCategory::Unknow)
                    && s.value().len() >= 2
                    && s.value().chars().all(|c| c.is_alphabetic())
            });
            if let Some((idx, _)) = candidate {
                let prev_is_found = idx > 0 && sub_tokens[idx - 1].is_category(SubTokenCategory::Found);
                if !prev_is_found {
                    continue;
                }
                let all_after_found = sub_tokens[idx + 1..]
                    .iter()
                    .all(|s| s.is_category(SubTokenCategory::Found) || s.value().is_empty());
                if !all_after_found {
                    continue;
                }
                let group = sub_tokens[idx].value().to_string();
                // Must be preceded by '-' in the raw filename (the "codec-GROUP" pattern).
                if !raw.contains(&format!("-{}", group)) {
                    continue;
                }
                sub_tokens[idx].category(SubTokenCategory::Found);
                found_elements.push(Category::ReleaseGroup, &group);
                break;
            }
        }
    }
}

/// Extracts the episode title from the first remaining unclassified non-bracketed token.
pub(crate) fn parse_episode_title(tokens: &mut [Token], found_elements: &mut Elements, d: &[char]) {
    for token in tokens.iter_mut() {
        if token.contains_unknow() && !token.is_inside_delimiter() {
            let has_alpha = token.raw_token().chars().any(|c| c.is_alphanumeric());
            if !has_alpha {
                continue;
            }
            parse_particular_string_subtoken(token, found_elements, Category::EpisodeTitle, d);
            return;
        }
    }
}

/// Collects the unclassified subtokens of `token` into a single string and pushes it as category `c`.
pub(crate) fn parse_particular_string_subtoken(
    token: &mut Token,
    e: &mut Elements,
    c: Category,
    d: &[char],
) {
    let all_subtoken = token.sub_tokens();
    let mut sub_token_id = 0;
    let mut string_to_categorise = String::new();
    while all_subtoken[sub_token_id].is_category(SubTokenCategory::Found) {
        sub_token_id += 1;
    }

    if c == Category::EpisodeTitle {
        while sub_token_id < all_subtoken.len()
            && !all_subtoken[sub_token_id].is_category(SubTokenCategory::Found)
            && !all_subtoken[sub_token_id].value().is_empty()
            && !all_subtoken[sub_token_id].value().chars().any(|ch| ch.is_alphanumeric())
        {
            all_subtoken[sub_token_id].category(SubTokenCategory::Found);
            sub_token_id += 1;
        }
    }

    while sub_token_id < all_subtoken.len() {
        if all_subtoken[sub_token_id].is_category(SubTokenCategory::Found) {
            let trimmed = string_to_categorise.trim_matches(d);
            if !trimmed.is_empty() {
                e.push(c, trimmed);
            }
            string_to_categorise.clear();
            break;
        }
        string_to_categorise.push(' ');
        string_to_categorise.push_str(all_subtoken[sub_token_id].value());
        all_subtoken[sub_token_id].category(SubTokenCategory::Found);
        sub_token_id += 1;
    }
    if !string_to_categorise.is_empty() {
        e.push(c, string_to_categorise.trim_matches(d));
    }
}

/// Tries to match a two-part keyword (`left` + `right`) and pushes the result into `e`; returns `true` on match.
pub(crate) fn parse_multiple_keyword(
    e: &mut Elements,
    keyword_manager: &Manager,
    left: &str,
    right: &str,
) -> bool {
    let mut buf = String::with_capacity(left.len() + right.len() + 1);
    for sep in ['.', ' ', '-', '_'] {
        buf.clear();
        buf.push_str(left);
        buf.push(sep);
        buf.push_str(right);
        let trimmed = buf.trim();
        if let Some(found) = keyword_manager.find(trimmed) {
            let category = found.get_category();
            let ok = category.is_searchable()
                && !(category.is_singular() && !e.is_category_empty(category));
            if ok {
                e.push(category, trimmed);
                return true;
            }
        }
    }
    false
}
