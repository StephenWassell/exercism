#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rank {
    n: u8,
}

impl Rank {
    fn new(s: &str) -> Rank {
        // Individual cards are ranked, from highest to lowest:
        // A, K, Q, J, 10, 9, 8, 7, 6, 5, 4, 3, and 2
        Rank {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(s: &str) -> Card {
        let (rank_str, suit_str) = s.split_at(s.len() - 1);
        Card {
            rank: Rank::new(rank_str),
            suit: Suit::new(suit_str),
        }
    }
}

/// A flush is a hand that contains five cards all of the same suit
fn is_flush(cards: &[Card]) -> bool {
    let first_suit = &cards[0].suit;
    cards[1..].iter().all(|c| c.suit == *first_suit)
}

/// A straight is a hand that contains five cards of sequential rank
fn is_straight(cards: &[Card]) -> bool {
    cards
        .windows(2)
        .all(|w| w[0].rank.n + 1 == w[1].rank.n)
}

/// If a slice of cards all have the same rank, return the slice
/// - this makes the functions that call this one simpler
fn same_rank(cards: &[Card]) -> Option<&[Card]> {
    let first_rank = &cards[0].rank;
    if cards[1..].iter().all(|c| c.rank == *first_rank) {
        Some(cards)
    } else {
        None
    }
}

fn is_n_of_a_kind(cards: &[Card], n: usize) -> Option<&[Card]> {
    for w in cards.windows(n) {
        if let Some(group) = same_rank(w) {
            return Some(group);
        }
    }
    None
}

/// If the hand contains a four of a kind, return that group as a slice
// fn is_four_of_a_kind(cards: &[Card]) -> Option<&[Card]> {
//     if let Some(four) = same_rank(&cards[0..4]) {
//         Some(four)
//     } else if let Some(four) = same_rank(&cards[1..5]) {
//         Some(four)
//     } else {
//         None
//     }
// }

/// For a full house, return the slice for the three of a kind only
fn is_full_house(cards: &[Card]) -> Option<&[Card]> {
    if let Some(three) = same_rank(&cards[0..3]) {
        if same_rank(&cards[3..5]) != None {
            return Some(three);
        }
    }
    if let Some(three) = same_rank(&cards[2..5]) {
        if same_rank(&cards[0..2]) != None {
            return Some(three);
        }
    }
    None
}

// fn is_three_of_a_kind(cards: &[Card]) -> Option<&[Card]> {
//     same_rank(&cards[0..3]) || same_rank(&cards[1..4]) || same_rank(&cards[2..5])
// }

// fn is_two_pair(cards: &[Card]) -> Option<&[Card]> {
//     (same_rank(&cards[0..2]) && same_rank(&cards[2..4]))
//         || (same_rank(&cards[0..2]) && same_rank(&cards[3..5]))
//         || (same_rank(&cards[1..3]) && same_rank(&cards[3..5]))
// }

// fn is_one_pair(cards: &[Card]) -> Option<&[Card]> {
//     same_rank(&cards[0..2])
//         || same_rank(&cards[1..3])
//         || same_rank(&cards[2..4])
//         || same_rank(&cards[3..5])
// }

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    original: &'a str,
    cards: Vec<Card>,
    value: u32,
}

impl<'a> Hand<'a> {
    fn new(original: &str) -> Hand {
        let mut cards: Vec<Card> = original.split_whitespace().map(|s| Card::new(s)).collect();
        if cards.len() != 5 {
            panic!("hands must have 5 cards only")
        }
        cards.sort_unstable();

        let mut hand = Hand {
            original: original,
            cards: cards,
            value: 0,
        };

        hand.calculate_value();

        hand
    }

    fn calculate_value(&mut self) {
        let flush = is_flush(&self.cards);
        let straight = is_straight(&self.cards);
        self.value = if straight && flush {
            8
        } else if is_four_of_a_kind(&self.cards) {
            7
        } else if is_full_house(&self.cards) {
            6
        } else if flush {
            5
        } else if straight {
            4
        } else if is_three_of_a_kind(&self.cards) {
            3
        } else if is_two_pair(&self.cards) {
            2
        } else if is_one_pair(&self.cards) {
            1
        } else {
            0
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hand_strings: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands: Vec<Hand> = hand_strings.iter().map(|s| Hand::new(s)).collect();
    hands.sort_unstable_by(|b, a| a.value.partial_cmp(&b.value).unwrap());
    let mut winners: Vec<&'a str> = Vec::new();
    winners.push(hands[0].original);
    Some(winners)
}
