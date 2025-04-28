use std::collections::HashMap;

use itertools::Itertools;

pub fn word_count(words: &str) -> HashMap<String, usize> {
    words
        .to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '\'')
        .filter_map(|s| {
            let s = s.trim_matches('\'').to_string();

            (!s.is_empty()).then_some(s)
        })
        .counts()
}
