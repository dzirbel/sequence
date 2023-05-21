use crate::core::board::{Board, BOARD_SIZE};
use crate::core::card::Card;
use crate::core::rank::Rank;
use crate::core::suit::Suit;

impl Board {
    /*
    | a0 -- | b0 2♠ | c0 3♠ | d0 4♠ | e0 5♠ | f0 6♠ | g0 7♠ | h0 8♠ | i0 9♠ | j0 -- |
    | a1 6♣ | b1 5♣ | c1 4♣ | d1 3♣ | e1 2♣ | f1 A♥ | g1 K♥ | h1 Q♥ | i1 T♥ | j1 T♠ |
    | a2 7♣ | b2 A♠ | c2 2♦ | d2 3♦ | e2 4♦ | f2 5♦ | g2 6♦ | h2 7♦ | i2 9♥ | j2 Q♠ |
    | a3 8♣ | b3 K♠ | c3 6♣ | d3 5♣ | e3 4♣ | f3 3♣ | g3 2♣ | h3 8♦ | i3 8♥ | j3 K♠ |
    | a4 9♣ | b4 Q♠ | c4 7♣ | d4 6♥ | e4 5♥ | f4 4♥ | g4 A♥ | h4 9♦ | i4 7♥ | j4 A♠ |
    | a5 T♣ | b5 T♠ | c5 8♣ | d5 7♥ | e5 2♥ | f5 3♥ | g5 K♥ | h5 T♦ | i5 6♥ | j5 2♦ |
    | a6 Q♣ | b6 9♠ | c6 9♣ | d6 8♥ | e6 9♥ | f6 T♥ | g6 Q♥ | h6 Q♦ | i6 5♥ | j6 3♦ |
    | a7 K♣ | b7 8♠ | c7 T♣ | d7 Q♣ | e7 K♣ | f7 A♣ | g7 A♦ | h7 K♦ | i7 4♥ | j7 4♦ |
    | a8 A♣ | b8 7♠ | c8 6♠ | d8 5♠ | e8 4♠ | f8 3♠ | g8 2♠ | h8 2♥ | i8 3♥ | j8 5♦ |
    | a9 -- | b9 A♦ | c9 K♦ | d9 Q♦ | e9 T♦ | f9 9♦ | g9 8♦ | h9 7♦ | i9 6♦ | j9 -- |
    */
    pub fn new() -> Board {
        Board {
            chips: [[None; BOARD_SIZE as usize]; BOARD_SIZE as usize],
            sequences: Vec::new(),
            cards: [
                [
                    None,
                    Some(Card { suit: Suit::Spades, rank: Rank::Two }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Three }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Four }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Five }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Six }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Seven }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Eight }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Nine }),
                    None,
                ],
                [
                    Some(Card { suit: Suit::Clubs, rank: Rank::Six }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Five }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Four }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Three }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Two }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Ace }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::King }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Queen }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Ten }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Ten }),
                ],
                [
                    Some(Card { suit: Suit::Clubs, rank: Rank::Seven }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Ace }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Two }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Three }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Four }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Five }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Six }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Seven }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Nine }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Queen }),
                ],
                [
                    Some(Card { suit: Suit::Clubs, rank: Rank::Eight }),
                    Some(Card { suit: Suit::Spades, rank: Rank::King }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Six }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Five }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Four }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Three }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Two }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Eight }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Eight }),
                    Some(Card { suit: Suit::Spades, rank: Rank::King }),
                ],
                [
                    Some(Card { suit: Suit::Clubs, rank: Rank::Nine }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Queen }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Seven }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Six }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Five }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Four }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Ace }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Nine }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Seven }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Ace }),
                ],
                [
                    Some(Card { suit: Suit::Clubs, rank: Rank::Ten }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Ten }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Eight }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Seven }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Two }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Three }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::King }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Ten }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Six }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Two }),
                ],
                [
                    Some(Card { suit: Suit::Clubs, rank: Rank::Queen }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Nine }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Nine }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Eight }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Nine }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Ten }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Queen }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Queen }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Five }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Three }),
                ],
                [
                    Some(Card { suit: Suit::Clubs, rank: Rank::King }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Eight }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Ten }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Queen }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::King }),
                    Some(Card { suit: Suit::Clubs, rank: Rank::Ace }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Ace }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::King }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Four }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Four }),
                ],
                [
                    Some(Card { suit: Suit::Clubs, rank: Rank::Ace }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Seven }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Six }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Five }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Four }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Three }),
                    Some(Card { suit: Suit::Spades, rank: Rank::Two }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Two }),
                    Some(Card { suit: Suit::Hearts, rank: Rank::Three }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Five }),
                ],
                [
                    None,
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Ace }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::King }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Queen }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Ten }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Nine }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Eight }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Seven }),
                    Some(Card { suit: Suit::Diamonds, rank: Rank::Six }),
                    None,
                ],
            ],
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn standard_board_has_two_copies_of_each_card_except_jacks() {
        let board = Board::new();
        let mut counts = HashMap::new();

        for row in board.cards.iter() {
            for card in row.iter().flatten() {
                let count = counts.entry(card).or_insert(0);
                *count += 1;
            }
        }

        for card in Card::standard_deck() {
            let expected = if card.rank == Rank::Jack { None } else { Some(&2) };
            let actual = counts.get(&card);
            assert_eq!(expected, actual, "Card count was unexpected for {}", card);
        }
    }
}
