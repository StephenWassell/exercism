fn code_char(c: char) -> char {
	const A: u8 = 'a' as u8;
	const Z: u8 = 'z' as u8;
	let c = c as u8;
	(A + (Z - c)) as char
}

fn code_str(src: &str) -> String {
	let mut dst = String::new();
	for c in src.to_lowercase().chars() {
		match c {
			'a'...'z' => dst.push(code_char(c)),
			'0'...'9' => dst.push(c),
			_ => ()
		}
	}
	dst
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
	code_str(plain)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
	code_str(cipher)
}
