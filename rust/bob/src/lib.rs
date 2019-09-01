fn is_question(message: &str) -> bool {
	message.ends_with('?')
}

fn is_yelled(message: &str) -> bool {
    message.chars().any(char::is_uppercase) &&
    !message.chars().any(char::is_lowercase)
}

pub fn reply(message: &str) -> &str {
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
    }
}
