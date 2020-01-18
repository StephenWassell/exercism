#[derive(Debug, PartialEq)]
struct Value {
    n: u8,
}

impl Value {
    fn new(s: &str) -> Value {
        // Individual cards are ranked, from highest to lowest: A, K, Q, J, 10, 9, 8, 7, 6, 5, 4, 3 and 2
        Value {
            n: match s {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                _ => s.parse().unwrap(),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum Suit {
    S,
    C,
    H,
    D,
}

impl Suit {
    fn new(s: &str) -> Suit {
        match s {
            "S" => Suit::S,
            "C" => Suit::C,
            "H" => Suit::H,
            "D" => Suit::D,
            _ => panic!("suit not recognised"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    fn new(s: &str) -> Card {
        let (value_str, suit_str) = s.split_at(s.len() - 1);
        Card {
            value: Value::new(value_str),
            suit: Suit::new(suit_str),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    original: &'a str,
    cards: Vec<Card>,
}

impl<'a> Hand<'a> {
    fn new(s: &str) -> Hand {
        Hand {
            original: s,
            cards: s.split_whitespace().map(|s| Card::new(s)).collect(),
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hand_strings: &[&'a str]) -> Option<Vec<&'a str>> {
    let hands: Vec<Hand> = hand_strings.iter().map(|s| Hand::new(s)).collect();
    let mut winners: Vec<&'a str> = Vec::new();
    winners.push(hands[0].original);
    Some(winners)
}
