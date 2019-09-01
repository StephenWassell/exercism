<<<<<<< HEAD
#[macro_use]
extern crate lazy_static;
=======
fn is_question(message: &str) -> bool {
	message.ends_with('?')
}
>>>>>>> ecf0f1c519b0406dfcfb5f8e86fba81b08c3c587

fn is_yelled(message: &str) -> bool {
    message.chars().any(char::is_uppercase) &&
    !message.chars().any(char::is_lowercase)
}

pub fn reply(message: &str) -> &str {
<<<<<<< HEAD
    lazy_static! {
        static ref RE: RegexSet = RegexSet::new(&[
            r"\?\s*$", // ends with a question mark
            r"[A-Z]",  // contains upper case
            r"[a-z]",  // contains lower case
            r"\S"      // not empty
        ]).unwrap();
    }

    let matches = RE.matches(message);

    match (
        matches.matched(0),
        matches.matched(1),
        matches.matched(2),
        matches.matched(3),
    ) {
        // He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
        (false, true, false, true) => "Whoa, chill out!",
        // He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
        (true, true, false, true) => "Calm down, I know what I'm doing!",
        // Bob answers 'Sure.' if you ask him a question, such as "How are you?".
        (true, _, _, true) => "Sure.",
        // He says 'Fine. Be that way!' if you address him without actually saying anything.
        (_, _, _, false) => "Fine. Be that way!",
        // He answers 'Whatever.' to anything else.
        (_, _, _, _) => "Whatever.",
=======
    let trimmed = message.trim();

    match (is_question(trimmed), is_yelled(trimmed), trimmed.is_empty()) {
        // He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
        (false, true, false) => "Whoa, chill out!",
        // He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
        (true, true, false) => "Calm down, I know what I'm doing!",
        // Bob answers 'Sure.' if you ask him a question, such as "How are you?".
        (true, false, false) => "Sure.",
        // He says 'Fine. Be that way!' if you address him without actually saying anything.
        (_, _, true) => "Fine. Be that way!",
        // He answers 'Whatever.' to anything else.
        (_, _, _) => "Whatever.",
>>>>>>> ecf0f1c519b0406dfcfb5f8e86fba81b08c3c587
    }
}
