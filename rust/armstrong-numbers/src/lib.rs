fn vec_of_digits(mut num: u32) -> std::vec::Vec<u32> {
    let mut digits = Vec::new();

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    digits
}

pub fn is_armstrong_number(num: u32) -> bool {
	let digits = vec_of_digits(num);
	let exp = digits.len() as u32;
	digits.iter().map(|x| x.pow(exp)).sum::<u32>() == num
}
