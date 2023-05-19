use std::fmt::{Display, Formatter, Result};

use strum_macros::EnumIter;

#[derive(Clone, Copy, EnumIter, Eq, Hash, PartialEq)]
pub enum Rank {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let rank_char = match self {
            Rank::TWO => '2',
            Rank::THREE => '3',
            Rank::FOUR => '4',
            Rank::FIVE => '5',
            Rank::SIX => '6',
            Rank::SEVEN => '7',
            Rank::EIGHT => '8',
            Rank::NINE => '9',
            Rank::TEN => 'T',
            Rank::JACK => 'J',
            Rank::QUEEN => 'Q',
            Rank::KING => 'K',
            Rank::ACE => 'A',
        };

        write!(f, "{}", rank_char)
    }
}
