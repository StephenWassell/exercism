#[macro_use] extern crate lazy_static;

use regex::Regex; // I♥RE

pub fn reply(message: &str) -> &str {
	lazy_static! {
		static ref question: Regex = Regex::new(r"\?\s*$").unwrap();
		static ref contains_uc: Regex = Regex::new(r"[A-Z]").unwrap();
		static ref contains_lc: Regex = Regex::new(r"[a-z]").unwrap();
		static ref empty: Regex = Regex::new(r"^\s*$").unwrap();
	}
	
	match (question.is_match(message),
		contains_uc.is_match(message),
		contains_lc.is_match(message),
		empty.is_match(message)) {

		// He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
		(false, true, false, false) => "Whoa, chill out!",
		// He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
		(true, true, false, false) => "Calm down, I know what I'm doing!",
		// Bob answers 'Sure.' if you ask him a question, such as "How are you?".
		(true, _, _, false) => "Sure.",
		// He says 'Fine. Be that way!' if you address him without actually saying anything.
		(_, _, _, true) => "Fine. Be that way!",
		// He answers 'Whatever.' to anything else.
		(_, _, _, _) => "Whatever."
	}
}
