pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = vec![];
    
    for f in 2.. {
    	if n <= 1 {
    		break;
		} 
    	while n % f == 0 {
    		result.push(f);
    		n /= f;
    	}
    }
    
    result
}
