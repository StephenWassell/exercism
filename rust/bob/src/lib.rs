pub fn reply(message: &str) -> &str {
    let mut contains_upper = false;
    let mut contains_lower = false;
    let mut not_empty = false;
    let mut last = 'x';

    for c in message.chars() {
        if c.is_uppercase() {
            contains_upper = true;
        }
        if c.is_lowercase() {
            contains_lower = true;
        }
        if !c.is_whitespace() {
            not_empty = true;
            last = c;
        }
    }

    match (last == '?', contains_upper, contains_lower, not_empty) {
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
    }
}
