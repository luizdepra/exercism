use std::cmp::{Ordering, Reverse};

use itertools::Itertools;

enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

enum Rank {
    N(u8),
    Jack,
    Queen,
    King,
    Ace,
}

struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn new(data: &str) -> Self {
        let i = data.len() - 1;
        let rank_str = &data[..i];
        let suit_str = &data[i..];

        let rank = match rank_str {
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            r => Rank::N(r.parse().unwrap()),
        };

        let suit = match suit_str {
            "H" => Suit::Hearts,
            "D" => Suit::Diamonds,
            "S" => Suit::Spades,
            "C" => Suit::Clubs,
            _ => panic!(),
        };

        Card {
            suit,
            rank,
        }
    }
}

impl From<&Card> for u8 {
    fn from(card: &Card) -> Self {
        match card.rank {
            Rank::N(n) => n,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

struct Hand<'a> {
    cards: Vec<Card>,
    data: &'a str,
}

impl<'a> Hand<'a> {
    fn new(data: &'a str) -> Self {
        let cards: Vec<Card> = data.split_whitespace()
            .map(|c| Card::new(c))
            .collect();

        Hand {
            cards,
            data,
        }
    }

    fn value(&self) -> Vec<u8> {
        let mut values: Vec<u8> = self.cards.iter()
            .map(Into::into)
            .collect();

        values.sort_by_key(|&n| Reverse(n));

        values
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value()
            .partial_cmp(&other.value())
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(input: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands: Vec<Hand> = input.iter()
        .map(|h| Hand::new(h))
        .collect();

    hands.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let result: Vec<&'a str> = hands.iter()
        .group_by(|h| h.value())
        .into_iter()
        .map(|(_, v)| v.map(|x| x.data).collect())
        .next()
        .unwrap();

    Some(result)
}
