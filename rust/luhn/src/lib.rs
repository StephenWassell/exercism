fn to_digits(code: &str) -> Option<Vec<u32>> {
    if code.chars().any(|c| !(c.is_numeric() || c.is_whitespace())) {
        None
    } else {
        Some(
            code.chars()
                .filter(|&c| !c.is_whitespace())
                .map(|c| c as u32 - b'0' as u32)
                .collect(),
        )
    }
}

fn luhn_double(n: u32) -> u32 {
    match n * 2 {
        m if m > 9 => m - 9,
        m => m,
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
