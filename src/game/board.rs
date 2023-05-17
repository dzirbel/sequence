use crate::game::card::Card;
use crate::game::rank::Rank;
use crate::game::square::Square;
use crate::game::suit::Suit;
use crate::game::team::Team;

pub const BOARD_SIZE: usize = 10;
pub const SEQUENCE_LENGTH: usize = 5;

pub struct Board {
    pub cards: [[Option<Card>; BOARD_SIZE]; BOARD_SIZE],
    chips: [[Option<Team>; BOARD_SIZE]; BOARD_SIZE],
    sequences: Vec<(Team, Vec<Square>)>,
}

impl Board {
    pub fn is_playable(square: &Square) -> bool {
        Board::is_valid(square) && !Board::is_corner(square)
    }

    pub fn is_corner(square: &Square) -> bool {
        (square.row == 0 && square.col == 0) ||
            (square.row == 0 && square.col == BOARD_SIZE - 1) ||
            (square.row == BOARD_SIZE - 1 && square.col == 0) ||
            (square.row == BOARD_SIZE - 1 && square.col == BOARD_SIZE - 1)
    }

    pub fn is_valid(square: &Square) -> bool {
        square.row < BOARD_SIZE && square.col < BOARD_SIZE
    }

    pub fn card_at(&self, square: &Square) -> Option<Card> {
        self.cards[square.row][square.col]
    }

    pub fn is_dead(&self, card: &Card) -> bool {
        if card.rank == Rank::JACK { return false; }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(c) = self.cards[row][col] {
                    if &c == card && self.chips[row][col] == None {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn is_full(&self) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if Board::is_corner(&Square { row, col }) {
                    continue;
                }

                if self.chips[row][col] == None {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_empty(&self) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if Board::is_corner(&Square { row, col }) {
                    continue;
                }

                if self.chips[row][col] != None {
                    return false;
                }
            }
        }

        true
    }

    // note: returns empty vector for jacks
    pub fn unoccupied_squares_for_card(&self, card: &Card) -> Vec<Square> {
        let mut squares = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(c) = self.cards[row][col] {
                    if &c == card && self.chips[row][col] == None {
                        squares.push(Square { row, col })
                    }
                }
            }
        }
        squares
    }

    pub fn chip_at(&self, square: &Square) -> Option<Team> {
        return self.chips[square.row][square.col]
    }

    pub fn belongs_to(&self, square: &Square, team: Team) -> bool {
        Board::is_corner(square) || self.chip_at(square) == Some(team)
    }

    pub fn in_sequence(&self, square: &Square) -> bool {
        self.sequences.iter().any(|(_, seq)| seq.contains(square))
    }

    pub fn remove_chip(&mut self, square: &Square) {
        if !Board::is_playable(square) {
            panic!("attempted to place chip at non-playable square {square}")
        }

        if self.in_sequence(square) {
            panic!("attempted to remove a chip in a sequence")
        }

        self.chips[square.row][square.col] = None
    }

    // returns the number of sequences owned by team if new one(s) were created
    pub fn add_chip(&mut self, square: &Square, team: Team) -> Option<usize> {
        if !Board::is_playable(square) {
            panic!("attempted to place chip at non-playable square {square}")
        }

        self.chips[square.row][square.col] = Some(team);
        self.find_new_sequences(square, team)
    }

    pub fn print(&self) {
        self.print_with_highlighted_cards(vec![])
    }

    pub fn print_with_highlighted_cards(&self, cards: Vec<Card>) {
        for (i, row) in self.cards.iter().enumerate() {
            for (j, card) in row.iter().enumerate() {
                let square = Square { row: i, col: j };
                let card_str = match card {
                    None => String::from("--"),
                    Some(c) => format!("{}", c),
                };
                let base = format!("{square} {card_str}");
                let colored = match self.chips[i][j] {
                    None => {
                        if let Some(card) = self.cards[i][j] {
                            if cards.contains(&card) {
                                format!("\x1b[33m{base}\x1b[39m")
                            } else {
                                base
                            }
                        } else {
                            base
                        }
                    },
                    Some(team) => {
                        team.with_team_color(base, self.in_sequence(&square))
                    },
                };

                print!("| {colored} ")
            }
            println!("|")
        }
    }

    // returns the number of sequences owned by team if new one(s) were created
    fn find_new_sequences(&mut self, source_square: &Square, team: Team) -> Option<usize> {
        let mut added_sequence = false;

        let directions = [
            (0i8, 1i8),
            (1i8, 1i8),
            (1i8, 0i8),
            (1i8, -1i8),
        ];

        for (row_delta, col_delta) in directions {
            // TODO if overlapping with two sequences in different directions, this always puts the
            //  overlap with the one in the "forward" direction; this is moot in practice since only
            //  two sequences can happen, but might cause weirdness if attempting 3 sequences

            let mut used_existing_sequence = false;
            let mut length = 1;
            let mut squares = vec![*source_square];

            // returns true if we can keep continuing in that direction, false to stop
            let mut check = |square| -> bool {
                if Board::is_valid(&square) && self.belongs_to(&square, team) {
                    if self.in_sequence(&square) {
                        if used_existing_sequence { return false; }
                        used_existing_sequence = true;
                    }

                    length += 1;
                    if !Board::is_corner(&square) {
                        squares.push(square);
                    }
                    true
                } else {
                    false
                }
            };

            // go forward along the direction
            for distance in 1..SEQUENCE_LENGTH {
                let square = source_square.plus(
                    row_delta * distance as i8,
                    col_delta * distance as i8,
                );

                if !check(square) { break; }
            }

            // go backward along the direction
            for distance in 1..SEQUENCE_LENGTH {
                let square = source_square.plus(
                    -row_delta * distance as i8,
                    -col_delta * distance as i8,
                );

                if !check(square) { break; }
            }

            if length >= SEQUENCE_LENGTH {
                self.sequences.push((team, squares));
                added_sequence = true;
            }
        }

        if added_sequence {
            Some(self.sequences.iter().filter(|(sequence_team, _)| sequence_team == &team).count())
        } else {
            None
        }
    }
}

