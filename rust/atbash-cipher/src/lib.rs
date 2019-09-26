use std::iter;

/// encode or decode a single char
fn coded_char(c: char) -> Option<char> {
    match c {
        'A'..='Z' => Some((b'a' + b'Z' - c as u8) as char),
        'a'..='z' => Some((b'a' + b'z' - c as u8) as char),
        '0'..='9' => Some(c),
        _ => None,
    }
}

/// return a space if needed at this point in the encoded string
fn space_or_none(i: usize) -> Option<char> {
    if i != 0 && i % 5 == 0 {
        Some(' ')
    } else {
        None
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    // the encoding is reversible,
    // but we need add spaces between groups of 5 chars
    decode(plain)
        .chars()
        .enumerate()
        .flat_map(|(i, c)| space_or_none(i).into_iter().chain(iter::once(c)))
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(coded_char)
        .collect()
}
