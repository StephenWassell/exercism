/// encode or decode a single char
fn coded_char(c: char) -> Option<char> {
    const A: u8 = 'a' as u8;
    const Z: u8 = 'z' as u8;

    match c {
        'a'...'z' => Some((A + Z - c as u8) as char),
        '0'...'9' => Some(c),
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
        .flat_map(|(i, c)| vec![space_or_none(i), Some(c)])
        .filter_map(|c| c)
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .to_lowercase()
        .chars()
        .filter_map(coded_char)
        .collect()
}