pub fn standard_board() -> Board {
    Board {
        chips: [[None; BOARD_SIZE]; BOARD_SIZE],
        sequences: Vec::new(),
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    impl Board {
        fn assert_sequences(&self, mut sequences: Vec<Vec<&str>>) {
            let actual: Vec<Vec<String>> = self.sequences.iter()
                .map(|(_, seq)| {
                    // convert each sequence to a string and sort them for normalization
                    let mut formatted = seq.iter()
                        .map(|square| format!("{}", square))
                        .collect::<Vec<String>>();
                    formatted.sort();
                    formatted
                })
                .collect();
            // sort each incoming sequence for normalization
            for sequence in sequences.iter_mut() { sequence.sort(); }

            assert_eq!(actual, sequences);
        }
    }

    #[test]
    fn standard_board_has_two_copies_of_each_card_except_jacks() {
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

        for card in Card::new_deck() {
            let expected = if card.rank == Rank::JACK { None } else { Some(&2) };
            let actual = counts.get(&card);
            assert_eq!(expected, actual, "Card count was unexpected for {}", card);
        }
    }

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

    #[test]
    fn create_horizontal_sequence_in_order() {
        let mut board = standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::ONE));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("c8"), Team::ONE));
        board.assert_sequences(vec![vec!["c4", "c5", "c6", "c7", "c8"]]);
    }

    #[test]
    fn create_vertical_sequence_with_middle_last() {
        let mut board = standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("h1"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("h2"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("h4"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("h5"), Team::ONE));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("h3"), Team::ONE));
        board.assert_sequences(vec![vec!["h1", "h2", "h3", "h4", "h5"]]);
    }

    #[test]
    fn create_diagonal_sequence_using_corner() {
        let mut board = standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("i8"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("h7"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("f5"), Team::ONE));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("g6"), Team::ONE));
        board.assert_sequences(vec![vec!["i8", "h7", "f5", "g6"]]);
    }

    #[test]
    fn create_horizontal_run_with_another_team_blocking() {
        let mut board = standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::TWO));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c8"), Team::ONE));
        board.assert_sequences(vec![]);
    }

    #[test]
    fn create_two_sequences_without_overlap() {
        let mut board = standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::ONE));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("c8"), Team::ONE));
        board.assert_sequences(vec![vec!["c4", "c5", "c6", "c7", "c8"]]);

        assert_eq!(None, board.add_chip(&Square::from_notation("h1"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("h2"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("h4"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("h5"), Team::ONE));
        assert_eq!(Some(2), board.add_chip(&Square::from_notation("h3"), Team::ONE));
        board.assert_sequences(
            vec![
                vec!["c4", "c5", "c6", "c7", "c8"],
                vec!["h1", "h2", "h3", "h4", "h5"],
            ]
        );
    }

    #[test]
    fn create_two_sequences_with_single_overlap_in_different_direction() {
        let mut board = standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::ONE));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("c8"), Team::ONE));
        board.assert_sequences(vec![vec!["c4", "c5", "c6", "c7", "c8"]]);

        assert_eq!(None, board.add_chip(&Square::from_notation("a5"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("b5"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("d5"), Team::ONE));
        assert_eq!(Some(2), board.add_chip(&Square::from_notation("e5"), Team::ONE));
        board.assert_sequences(
            vec![
                vec!["c4", "c5", "c6", "c7", "c8"],
                vec!["a5", "b5", "c5", "d5", "e5"],
            ]
        );
    }

    #[test]
    fn create_two_sequences_without_single_overlap_in_same_direction() {
        let mut board = standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c0"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c1"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c2"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c3"), Team::ONE));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("c4"), Team::ONE));
        board.assert_sequences(vec![vec!["c0", "c1", "c2", "c3", "c4"]]);

        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::ONE));
        assert_eq!(Some(2), board.add_chip(&Square::from_notation("c8"), Team::ONE));
        board.assert_sequences(
            vec![
                vec!["c0", "c1", "c2", "c3", "c4"],
                vec!["c4", "c5", "c6", "c7", "c8"],
            ]
        );
    }

    // TODO this just creates an extra-long sequence, which protects all squares in it (and prevents
    //  them from being extended into a second sequence), but according to
    //  https://boardgamegeek.com/thread/525241/article/5027571#5027571, the creating team/player
    //  should declare which 5 squares are in the sequence. This is fairly involved to do correctly.
    #[test]
    fn create_sequence_with_more_than_5_squares() {
        let mut board = standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("b0"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c0"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("d0"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("f0"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("g0"), Team::ONE));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("e0"), Team::ONE));
        board.assert_sequences(vec![vec!["b0", "c0", "d0", "e0", "f0", "g0"]]);
    }

    #[test]
    fn adding_chip_at_end_of_sequence_does_not_lengthen_it() {
        let mut board = standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("b0"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("c0"), Team::ONE));
        assert_eq!(None, board.add_chip(&Square::from_notation("d0"), Team::ONE));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("e0"), Team::ONE));
        board.assert_sequences(vec![vec!["b0", "c0", "d0", "e0"]]);

        assert_eq!(None, board.add_chip(&Square::from_notation("f0"), Team::ONE));
        board.assert_sequences(vec![vec!["b0", "c0", "d0", "e0"]]);
    }
}