//! String pre-processing utilities applied before tokenisation.

/// Removes all strings in `ignored_str` from `working_string`, also handling Python-list syntax.
pub(crate) fn remove_ignored_string(working_string: &str, ignored_str: &[String]) -> String {
    let mut return_string = working_string.to_string();
    for i_s in ignored_str {
        // Support Python-list format like "['foo', 'bar']" — extract each quoted item
        if (i_s.starts_with("['") || i_s.starts_with("[\"")) && i_s.ends_with(']') {
            let mut in_str = false;
            let mut current = String::new();
            for c in i_s.chars() {
                match c {
                    '\'' | '"' => {
                        if in_str {
                            return_string = return_string.replace(&current, "");
                            current.clear();
                        }
                        in_str = !in_str;
                    }
                    _ if in_str => current.push(c),
                    _ => {}
                }
            }
        } else {
            return_string = return_string.replace(i_s.as_str(), "");
        }
    }
    return_string
}

#[cfg(test)]
mod tests {
    use super::remove_ignored_string;
    use pretty_assertions::assert_eq;

    #[test]
    fn remove_one_string() {
        let tested_string = "Hello World!";
        let r1 = remove_ignored_string(tested_string, &["World".to_string()]);
        assert_eq!(r1, "Hello !");
    }

    #[test]
    fn remove_multiple() {
        let tested_string = "Hello World!";
        let r2 = remove_ignored_string(tested_string, &["World".to_string(), "Hello".to_string()]);
        assert_eq!(r2, " !");
    }

    #[test]
    fn nothing_to_remove() {
        let tested_string = "EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv";
        let r2 = remove_ignored_string(tested_string, &["['41EvoBot.']".to_string()]);
        assert_eq!(r2, tested_string);
    }

    #[test]
    fn real_test_remove() {
        let tested_string = "EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv";
        let r2 = remove_ignored_string(tested_string, &["EvoBot.".to_string()]);
        assert_eq!(
            r2,
            "[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv"
        );
    }
}
