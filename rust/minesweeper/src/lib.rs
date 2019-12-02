use std::collections::HashMap;

fn neighbours(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut counts: HashMap<(i32, i32), u8> = HashMap::new();

    for (i, row) in minefield.iter().enumerate() {
        for (j, _) in row.chars().enumerate().filter(|(_, val)| *val == '*') {
            for neighbour in neighbours(i as i32, j as i32) {
                *counts.entry(neighbour).or_insert(b'0') += 1;
            }
        }
    }

    minefield.iter().enumerate().map(|(i, row)|
        row.chars()
            .enumerate()
            .map(|(j, val)| match (val, counts.get(&(i as i32, j as i32))) {
                ('*', _) => '*',
                (_, None) => ' ',
                (_, Some(&count)) => count as char
            })
            .collect()
        ).collect()
}
