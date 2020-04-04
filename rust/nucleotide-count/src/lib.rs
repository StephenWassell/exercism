use std::collections::HashMap;

fn new_counts() -> HashMap<char, usize> {
    [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .iter()
        .cloned()
        .collect()
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let counts = nucleotide_counts(dna)?;
    match counts.get(&nucleotide) {
        Some(val) => Ok(*val),
        None => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = new_counts();
    for key in dna.chars() {
        match counts.get_mut(&key) {
            Some(val) => *val += 1,
            None => return Err(key),
        };
    }
    Ok(counts)
}
