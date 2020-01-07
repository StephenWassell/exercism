use crossbeam::scope;
// use crossbeam::thread::ScopedJoinHandle;
use std::collections::HashMap;
// use std::sync::Mutex;

type Counts = HashMap<char, usize>;

/// the reference implementation from benchmark.rs
fn calc_frequency(texts: &[&str]) -> Counts {
    let mut map = HashMap::new();

    for line in texts {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                (*map.entry(c).or_insert(0)) += 1;
            }
        }
    }

    map
}

pub fn frequency(input: &[&str], worker_count: usize) -> Counts {
    let mut map = HashMap::new();
    // let map_mutex = Mutex::new(&map);

    scope(|s| {
        //crossbeam::thread::ScopedJoinHandle<'_, Counts>
        // let workers: Vec<_> = (0..worker_count)
        let mut workers = Vec::new();
        for i in 0..worker_count {
            // .map(|i| {
            let partition =
                &input[(i * input.len()) / worker_count..((i + 1) * input.len()) / worker_count];

            let worker = s.spawn(move |_| {
                // let partial_map =
                calc_frequency(partition)
                // map_ref.lock().unwrap().extend(partial_map);
            });

            workers.push(worker);
            // })
            // .collect();
        }

        // let map_ref = &mut map;

        for worker in workers {
            map.extend(worker.join().unwrap());
        }
    })
    .unwrap();

    map
}
