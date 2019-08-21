pub fn nth(n: u32) -> u32 {
	let n = n as usize;
	let mut primes = vec![2];
	let mut candidate = 3;
	
	while primes.len() <= n {
		if primes.iter().find(|&x| candidate % x == 0) == None {
			primes.push(candidate);
		}
		candidate += 2;
	}
	
	primes[n]
}
