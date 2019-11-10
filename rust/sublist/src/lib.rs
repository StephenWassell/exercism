#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_in_list<T: PartialEq>(needle: &[T], haystack: &[T]) -> bool {
    for i in 0..=haystack.len() - needle.len() {
        if haystack.iter().skip(i).take(needle.len()).eq(needle.iter())
        {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let len1 = first_list.len();
    let len2 = second_list.len();

    let (needle, haystack, success) = if len1 == len2 {
        (first_list, second_list, Comparison::Equal)
    } else if len1 < len2 {
        (first_list, second_list, Comparison::Sublist)
    } else {
        (second_list, first_list, Comparison::Superlist)
    };

    if is_in_list(needle, haystack) {
        success
    } else {
        Comparison::Unequal
    }
}
