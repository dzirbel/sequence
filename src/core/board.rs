use std::collections::{HashMap, HashSet};

use crate::core::card::Card;
use crate::core::square::Square;
use crate::core::team::Team;
use crate::util::wrapper::Wrapper;

pub const BOARD_SIZE: u8 = 10;
pub const SEQUENCE_LENGTH: u8 = 5;
pub const NUM_PLAYABLE_SQUARES: u8 = BOARD_SIZE * BOARD_SIZE - 4;

// TODO relax requirements on board to allow different sizes and arrangements of wildcard squares
pub struct Board {
    cards: [[Option<Card>; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    chips: [[Option<Team>; BOARD_SIZE as usize]; BOARD_SIZE as usize],

    // map from each type of card (excluding jacks) to the set of squares where it occurs
    card_to_squares: HashMap<Card, HashSet<Square>>,
    // map from each team to the set of squares they have claimed
    team_to_squares: HashMap<Team, HashSet<Square>>,

    // list of fully formed and protected sequences, paired with the team owning them
    sequences: Vec<(Team, HashSet<Square>)>,
    squares_in_sequence: HashSet<Square>,

    // number of chips on the board, to make it faster to check whether it is full/empty
    num_chips: u8,
}

impl Board {
    #[cfg(debug_assertions)]
    fn assert_invariants(&self) {
        for (card, squares) in &self.card_to_squares {
            for square in squares {
                assert_eq!(card, &self.card_at(square).unwrap());
            }
        }

        for square in Square::playable_squares() {
            let card = self.card_at(&square).unwrap();
            assert!(self.card_to_squares.get(&card).unwrap().contains(&square));
        }

        assert_eq!(
            self.num_chips as usize,
            Square::playable_squares().filter(|square| self.chip_at(square).is_some()).count(),
        );

        for (team, sequence) in &self.sequences {
            for square in sequence {
                assert_eq!(team, &self.chip_at(square).unwrap());
                assert!(self.squares_in_sequence.contains(square));
            }
        }

        for square in &self.squares_in_sequence {
            assert!(self.sequences.iter().any(|(_, squares)| squares.contains(square)));
        }

        for (team, squares) in &self.team_to_squares {
            for square in squares {
                assert_eq!(team, &self.chip_at(square).unwrap());
            }
        }

        for square in Square::playable_squares() {
            if let Some(chip) = self.chip_at(&square) {
                assert!(self.team_to_squares.get(&chip).unwrap().contains(&square));
            }
        }
    }

    #[cfg(not(debug_assertions))]
    fn assert_invariants(&self) {}

    pub fn new(cards: [[Option<Card>; BOARD_SIZE as usize]; BOARD_SIZE as usize]) -> Board {
        let mut card_to_squares: HashMap<Card, HashSet<Square>> = HashMap::new();

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(card) = cards[row as usize][col as usize] {
                    card_to_squares.entry(card)
                        .and_modify(|squares| { squares.insert(Square { row, col }); })
                        .or_insert_with(|| HashSet::from([Square { row, col }; 1]));
                }
            }
        }

        let board = Board {
            cards,
            chips: [[None; BOARD_SIZE as usize]; BOARD_SIZE as usize],
            card_to_squares,
            team_to_squares: HashMap::new(),
            sequences: Vec::new(),
            squares_in_sequence: HashSet::new(),
            num_chips: 0,
        };

        board.assert_invariants();

        board
    }

    // returns the card at the given square; None for corners
    pub fn card_at(&self, square: &Square) -> Option<Card> {
        self.cards[square.row as usize][square.col as usize]
    }

    // returns the team which has claimed the given square; None if unclaimed (or a corner)
    pub fn chip_at(&self, square: &Square) -> Option<Team> {
        self.chips[square.row as usize][square.col as usize]
    }

    // returns true if there is a chip on all playable squares
    pub fn is_full(&self) -> bool {
        self.num_chips == NUM_PLAYABLE_SQUARES
    }

    // returns true if there are no chips on the board
    pub fn is_empty(&self) -> bool {
        self.num_chips == 0
    }

    // returns a set of squares on which the given card occurs; None for jacks
    pub fn squares_for_card(&self, card: &Card) -> Option<HashSet<Square>> {
        self.card_to_squares.get(card).cloned()
    }

