use std::fmt::{Display, Formatter};

use itertools::iproduct;
use strum::IntoEnumIterator;

use crate::core::rank::Rank;
use crate::core::suit::Suit;

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
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
        self.rank == Rank::Jack && (self.suit == Suit::Spades || self.suit == Suit::Hearts)
    }

    // two-eyed jacks are wild
    pub fn is_two_eyed_jack(&self) -> bool {
        self.rank == Rank::Jack && (self.suit == Suit::Diamonds || self.suit == Suit::Clubs)
    }

    // returns a new vector of cards where each suit/rank combination is represented exactly once
    pub fn standard_deck() -> impl Iterator<Item = Card> {
        iproduct!(Suit::iter(), Rank::iter())
            .map(|(suit, rank)| Card { suit, rank })
    }
}
