// fn digits<'a>(code: &'a str) -> Option<impl Iterator> {
//     let no_spaces = code.chars().filter(|&c| c != ' ').cloned();

//     if no_spaces.any(|c| match c {
//         '0'..='9' => false,
//         _ => true,
//     }) {
//         return None;
//     }

//     Some(no_spaces.map(|c| c as u8 - b'0'))
// }

fn not_digit(c: char) -> bool {
    match c {
        '0'..='9' => false,
        _ => true,
    }
}
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits = code
        .chars()
        .filter(|&c| c != ' ');

    if digits.clone().count() < 2 || digits.clone().any(not_digit) {
        return false;
    }

    let sum: u32 = digits
        .map(|c| c as u8 - b'0')
        .rev()
        .enumerate()
        .map(|(i, val)| (val as u32) << (i & 1))
        .map(|val| if val > 9 { val - 9 } else { val })
        .sum();

    sum % 10 == 0
}
