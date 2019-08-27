//extern crate itertools;
use itertools::Itertools;

fn for_want_of(a: &str, b: &str) -> String {
	format!("For want of a {} the {} was lost.\n", a, b)
}

fn and_all_for(a: &str) -> String {
	format!("And all for the want of a {}.", a)
}

pub fn build_proverb(list: &[&str]) -> String {
	if list.is_empty() {
		String::new()
	} else {
		list
			.iter()
			.tuple_windows()
			.map(|(a, b)| for_want_of(a, b))
			.fold(String::new(), |acc, x| acc + &x)
			+ &and_all_for(&list[0])
	}
}
