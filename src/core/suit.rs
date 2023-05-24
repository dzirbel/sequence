use std::fmt::{Display, Formatter, Result};

use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let suit_char = match self {
            Suit::Spades => '♠',
            Suit::Hearts => '♥',
            Suit::Diamonds => '♦',
            Suit::Clubs => '♣',
        };

        write!(f, "{}", suit_char)
    }
}
