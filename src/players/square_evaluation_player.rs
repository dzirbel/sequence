use std::cmp;
use std::collections::HashSet;

use itertools::Itertools;

use crate::core::board::{Board, SEQUENCE_LENGTH};
use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::player::Player;
use crate::core::square::Square;
use crate::core::team::Team;
use crate::players::random_player::rand_occupied_square_not_in_sequence;
use crate::util::wrapper::Wrapper;

pub struct SquareEvaluationPlayer {
    pub two_eyed_jack_cutoff: i32,
}

impl Default for SquareEvaluationPlayer {
    fn default() -> Self {
        SquareEvaluationPlayer {
            two_eyed_jack_cutoff: 50,
        }
    }
}

impl Board {
    fn normally_playable_squares(&self, cards: &[Card]) -> HashSet<Square> {
        let mut squares = HashSet::new();
        for card in cards {
            if let Some(card_squares) = self.squares_for_card(card) {
                for square in card_squares {
                    squares.insert(square);
                }
            }
        }

        squares
    }
}

impl SquareEvaluationPlayer {
    fn evaluate_empty_square(square: &Square, team: &Team, hand: &[Card], board: &Board) -> i32 {
        let mut score = 0;
        let normal_squares: HashSet<Square> = board.normally_playable_squares(hand);

        let directions = [
            (0i8, 1i8),
            (1i8, 1i8),
            (1i8, 0i8),
            (1i8, -1i8),
        ];

        // TODO consolidate traversal logic
        for (row_delta, col_delta) in directions {
            for i in 1..=SEQUENCE_LENGTH {
                let square = square.plus((i as i8) * row_delta, (i as i8) * col_delta);
                if !square.is_valid() { break }

                let chip = board.chip_at(&square);
                if square.is_corner() || chip.wraps(team) {
                    score += 10;
                } else if chip.is_some() {
                    break;
                } else if normal_squares.contains(&square) {
                    score += 5;
                }
            }

            for i in 1..=SEQUENCE_LENGTH {
                let square = square.plus(-(i as i8) * row_delta, -(i as i8) * col_delta);
                if !square.is_valid() { break }

                let chip = board.chip_at(&square);
                if square.is_corner() || chip.wraps(team) {
                    score += 10;
                } else if chip.is_some() {
                    break;
                } else if normal_squares.contains(&square) {
                    score += 5;
                }
            }
        }

        score
    }

    #[allow(dead_code)] // TODO extract and test
    fn sequence_distance(square1: &Square, square2: &Square) -> Option<u8> {
        let row_diff = square1.row.abs_diff(square2.row);
        let col_diff = square1.col.abs_diff(square2.col);
        if row_diff == 0 || col_diff == 0 || row_diff == col_diff {
            Some(cmp::max(row_diff, col_diff))
        } else {
            None
        }
    }
}

impl Player for SquareEvaluationPlayer {
    // TODO play one-eyed jacks if evaluation of a square for another team is above a threshold
    fn play(&self, team: &Team, hand: &[Card], board: &Board, _deck: &Deck) -> (u8, Square) {
        let normal_squares: HashSet<Square> = board.normally_playable_squares(hand);

        let two_eyed_jack_index: Option<usize> = hand.iter()
            .enumerate()
            .find(|(_, card)| card.is_two_eyed_jack())
            .map(|(index, _)| index);

        // TODO these are already iterators, no need to collect/clone and then re-iterate them
        let playable_squares = if two_eyed_jack_index.is_some() {
            Square::playable_squares().collect()
        } else {
            normal_squares.clone()
        };

        // set of squares with tied maximum evaluation
        let best_squares: HashSet<Square> = HashSet::from_iter(
            playable_squares
                .iter()
                .filter(|square| board.chip_at(square).is_none())
                .map(|square| {
                    let evaluation = SquareEvaluationPlayer::evaluate_empty_square(
                        square, team, hand, board);
                    (square, evaluation)
                })
                .filter(|(square, evaluation)| {
                    *evaluation >= self.two_eyed_jack_cutoff || normal_squares.contains(square)
                })
                .max_set_by_key(|(_, evaluation)| *evaluation)
                .iter()
                .map(|(square, _)| **square)
        );

        // find an arbitrary normal card which can be played on one of the best squares, if there is
        // one
        hand.iter()
            .enumerate()
            .find_map(|(index, card)| {
                board.squares_for_card(card)
                    .iter()
                    .filter_map(|squares| squares.intersection(&best_squares).next())
                    .next()
                    .map(|square| (index as u8, *square))
            })
            .unwrap_or_else(|| {
                if two_eyed_jack_index.is_none() || board.is_full() {
                    // no two eyed jack and no normal card can be played -> only have dead cards and
                    // one eyed jacks; for now play a one-eyed jack on a random square

                    let one_eyed_jack_index: usize = hand.iter()
                        .enumerate()
                        .find(|(_, card)| card.is_one_eyed_jack())
                        .map(|(index, _)| index)
                        .unwrap();
                    let square = rand_occupied_square_not_in_sequence(board, team)
                        .unwrap();

                    return (one_eyed_jack_index as u8, square);
                }

                let index = two_eyed_jack_index.unwrap() as u8;
                let square = best_squares.into_iter().next().unwrap_or_else(|| {
                    // no best squares -> only playable card is a two-eyed jack, but no square met
                    // the threshold to play a two eyed jack; choose an arbitrary square
                    Square::playable_squares()
                        .find(|square| board.chip_at(square).is_none())
                        .unwrap()
                });
                (index, square)
            })
    }
}
