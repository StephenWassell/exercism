use std::collections::HashSet;

fn sorted_letters(s: &str) -> Vec<char> {
    let mut letters = s.chars().collect::<Vec<_>>();
    letters.sort_unstable();
    letters
}

fn is_anagram(candidate: &str, word: &str, letters: &[char]) -> bool {
    word != candidate && *letters == *sorted_letters(candidate)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let letters = sorted_letters(&word);

    possible_anagrams
        .iter()
        .filter(|s| is_anagram(&s.to_lowercase(), &word, &letters))
        .copied()
        .collect::<HashSet<&'a str>>()
}
