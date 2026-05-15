//! String pre-processing utilities applied before tokenisation.

/// Removes all strings in `ignored_str` from `working_string`, also handling Python-list syntax.
pub fn remove_ignored_string(working_string: &str, ignored_str: &[String]) -> String {
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
