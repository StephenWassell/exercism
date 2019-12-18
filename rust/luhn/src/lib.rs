fn to_digits(code: &str) -> Option<Vec<u32>> {
    let no_spaces = code.chars().filter(|&c| !c.is_whitespace());

    if no_spaces.clone().any(|c| !c.is_numeric()) {
        None
    } else {
        Some(no_spaces.map(|c| c as u32 - b'0' as u32).collect())
    }
}

fn luhn_double(x: u32) -> u32 {
    match x * 2 {
        x2 if x2 > 9 => x2 - 9,
        x2 => x2,
    }
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits = match to_digits(code) {
        Some(v) if v.len() > 1 => v,
        _ => return false,
    };

    let sum = digits.rchunks(2).fold(0, |acc, item| {
        acc + match item {
            &[a, b] => luhn_double(a) + b,
            &[a] => a,
            _ => 0,
        }
    });

    sum % 10 == 0
}
