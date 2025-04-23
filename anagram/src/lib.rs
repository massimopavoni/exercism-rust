use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_lcs = letter_counts(&word);

    possible_anagrams
        .iter()
        .filter(|anagram| {
            if anagram.len() != word.len() {
                return false;
            }
            let anagram = anagram.to_lowercase();
            anagram != word && letter_counts(&anagram) == word_lcs
        })
        .copied()
        .collect()
}

// Checking equality between letter counts is much more efficient than sorting for long strings.
fn letter_counts(word: &str) -> HashMap<char, usize> {
    word.chars().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    })
}
