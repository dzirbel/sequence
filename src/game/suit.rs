use std::fmt::{Display, Formatter, Result};

use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Hash, EnumIter, Copy, Clone)]
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
