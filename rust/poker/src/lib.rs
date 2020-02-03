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

    fn pack(&self) -> u8 {
        self.buff.checked_shl(4).unwrap() | self.rank.n
    }
}

fn buff(cards: &mut [Card], range: (usize, usize), amount: u8) {
    for c in cards[range.0..range.1].iter_mut() {
        c.buff = amount;
    }
}

fn buff_combinations(cards: &mut [Card]) {
    let mut straight = true;
    let mut flush = true;

    // index of the first card that's part of an n of a kind combination
    let mut kind_start = 0;

    // collections of n of a kind
    // ranges in cards instead of slices, because the compiler is worried that they might overlap
    let mut twos = Vec::<(usize, usize)>::new();
    let mut threes = Vec::<(usize, usize)>::new();
    let mut fours = Vec::<(usize, usize)>::new();

    // look at each pair of cards in sequence
    for (i, window) in cards.windows(2).enumerate() {
        let a = &window[0];
        let b = &window[1];
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
            // found the end of an n of a kind sequence
            let range = (kind_start, j);
            match j - kind_start {
                2 => twos.push(range),
                3 => threes.push(range),
                4 => fours.push(range),
                _ => (),
            };
            kind_start = j;
        }
    }

    if straight && flush {
        buff(cards, (0, cards.len()), 8);
    } else if fours.len() >= 1 {
        buff(cards, fours[0], 7);
    } else if threes.len() >= 1 && twos.len() >= 1 {
        buff(cards, threes[0], 6);
    } else if flush {
        buff(cards, (0, cards.len()), 5);
    } else if straight {
        buff(cards, (0, cards.len()), 4);
    } else if threes.len() >= 1 {
        buff(cards, threes[0], 3);
    } else if twos.len() >= 2 {
        buff(cards, twos[0], 2);
        buff(cards, twos[1], 2);
    } else if twos.len() >= 1 {
        buff(cards, twos[0], 1);
    }
}

/// pack all the cards into a single u64
fn calculate_score(cards: &[Card]) -> u64 {
    cards.iter().rev().fold(0, |acc, card| acc.checked_shl(8).unwrap() | card.pack() as u64)
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

        buff_combinations(&mut cards);

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
    winners.push(hands[0].original);
    Some(winners)
}