    // checks if the given card is dead, i.e. not a Jack and both of its squares already have a chip
    pub fn is_dead(&self, card: &Card) -> bool {
        if let Some(squares) = &self.card_to_squares.get(card) {
            squares.iter().all(|square| self.chip_at(square).is_some())
        } else {
            false // no mapping of cards to squares -> this is a jack, which is never dead
        }
    }

    // checks if it is possible to play the given card:
    // - for one-eyed jacks, true if the board is not empty
    // - for two-eyed jacks, true if the board is not full
    // - for regular cards, true if at least one of its squares does not have a chip
    pub fn can_be_played(&self, card: &Card) -> bool {
        if card.is_one_eyed_jack() {
            !self.is_empty()
        } else if card.is_two_eyed_jack() {
            !self.is_full()
        } else {
            !self.is_dead(card)
        }
    }

    // returns true if the given team has a chip at the given square, or it is a corner square
    pub fn counts_for(&self, square: &Square, team: &Team) -> bool {
        square.is_corner() || self.chip_at(square).wraps(team)
    }

    pub fn squares_owned_by(&self, team: &Team) -> HashSet<Square> {
        self.team_to_squares[team].clone()
    }

    // returns true if the given square is in a sequence
    pub fn in_sequence(&self, square: &Square) -> bool {
        self.squares_in_sequence.contains(square)
    }

    pub fn remove_chip(&mut self, square: &Square) {
        debug_assert!(square.is_playable(), "attempted to remove chip at non-playable square {square}");
        debug_assert!(!self.in_sequence(square), "attempted to remove a chip in a sequence");

        let team = self.chips[square.row as usize][square.col as usize]
            .unwrap_or_else(|| panic!("attempted to remove a chip from an un-owned square"));

        self.chips[square.row as usize][square.col as usize] = None;
        self.num_chips -= 1;
        self.team_to_squares.entry(team)
            .and_modify(|squares| { squares.remove(square); });

        self.assert_invariants();
    }

    // returns the number of sequences owned by team if new one(s) were created
    pub fn add_chip(&mut self, square: &Square, team: Team) -> Option<usize> {
        debug_assert!(square.is_playable(), "attempted to place chip at non-playable square {square}");
        debug_assert!(self.chip_at(square).is_none(), "attempted to place chip with a chip already present at {square}");

        self.chips[square.row as usize][square.col as usize] = Some(team);
        self.num_chips += 1;
        self.team_to_squares.entry(team)
            .and_modify(|squares| { squares.insert(*square); })
            .or_insert_with(|| HashSet::from([*square; 1]));

        let result = self.find_new_sequences(square, team);
        self.assert_invariants();
        result
    }

    pub fn print(&self) {
        self.print_with_highlighted_cards(&HashSet::new())
    }

