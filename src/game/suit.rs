use std::fmt::{Display, Formatter, Result};

use strum_macros::EnumIter;

#[derive(Clone, Copy, EnumIter, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Suit {
    SPADES,
    HEARTS,
    DIAMONDS,
    CLUBS,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let suit_char = match self {
            Suit::SPADES => '♠',
            Suit::HEARTS => '♥',
            Suit::DIAMONDS => '♦',
            Suit::CLUBS => '♣',
        };

        write!(f, "{}", suit_char)
    }
}
