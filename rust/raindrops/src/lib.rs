pub fn raindrops(n: u32) -> String {

	//has 3 as a factor, add 'Pling' to the result.
	//has 5 as a factor, add 'Plang' to the result.
	//has 7 as a factor, add 'Plong' to the result.
	const FACTOR_NAMES: &[(u32, &str)] = &[
		(3, "Pling"),
		(5, "Plang"),
		(7, "Plong")
	];

	let mut result = String::new();

	for (factor, name) in FACTOR_NAMES {
		if n % factor == 0 {
			result += name;
		}
	}

	//does not have any of 3, 5, or 7 as a factor,
	//the result should be the digits of the number.
	if result.len() == 0 {
		result = n.to_string()
	}

	result
}
