use std::cmp::max;
use std::cmp::min;

// formulae from https://brilliant.org/wiki/sum-of-n-n2-or-n3/

fn arithmetic_sum(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

pub fn square_of_sum(n: u32) -> u32 {
    arithmetic_sum(n).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: u32) -> u32 {
    let a = sum_of_squares(n);
    let b = square_of_sum(n);
    max(a, b) - min(a, b)
}
