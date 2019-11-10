use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_in_list<T: PartialEq>(needle: &[T], haystack: &[T]) -> bool {
    for i in 0..=haystack.len() - needle.len() {
        if haystack.iter().skip(i).take(needle.len()).eq(needle.iter()) {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let (needle, haystack, success) = match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => (first_list, second_list, Comparison::Equal),
        Ordering::Less => (first_list, second_list, Comparison::Sublist),
        Ordering::Greater => (second_list, first_list, Comparison::Superlist),
    };

    if is_in_list(needle, haystack) {
        success
    } else {
        Comparison::Unequal
    }
}
