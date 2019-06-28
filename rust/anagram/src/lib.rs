use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_map = build_char_map(&word);

    possible_anagrams
        .iter()
        .cloned()
        .filter(|&s| is_anagram(&word, &word_map, &s.to_lowercase()))
        .collect()
}

fn build_char_map(word: &str) -> HashMap<char, usize> {
    let mut char_map = HashMap::new();

    word.chars()
        .for_each(|c| {
            *char_map.entry(c).or_insert(0) += 1;
        });
    char_map
}

fn is_anagram(word: &str, word_map: &HashMap<char, usize>, candidate: &str) -> bool {
    let candidate_map = build_char_map(candidate);
    word != candidate && *word_map == candidate_map
}

