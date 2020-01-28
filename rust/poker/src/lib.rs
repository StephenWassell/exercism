#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rank {
    n: u16,
}

impl Rank {
    fn new(s: &str) -> Rank {
        // Individual cards are ranked, from highest to lowest:
        // A, K, Q, J, 10, 9, 8, 7, 6, 5, 4, 3, and 2
        Rank {
            n: match s {
                "A" => (1 << 13) | 1,
                "K" => 1 << 12,
                "Q" => 1 << 11,
                "J" => 1 << 10,
                _ => 1 << (s.parse::<u16>().unwrap() - 1),
            },
        }
    }

    fn is_next(&self, other: &Rank) -> bool {
        (self.n << 1) & other.n != 0
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
    score: u32,
}

impl<'a> Hand<'a> {
    fn new(original: &str) -> Hand {
        let cards: Vec<Card> = original.split_whitespace().map(|s| Card::new(s)).collect();
        //if cards.len() != 5 {
        //    panic!("hands must have 5 cards only")
        //}

        let mut hand = Hand {
            original: original,
            cards: cards,
            score: 0,
        };

        hand.calculate_score();

        hand
    }

    fn calculate_score(&mut self) {
        self.cards.sort_unstable(); // todo: not right for low ace

        let mut straight = true;
        let mut flush = true;
        let mut kind_start = 0;

        let mut twos = Vec::<&[Card]>::new();
        let mut threes = Vec::<&[Card]>::new();
        let mut fours = Vec::<&[Card]>::new();

        for (i, w) in self.cards.windows(2).enumerate() {
            let a = &w[0];
            let b = &w[1];
            let j = i + 1;

            if a.suit != b.suit {
                flush = false;
            }

            if !a.rank.is_next(&b.rank) {
                straight = false;
            }

            if a.rank != b.rank {
                let slice = &self.cards[kind_start..j];
                match j - kind_start {
                    2 => twos.push(slice),
                    3 => threes.push(slice),
                    4 => fours.push(slice),
                    _ => ()
                };
                kind_start = j;
            }
        }
        // todo: deal with last card being part of a kind

//        let flush = is_flush(&self.cards);
//        let straight = is_straight(&self.cards);
        self.score = if straight && flush {
            8
        } else if fours.len() >= 1 {
            7
        } else if threes.len() >= 1 && twos.len() >= 1 {
            6
        } else if flush {
            5
        } else if straight {
            4
        } else if threes.len() >= 1 {
            3
        } else if twos.len() >= 2 {
            2
        } else if twos.len() >= 1 {
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
    hands.sort_unstable_by(|b, a| a.score.partial_cmp(&b.score).unwrap());
    let mut winners: Vec<&'a str> = Vec::new();
    winners.push(hands[0].original);
    Some(winners)
}
