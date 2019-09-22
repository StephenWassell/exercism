/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = sentence
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .collect::<Vec<_>>();

    letters.sort_unstable();
    letters.dedup();

    let alphabet = (b'a'..=b'z').map(|b| b as char).collect::<Vec<_>>();

    letters == alphabet
}
