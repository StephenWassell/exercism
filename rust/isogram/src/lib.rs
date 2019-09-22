fn matching_pair(pair: &[char]) -> bool {
    match pair {
        [a, b] => a == b,
        _ => false,
    }
}

pub fn check(candidate: &str) -> bool {
    let mut letters = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    letters.sort_unstable();

    !letters.windows(2).any(matching_pair)
}
