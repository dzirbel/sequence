use std::collections::HashSet;

use crate::core::card::Card;
use crate::core::rank::Rank;
use crate::core::square::Square;
use crate::core::team::Team;
use crate::util::wrapper::Wrapper;

pub const BOARD_SIZE: u8 = 10;
pub const SEQUENCE_LENGTH: u8 = 5;

pub struct Board {
    // TODO also have / replace with a hashmap from card to square for performance
    pub cards: [[Option<Card>; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    pub chips: [[Option<Team>; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    pub sequences: Vec<(Team, HashSet<Square>)>,
}

impl Board {
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
        Square::playable_squares().all(|square| self.chip_at(&square).is_some())
    }

    // returns true if there are no chips on the board
    pub fn is_empty(&self) -> bool {
        Square::playable_squares().all(|square| self.chip_at(&square).is_none())
    }

    // checks if the given card is dead, i.e. not a Jack and both of its squares already have a chip
    pub fn is_dead(&self, card: &Card) -> bool {
        if card.rank == Rank::Jack { return false; }

        !Square::playable_squares()
            .any(|square| self.card_at(&square).wraps(card) && self.chip_at(&square).is_none())
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

    // note: returns empty iterator for jacks
    pub fn unoccupied_squares_for_card<'a>(&'a self, card: &'a Card) -> impl Iterator<Item=Square> + 'a {
        Square::playable_squares()
            .filter(|square| {
                self.card_at(square).wraps(card) && self.chip_at(square).is_none()
            })
    }

    // returns true if the given team has a chip at the given square, or it is a corner square
    pub fn counts_for(&self, square: &Square, team: &Team) -> bool {
        square.is_corner() || self.chip_at(square).wraps(team)
    }

    // returns true if the given square is in a sequence
    pub fn in_sequence(&self, square: &Square) -> bool {
        self.sequences.iter().any(|(_, seq)| seq.contains(square))
    }

    pub fn remove_chip(&mut self, square: &Square) {
        if !square.is_playable() {
            panic!("attempted to remove chip at non-playable square {square}")
        }

        if self.in_sequence(square) {
            panic!("attempted to remove a chip in a sequence")
        }

        self.chips[square.row as usize][square.col as usize] = None
    }

    // returns the number of sequences owned by team if new one(s) were created
    pub fn add_chip(&mut self, square: &Square, team: Team) -> Option<usize> {
        if !square.is_playable() {
            panic!("attempted to place chip at non-playable square {square}")
        }

        if self.chip_at(square).is_some() {
            panic!("attempted to place chip with a chip already present at {square}")
        }

        self.chips[square.row as usize][square.col as usize] = Some(team);
        self.find_new_sequences(square, team)
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

            let mut used_existing_sequence = false;
            let mut length = 1;
            let mut squares = HashSet::new();
            squares.insert(*source_square);

            // returns true if we can keep continuing in that direction, false to stop
            let mut check = |square: Square| -> bool {
                if square.is_valid() && self.counts_for(&square, &team) {
                    if self.in_sequence(&square) {
                        if used_existing_sequence { return false; }
                        used_existing_sequence = true;
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
    fn create_horizontal_sequence_in_order() {
        let mut board = Board::new();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("c8"), Team::One));
        board.assert_sequences(vec![vec!["c4", "c5", "c6", "c7", "c8"]]);
    }

    #[test]
    fn create_vertical_sequence_with_middle_last() {
        let mut board = Board::new();
        assert_eq!(None, board.add_chip(&Square::from_notation("h1"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h2"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h5"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("h3"), Team::One));
        board.assert_sequences(vec![vec!["h1", "h2", "h3", "h4", "h5"]]);
    }

    #[test]
    fn create_diagonal_sequence_using_corner() {
        let mut board = Board::new();
        assert_eq!(None, board.add_chip(&Square::from_notation("i8"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("h7"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("f5"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("g6"), Team::One));
        board.assert_sequences(vec![vec!["i8", "h7", "f5", "g6"]]);
    }

    #[test]
    fn create_horizontal_run_with_another_team_blocking() {
        let mut board = Board::new();
        assert_eq!(None, board.add_chip(&Square::from_notation("c4"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c5"), Team::Two));
        assert_eq!(None, board.add_chip(&Square::from_notation("c6"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c7"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c8"), Team::One));
        board.assert_sequences(vec![]);
    }

    #[test]
    fn create_two_sequences_without_overlap() {
        let mut board = Board::new();
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
        let mut board = Board::new();
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
    fn create_two_sequences_without_single_overlap_in_same_direction() {
        let mut board = Board::new();
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
        let mut board = Board::new();
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
        let mut board = Board::new();
        assert_eq!(None, board.add_chip(&Square::from_notation("b0"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("c0"), Team::One));
        assert_eq!(None, board.add_chip(&Square::from_notation("d0"), Team::One));
        assert_eq!(Some(1), board.add_chip(&Square::from_notation("e0"), Team::One));
        board.assert_sequences(vec![vec!["b0", "c0", "d0", "e0"]]);

        assert_eq!(None, board.add_chip(&Square::from_notation("f0"), Team::One));
        board.assert_sequences(vec![vec!["b0", "c0", "d0", "e0"]]);
    }
}
