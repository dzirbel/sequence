use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter, Clone, Copy)]
pub enum Suit {
    SPADES,
    HEARTS,
    DIAMONDS,
    CLUBS,
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn is_one_eyed_jack(&self) -> bool {
        self.rank == Rank::JACK && (self.suit == Suit::SPADES || self.suit == Suit::HEARTS)
    }

    fn is_two_eyed_jack(&self) -> bool {
        self.rank == Rank::JACK && (self.suit == Suit::DIAMONDS || self.suit == Suit::CLUBS)
    }

    fn to_string(&self) -> String {
        let suit = match self.suit {
            Suit::SPADES => '♠',
            Suit::HEARTS => '♥',
            Suit::DIAMONDS => '♦',
            Suit::CLUBS => '♣',
        };

        let rank = match self.rank {
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

        format!("{suit}{rank}")
    }

    fn all_cards() -> Vec<Card> {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card { suit, rank })
            }
        }
        cards
    }
}

pub struct Board {
    cards: [[Option<Card>; 10]; 10],
    tokens: [[u8; 10]; 10],
}

impl Board {
    fn coord_str(row: usize, col: usize) -> String {
        let col_str = (('a' as u8) + col as u8) as char;
        format!("{col_str}{row}")
    }

    pub fn print(&self) {
        for (i, row) in self.cards.iter().enumerate() {
            print!("| ");
            for (j, card) in row.iter().enumerate() {
                let token = self.tokens[i][j];
                let coord = Board::coord_str(i, j);
                let card_str = match card {
                    None => String::from("--"),
                    Some(card) => card.to_string(),
                };
                print!("{coord} [{token}] {card_str} | ")
            }
            println!()
        }
    }
}

pub fn standard_board() -> Board {
    Board {
        tokens: [[0u8; 10]; 10],
        cards: [
            [
                None,
                Some(Card { suit: Suit::SPADES, rank: Rank::TWO }),
                Some(Card { suit: Suit::SPADES, rank: Rank::THREE }),
                Some(Card { suit: Suit::SPADES, rank: Rank::FOUR }),
                Some(Card { suit: Suit::SPADES, rank: Rank::FIVE }),
                Some(Card { suit: Suit::SPADES, rank: Rank::SIX }),
                Some(Card { suit: Suit::SPADES, rank: Rank::SEVEN }),
                Some(Card { suit: Suit::SPADES, rank: Rank::EIGHT }),
                Some(Card { suit: Suit::SPADES, rank: Rank::NINE }),
                None,
            ],
            [
                Some(Card { suit: Suit::CLUBS, rank: Rank::SIX }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::FIVE }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::FOUR }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::THREE }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::TWO }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::ACE }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::KING }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::QUEEN }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::TEN }),
                Some(Card { suit: Suit::SPADES, rank: Rank::TEN }),
            ],
            [
                Some(Card { suit: Suit::CLUBS, rank: Rank::SEVEN }),
                Some(Card { suit: Suit::SPADES, rank: Rank::ACE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::TWO }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::THREE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::FOUR }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::FIVE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::SIX }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::SEVEN }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::NINE }),
                Some(Card { suit: Suit::SPADES, rank: Rank::QUEEN }),
            ],
            [
                Some(Card { suit: Suit::CLUBS, rank: Rank::EIGHT }),
                Some(Card { suit: Suit::SPADES, rank: Rank::KING }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::SIX }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::FIVE }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::FOUR }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::THREE }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::TWO }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::EIGHT }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::EIGHT }),
                Some(Card { suit: Suit::SPADES, rank: Rank::KING }),
            ],
            [
                Some(Card { suit: Suit::CLUBS, rank: Rank::NINE }),
                Some(Card { suit: Suit::SPADES, rank: Rank::QUEEN }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::SEVEN }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::SIX }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::FIVE }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::FOUR }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::ACE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::NINE }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::SEVEN }),
                Some(Card { suit: Suit::SPADES, rank: Rank::ACE }),
            ],
            [
                Some(Card { suit: Suit::CLUBS, rank: Rank::TEN }),
                Some(Card { suit: Suit::SPADES, rank: Rank::TEN }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::EIGHT }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::SEVEN }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::TWO }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::THREE }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::KING }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::TEN }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::SIX }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::TWO }),
            ],
            [
                Some(Card { suit: Suit::CLUBS, rank: Rank::QUEEN }),
                Some(Card { suit: Suit::SPADES, rank: Rank::NINE }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::NINE }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::EIGHT }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::NINE }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::TEN }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::QUEEN }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::QUEEN }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::FIVE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::THREE }),
            ],
            [
                Some(Card { suit: Suit::CLUBS, rank: Rank::KING }),
                Some(Card { suit: Suit::SPADES, rank: Rank::EIGHT }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::TEN }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::QUEEN }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::KING }),
                Some(Card { suit: Suit::CLUBS, rank: Rank::ACE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::ACE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::KING }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::FOUR }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::FOUR }),
            ],
            [
                Some(Card { suit: Suit::CLUBS, rank: Rank::ACE }),
                Some(Card { suit: Suit::SPADES, rank: Rank::SEVEN }),
                Some(Card { suit: Suit::SPADES, rank: Rank::SIX }),
                Some(Card { suit: Suit::SPADES, rank: Rank::FIVE }),
                Some(Card { suit: Suit::SPADES, rank: Rank::FOUR }),
                Some(Card { suit: Suit::SPADES, rank: Rank::THREE }),
                Some(Card { suit: Suit::SPADES, rank: Rank::TWO }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::TWO }),
                Some(Card { suit: Suit::HEARTS, rank: Rank::THREE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::FIVE }),
            ],
            [
                None,
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::ACE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::KING }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::QUEEN }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::TEN }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::NINE }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::EIGHT }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::SEVEN }),
                Some(Card { suit: Suit::DIAMONDS, rank: Rank::SIX }),
                None,
            ],
        ],
    }
}

#[test]
fn standard_board_has_correct_counts() {
    let board = standard_board();
    let mut counts = HashMap::new();

    for row in board.cards.iter() {
        for card in row.iter() {
            if let Some(card) = card {
                let count = counts.entry(card).or_insert(0);
                *count += 1;
            }
        }
    }

    let cards = Card::all_cards();
    for card in cards {
        let expected = if card.rank == Rank::JACK { None } else { Some(&2) };
        let actual = counts.get(&card);
        assert_eq!(expected, actual, "Card count was unexpected for {:?}", card);
    }
}
