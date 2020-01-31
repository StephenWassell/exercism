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
                "X" => 255, // end marker
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                _ => s.parse().unwrap(),
            },
        }
    }

    /// Does another card come after this one in a straight?
    fn is_next(&self, other: &Rank) -> bool {
        // The strange logic here is because an ace can be high or low for a straight,
        // but they are stored as high so will be last when the hand is sorted.
        // This makes A2345 equivalent to 2345A:
        self.n + 1 == other.n || self.n == 5 && other.n == 14
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    S,
    C,
    H,
    D,
    EndMarker,
}

impl Suit {
    fn new(s: &str) -> Suit {
        match s {
            "S" => Suit::S,
            "C" => Suit::C,
            "H" => Suit::H,
            "D" => Suit::D,
            "X" => Suit::EndMarker,
            _ => panic!("suit not recognised"),
        }
    }

    fn is_end_marker(&self) -> bool {
        *self == Suit::EndMarker
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    buff: u8,
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(s: &str) -> Card {
        let (rank_str, suit_str) = s.split_at(s.len() - 1);
        Card {
            buff: 0,
            rank: Rank::new(rank_str),
            suit: Suit::new(suit_str),
        }
    }
}

fn calculate_score(cards: &mut Vec<Card>) -> u64 {

    let mut straight = true;
    let mut flush = true;
    let mut kind_start = 0;

    let mut twos = Vec::<&[Card]>::new();
    let mut threes = Vec::<&[Card]>::new();
    let mut fours = Vec::<&[Card]>::new();

    for (i, w) in cards.windows(2).enumerate() {
        let a = &w[0];
        let b = &w[1];
        let j = i + 1;

        if !b.suit.is_end_marker() {
            if a.suit != b.suit {
                flush = false;
            }
    
            if !a.rank.is_next(&b.rank) {
                straight = false;
            }
        }
        
        if a.rank != b.rank {
            let slice = &cards[kind_start..j];
            match j - kind_start {
                2 => twos.push(slice),
                3 => threes.push(slice),
                4 => fours.push(slice),
                _ => (),
            };
            kind_start = j;
        }
    }

    if straight && flush {
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

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    original: &'a str,
    cards: Vec<Card>,
    score: u64,
}

impl<'a> Hand<'a> {
    fn new(original: &str) -> Hand {
        let mut cards: Vec<Card> = original.split_whitespace().map(|s| Card::new(s)).collect();
        //if cards.len() != 5 {
        //    panic!("hands must have 5 cards only")
        //}

        cards.sort_unstable();

        // add end marker
        cards.push(Card::new("XX"));

        //buff_specials(&mut cards);

        //cards.sort_unstable();

        let score = calculate_score(&mut cards);

        Hand {
            original: original,
            cards: cards,
            score: score,
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
