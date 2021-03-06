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
                "X" => 0, // end marker
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
        // The special case here is because an ace can be high or low for a straight,
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
    // The boost is set for the first card in a combination (eg n of a kind)
    // to a value that corresponds to the value of the hand.
    boost: u8,
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(s: &str) -> Card {
        let (rank_str, suit_str) = s.split_at(s.len() - 1);
        Card {
            boost: 0,
            rank: Rank::new(rank_str),
            suit: Suit::new(suit_str),
        }
    }

    fn pack(&self) -> u8 {
        self.boost.checked_shl(4).unwrap() | self.rank.n
    }
}

fn boost_combinations(cards: &mut [Card]) {
    // Assume every hand is a straight or flush with the lowest card first
    // (because it's already sorted) unless we discover otherwise.
    let mut straight: Option<usize> = Some(0);
    let mut flush = true;

    // Index of the first card that's part of an n of a kind combination
    let mut kind_start = 0;

    // Collections of n of a kind - index of the first card in the combination
    let mut twos = Vec::<usize>::new();
    let mut threes = Vec::<usize>::new();
    let mut fours = Vec::<usize>::new();

    // Look at each pair of cards in sequence
    for (i, window) in cards.windows(2).enumerate() {
        let a = &window[0];
        let b = &window[1];
        let j = i + 1;

        if !b.suit.is_end_marker() {
             // Check if it's not a straight or flush after all
            if a.suit != b.suit {
                flush = false;
            }
            if !a.rank.is_next(&b.rank) {
                straight = None;
            }
        }

        if a.rank != b.rank {
            // Found the end of an n of a kind sequence
            match j - kind_start {
                2 => twos.push(kind_start),
                3 => threes.push(kind_start),
                4 => fours.push(kind_start),
                _ => (),
            };
            kind_start = j;
        }
    }

    // Special case for straight with a low ace
    // - we want to boost the lowest card in the sequence
    if straight != None && cards[0].rank.n == 2 && cards[4].rank.n == 14 {
        straight = Some(4);
        cards[4].rank.n = 1;
    }

    if straight != None && flush {
        cards[straight.unwrap()].boost = 8;
    } else if fours.len() >= 1 {
        cards[fours[0]].boost = 7;
    } else if threes.len() >= 1 && twos.len() >= 1 {
        cards[threes[0]].boost = 6;
    } else if flush {
        cards[0].boost = 5;
    } else if straight != None {
        cards[straight.unwrap()].boost = 4;
    } else if threes.len() >= 1 {
        cards[threes[0]].boost = 3;
    } else if twos.len() >= 2 {
        cards[twos[0]].boost = 2;
        cards[twos[1]].boost = 2;
    } else if twos.len() >= 1 {
        cards[twos[0]].boost = 1;
    }
}

/// pack all the cards into a single u64
fn calculate_score(cards: &[Card]) -> u64 {
    cards.iter().rev().fold(0, |acc, card| {
        acc.checked_shl(8).unwrap() | card.pack() as u64
    })
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

        // sort the cards by rank, low to high
        cards.sort_unstable();

        // add end marker to help find n of a kind combinations
        cards.push(Card::new("XX"));

        boost_combinations(&mut cards);

        cards.pop();

        // now combinations are worth extra so sort them to the end
        cards.sort_unstable();

        let score = calculate_score(&cards);

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
    for hand in hands.iter().filter(|h| h.score == hands[0].score) {
        winners.push(hand.original);
    }

    Some(winners)
}
