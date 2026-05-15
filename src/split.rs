//! Filename and token splitting utilities.
use crate::token::main_token::Token;

/// Splits a compound string like `"OVA3"` into its alpha prefix and digit suffix.
pub(crate) fn split_type_and_ep(to_parse: &str) -> (&str, &str) {
    let split_at = to_parse
        .char_indices()
        .find(|(_, c)| c.is_ascii_digit())
        .map_or(to_parse.len(), |(i, _)| i);
    (&to_parse[..split_at], &to_parse[split_at..])
}

/// Splits a filename string into [`Token`]s, respecting bracket pairs (including Japanese brackets).
pub(crate) fn split_raw_data(raw_data: &str, delimiter: &[char]) -> Vec<Token> {
    let mut raw_tokens: Vec<Token> = Vec::default();
    let mut segment_start = 0;
    let mut segment_end = 0;

    for (i, c) in raw_data.char_indices() {
        let is_open = matches!(
            c,
            '[' | '(' | '{' | '\u{300C}' | '\u{300E}' | '\u{3010}' | '\u{FF08}'
        );
        let is_close = matches!(
            c,
            ']' | ')' | '}' | '\u{300D}' | '\u{300F}' | '\u{3011}' | '\u{FF09}'
        );

        if is_open {
            if segment_end > segment_start {
                raw_tokens.push(Token::new(
                    &raw_data[segment_start..segment_end],
                    delimiter,
                    false,
                    false,
                    false,
                ));
            }
            segment_start = i + c.len_utf8();
            segment_end = segment_start;
        } else if is_close {
            if segment_end > segment_start {
                let paren = matches!(c, ')' | '\u{FF09}');
                let corner = matches!(c, '\u{300D}' | '\u{300F}');
                raw_tokens.push(Token::new(
                    &raw_data[segment_start..segment_end],
                    delimiter,
                    true,
                    paren,
                    corner,
                ));
            }
            segment_start = i + c.len_utf8();
            segment_end = segment_start;
        } else {
            segment_end = i + c.len_utf8();
        }
    }
    if segment_end > segment_start {
        raw_tokens.push(Token::new(
            &raw_data[segment_start..segment_end],
            delimiter,
            false,
            false,
            false,
        ));
    }
    raw_tokens
}

/// Splits a raw token string into subtoken strings on the given delimiters.
pub(crate) fn split_token(raw_token: &str, delimiter: &[char]) -> Vec<String> {
    let trimmed = raw_token.trim_matches(delimiter);
    let mut tokenized: Vec<String> = Vec::default();
    let mut seg_start = 0;
    let chars: Vec<(usize, char)> = trimmed.char_indices().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let (byte_i, c) = chars[i];
        if delimiter.contains(&c) {
            if byte_i > seg_start {
                tokenized.push(trimmed[seg_start..byte_i].to_string());
            }
            // Look-ahead: two consecutive delimiters → emit the first as a subtoken
            if i + 2 < len
                && delimiter.contains(&chars[i + 1].1)
                && delimiter.contains(&chars[i + 2].1)
            {
                let (deli_byte, deli_char) = chars[i + 1];
                let deli_end = deli_byte + deli_char.len_utf8();
                tokenized.push(trimmed[deli_byte..deli_end].to_string());
                i += 2;
            } else {
                i += 1;
            }
            let next_byte = if i < len { chars[i].0 } else { trimmed.len() };
            seg_start = next_byte;
        } else {
            i += 1;
        }
    }
    if seg_start < trimmed.len() {
        tokenized.push(trimmed[seg_start..].to_string());
    }
    tokenized
}

#[cfg(test)]
mod tests {
    use super::{split_raw_data, split_token};
    use crate::token::main_token::Token;
    use pretty_assertions::assert_eq;

    #[test]
    fn split_regex() {
        let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
        let s = "hello_world I.m&a+beautifull,rust|test";
        let e: Vec<&str> = vec![
            "hello",
            "world",
            "I",
            "m",
            "a",
            "beautifull",
            "rust",
            "test",
        ];
        assert_eq!(e, split_token(s, &d))
    }

    #[test]
    fn non_normal_split() {
        let d: &[char; 8] = &[' ', '_', '.', '-', '&', '+', ',', '|'];
        let tested = split_raw_data(
            "[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv",
            d,
        );
        let mut wanted = vec![
            Token::new("TaigaSubs", d, true, false, false),
            Token::new("_Toradora!_", d, false, false, false),
            Token::new("2008", d, true, true, false),
            Token::new("_-_01v2_-_Tiger_and_Dragon_", d, false, false, false),
            Token::new("1280x720_H.264_FLAC", d, true, false, false),
            Token::new("1234ABCD", d, true, false, false),
        ];
        assert_ne!(wanted, tested);
        wanted.push(Token::new(".mkv", d, false, false, false));
        assert_eq!(wanted, tested);
    }

    #[test]
    fn normal_split() {
        let d: &[char; 8] = &[' ', '_', '.', '-', '&', '+', ',', '|'];
        let tested = split_raw_data(
            "[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD]",
            d,
        );
        let wanted = vec![
            Token::new("TaigaSubs", d, true, false, false),
            Token::new("_Toradora!_", d, false, false, false),
            Token::new("2008", d, true, true, false),
            Token::new("_-_01v2_-_Tiger_and_Dragon_", d, false, false, false),
            Token::new("1280x720_H.264_FLAC", d, true, false, false),
            Token::new("1234ABCD", d, true, false, false),
        ];
        assert_eq!(wanted, tested);
    }

    #[test]
    fn test_split_sub_token() {
        let tested = split_token(
            "_-_01v2_-_Tiger_and_Dragon_",
            &[' ', '_', '.', '-', '&', '+', ',', '|'],
        );
        let wanted = vec!["01v2", "-", "Tiger", "and", "Dragon"];
        assert_eq!(wanted, tested);
    }
}
