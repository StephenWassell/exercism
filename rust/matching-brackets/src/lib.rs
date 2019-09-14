#[derive(PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
}

struct BracketStack {
    stack: Vec<Bracket>,
}

impl BracketStack {
    fn new() -> BracketStack {
        BracketStack { stack: Vec::new() }
    }
    fn push_true(&mut self, b: Bracket) -> bool {
        self.stack.push(b);
        true
    }
    fn pop_check(&mut self, b: Bracket) -> bool {
        self.stack.pop() == Some(b)
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    fn check_char(&mut self, c: char) -> bool {
        match c {
            '(' => self.push_true(Bracket::Round),
            ')' => self.pop_check(Bracket::Round),
            '[' => self.push_true(Bracket::Square),
            ']' => self.pop_check(Bracket::Square),
            '{' => self.push_true(Bracket::Curly),
            '}' => self.pop_check(Bracket::Curly),
            _ => true,
        }
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = BracketStack::new();
    string.chars().all(|c| stack.check_char(c)) && stack.is_empty()
}
