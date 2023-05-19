use std::fmt::{Display, Formatter};

use strum::IntoEnumIterator;

use itertools::iproduct;

use crate::game::rank::Rank;
use crate::game::suit::Suit;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    // one-eyed jacks remove
    pub fn is_one_eyed_jack(&self) -> bool {
        self.rank == Rank::JACK && (self.suit == Suit::SPADES || self.suit == Suit::HEARTS)
    }

    // two-eyed jacks are wild
    pub fn is_two_eyed_jack(&self) -> bool {
        self.rank == Rank::JACK && (self.suit == Suit::DIAMONDS || self.suit == Suit::CLUBS)
    }

    // returns a new vector of cards where each suit/rank combination is represented exactly once
    pub fn standard_deck() -> impl Iterator<Item = Card> {
        iproduct!(Suit::iter(), Rank::iter())
            .map(|(suit, rank)| Card { suit, rank })
    }
}
