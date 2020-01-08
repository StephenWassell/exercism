// Allows threads to use the input array directly
use crossbeam::scope;

// Faster for small keys than the standard HashMap
// (see http://cglab.ca/%7Eabeinges/blah/hash-rs/)
// It lets bench_large_parallel run about 15% faster
use fnv::FnvHashMap;

use std::collections::HashMap;

/// Based on the reference implementation from benchmark.rs
fn calc_frequency(texts: &[&str]) -> FnvHashMap<char, usize> {
    let mut map = FnvHashMap::default();

    for line in texts {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                (*map.entry(c).or_insert(0)) += 1;
            }
        }
    }

    map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    scope(|s| {
        let mut workers = Vec::new();

        for i in 0..worker_count {
            let partition =
                &input[(i * input.len()) / worker_count..((i + 1) * input.len()) / worker_count];

            workers.push(s.spawn(move |_| calc_frequency(partition)));
        }

        for worker in workers {
            for (key, value) in worker.join().unwrap() {
                (*map.entry(key).or_insert(0)) += value;
            }
        }
    })
    .unwrap();

    map
}
