use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use crate::game::suit::Suit;
use crate::game::rank::Rank;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    // one-eyed jacks remove
    pub fn is_one_eyed_jack(self) -> bool {
        self.rank == Rank::JACK && (self.suit == Suit::SPADES || self.suit == Suit::HEARTS)
    }

    // two-eyed jacks are wild
    pub fn is_two_eyed_jack(self) -> bool {
        self.rank == Rank::JACK && (self.suit == Suit::DIAMONDS || self.suit == Suit::CLUBS)
    }

    pub fn new_deck() -> Vec<Card> {
        Suit::iter()
            .flat_map(|suit| Rank::iter().map(move |rank| Card { suit, rank }))
            .collect()
    }
}