    pub fn print_with_highlighted_cards(&self, cards: &HashSet<Card>) {
        for (i, row) in self.cards.iter().enumerate() {
            for (j, card) in row.iter().enumerate() {
                let square = Square { row: i as u8, col: j as u8 };
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
                    }
                    Some(team) => team.with_team_color(&base, self.in_sequence(&square))
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

            let mut overlap_with_existing_sequence = false;
            let mut length = 1;
            let mut squares = HashSet::new();
            squares.insert(*source_square);

            // returns true if the given square is a valid addition to the sequence
            let mut add_to_sequence = |square: Square| -> bool {
                if square.is_valid() && self.counts_for(&square, &team) {
                    if self.in_sequence(&square) {
                        // allow the first overlap; then abort
                        if overlap_with_existing_sequence { return false; }
                        overlap_with_existing_sequence = true;
                    }

                    length += 1;
                    if !square.is_corner() {
                        squares.insert(square);
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

                if !add_to_sequence(square) { break; }
            }

            // go backward along the direction
            for distance in 1..SEQUENCE_LENGTH {
                let square = source_square.plus(
                    -row_delta * distance as i8,
                    -col_delta * distance as i8,
                );

                if !add_to_sequence(square) { break; }
            }

            if length >= SEQUENCE_LENGTH {
                for square in &squares {
                    self.squares_in_sequence.insert(*square);
                }
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

#[cfg(test)]
mod tests {
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
    fn add_chip() {
        let square = Square::from_notation("e4");
        let mut board = Board::standard_board();

        assert!(board.is_empty());
        assert!(!board.is_full());
        assert!(board.sequences.is_empty());
        assert!(board.team_to_squares.is_empty());
        assert_eq!(board.num_chips, 0);

        board.add_chip(&square, Team::One);

        assert!(!board.is_empty());
        assert!(!board.is_full());
        assert!(board.sequences.is_empty());
        assert_eq!(board.team_to_squares, HashMap::from([(Team::One, HashSet::from([square; 1]))]));
        assert_eq!(board.num_chips, 1);
    }

    #[test]
    fn remove_chip() {
        let square = Square::from_notation("e4");
        let mut board = Board::standard_board();

        board.add_chip(&square, Team::One);
        board.remove_chip(&square);

        assert!(board.is_empty());
        assert!(!board.is_full());
        assert!(board.sequences.is_empty());
        assert_eq!(board.team_to_squares, HashMap::from([(Team::One, HashSet::new())]));
        assert_eq!(board.num_chips, 0);
    }

    #[test]
    fn create_horizontal_sequence_in_order() {
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("c8"), Team::One));
        board.assert_sequences(vec![vec!["c4", "c5", "c6", "c7", "c8"]]);
    }

    #[test]
    fn create_vertical_sequence_with_middle_last() {
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("h1"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h2"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h5"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("h3"), Team::One));
        board.assert_sequences(vec![vec!["h1", "h2", "h3", "h4", "h5"]]);
    }

    #[test]
    fn create_diagonal_sequence_using_corner() {
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("i8"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h7"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("f5"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("g6"), Team::One));
        board.assert_sequences(vec![vec!["i8", "h7", "f5", "g6"]]);
    }

    #[test]
    fn create_horizontal_run_with_another_team_blocking() {
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::Two));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c8"), Team::One));
        board.assert_sequences(vec![]);
    }

    #[test]
    fn create_two_sequences_without_overlap() {
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("c8"), Team::One));
        board.assert_sequences(vec![vec!["c4", "c5", "c6", "c7", "c8"]]);

        assert_eq!(None, board.add_chip(&Square::from_notation("h1"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h2"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h5"), Team::One));
        assert_eq!(Some(2), board.add_chip(&Square::from_notation("h3"), Team::One));
        board.assert_sequences(
            vec![
                vec!["c4", "c5", "c6", "c7", "c8"],
                vec!["h1", "h2", "h3", "h4", "h5"],
            ]
        );
    }

    #[test]
    fn create_two_sequences_with_single_overlap_in_different_direction() {
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("c8"), Team::One));
        board.assert_sequences(vec![vec!["c4", "c5", "c6", "c7", "c8"]]);

        assert_eq!(None, board.add_chip(&Square::from_notation("a5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("b5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("d5"), Team::One));
        assert_eq!(Some(2), board.add_chip(&Square::from_notation("e5"), Team::One));
        board.assert_sequences(
            vec![
                vec!["c4", "c5", "c6", "c7", "c8"],
                vec!["a5", "b5", "c5", "d5", "e5"],
            ]
        );
    }

    #[test]
    fn create_two_sequences_simultaneously_with_overlap_in_different_direction() {
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c8"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("a5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("b5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("d5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("e5"), Team::One));
        board.assert_sequences(vec![]);

        assert_eq!(Some(2), board.add_chip(&Square::from_notation("c5"), Team::One));
        board.assert_sequences(
            vec![
                vec!["a5", "b5", "c5", "d5", "e5"],
                vec!["c4", "c5", "c6", "c7", "c8"],
            ]
        );
    }

    #[test]
    fn create_two_sequences_without_single_overlap_in_same_direction() {
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("c0"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c1"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c2"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c3"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("c4"), Team::One));
        board.assert_sequences(vec![vec!["c0", "c1", "c2", "c3", "c4"]]);

        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::One));
        assert_eq!(Some(2), board.add_chip(&Square::from_notation("c8"), Team::One));
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
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("b0"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c0"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("d0"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("f0"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("g0"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("e0"), Team::One));
        board.assert_sequences(vec![vec!["b0", "c0", "d0", "e0", "f0", "g0"]]);
    }

    #[test]
    fn adding_chip_at_end_of_sequence_does_not_lengthen_it() {
        let mut board = Board::standard_board();
        assert_eq!(None, board.add_chip(&Square::from_notation("b0"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c0"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("d0"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("e0"), Team::One));
        board.assert_sequences(vec![vec!["b0", "c0", "d0", "e0"]]);

        assert_eq!(None, board.add_chip(&Square::from_notation("f0"), Team::One));
        board.assert_sequences(vec![vec!["b0", "c0", "d0", "e0"]]);
    }
}
