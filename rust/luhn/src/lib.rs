/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
	let n = code.chars().filter(|&c| c != ' ').pairs();
	false
}
