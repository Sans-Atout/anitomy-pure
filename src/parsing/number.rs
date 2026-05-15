//! Numeric pattern recognition helpers used by the parsing passes.

const ANIME_YEAR_MIN: i32 = 1917;
const ANIME_YEAR_MAX: i32 = 2050;

/// Returns `true` if `s` is exactly 8 hexadecimal characters (CRC32 format).
pub(crate) fn is_crc32(s: &str) -> bool {
    s.len() == 8 && is_hexa(s)
}

/// Returns `true` if `s` looks like a video resolution (`1080p`, `720i`, `1920x1080`, `4K`…).
pub(crate) fn is_resolution(s: &str) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return false;
    }
    // "2K" / "4K" / "8K"
    if n == 2 && matches!(b[0], b'2' | b'4' | b'8') && (b[1] == b'K' || b[1] == b'k') {
        return true;
    }
    // ends with p/P/i/I: 3–4 digits before
    if matches!(b[n - 1], b'p' | b'P' | b'i' | b'I') {
        let num_len = b[..n - 1]
            .iter()
            .rev()
            .take_while(|c| c.is_ascii_digit())
            .count();
        return matches!(num_len, 3 | 4) && num_len == n - 1;
    }
    // WxH or W×H: 3–4 digits, separator, 3–4 digits
    if let Some(sep) = s.find(['x', 'X', '×']) {
        let sep_char = s[sep..].chars().next().unwrap();
        let left = &s[..sep];
        let right = &s[sep + sep_char.len_utf8()..];
        let l_ok = matches!(left.len(), 3 | 4) && left.bytes().all(|c| c.is_ascii_digit());
        let r_ok = matches!(right.len(), 3 | 4) && right.bytes().all(|c| c.is_ascii_digit());
        return l_ok && r_ok;
    }
    false
}

/// Returns `true` if `s` parses as a plausible anime production year (1917–2050).
pub(crate) fn is_anime_year(s: &str) -> bool {
    match s.parse::<i32>() {
        Ok(y) => (ANIME_YEAR_MIN..=ANIME_YEAR_MAX).contains(&y),
        Err(_) => false,
    }
}

/// Returns `true` if every byte of `s` is a valid hexadecimal digit.
pub(crate) fn is_hexa(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b: u8| b.is_ascii_hexdigit())
}

/// Returns `true` if `s` is a non-empty string of ASCII digits (optional leading `+`/`-`).
pub(crate) fn is_digit(s: &str) -> bool {
    let s = s.trim_start_matches(['+', '-']);
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

/// Returns `true` if `s` contains at least one numeric character.
pub(crate) fn contains_digit(s: &str) -> bool {
    s.chars().any(|c| c.is_numeric())
}

/// Converts English ordinals (`"1st"`, `"first"`, …) to their digit string; returns `""` otherwise.
pub(crate) fn ordinals_to_nb(ordinal: &str) -> &str {
    match ordinal.to_lowercase().as_str() {
        "1st" | "first" => "1",
        "2nd" | "second" => "2",
        "3rd" | "third" => "3",
        "4th" | "fourth" => "4",
        "5th" | "fifth" => "5",
        "6th" | "sixth" => "6",
        "7th" | "seventh" => "7",
        "8th" | "eighth" => "8",
        "9th" | "ninth" => "9",
        _ => "",
    }
}
