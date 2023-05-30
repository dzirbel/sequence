use std::cmp;
use std::collections::HashSet;

use itertools::Itertools;

use crate::core::board::Board;
use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::grid_traversal::open_runs_for_team;
use crate::core::simple_player::SimplePlayer;
use crate::core::square::Square;
use crate::core::team::Team;
use crate::players::random_player::rand_occupied_square_not_in_sequence;

pub struct SquareEvaluationPlayer {
    pub two_eyed_jack_cutoff: i32,
}

impl Default for SquareEvaluationPlayer {
    fn default() -> Self {
        SquareEvaluationPlayer {
            two_eyed_jack_cutoff: 100,
        }
    }
}

impl Board {
    fn normally_playable_squares(&self, cards: &[Card]) -> HashSet<Square> {
        cards.iter()
            .flat_map(|card| self.squares_for_card(card))
            .flatten()
            .collect()
    }
}

impl SquareEvaluationPlayer {
    fn evaluate_empty_square(square: &Square, team: &Team, hand: &[Card], board: &Board) -> i32 {
        let mut score: i32 = 0;
        let normal_squares: HashSet<Square> = board.normally_playable_squares(hand);

        // TODO if the run has length less than the max sequence length, give it no points
        for run in open_runs_for_team(board, square, team) {
            for run_square in run {
                let value: f32 = if board.counts_for(&run_square, team) {
                    10.0
                } else if normal_squares.contains(&run_square) {
                    2.5
                } else {
                    0.5
                };

                let dist = SquareEvaluationPlayer::sequence_distance(square, &run_square).unwrap() as f32;
                let dist_multiplier = (dist - 1.0) * 0.05;
                score += (value / (1.0 + dist_multiplier)) as i32;
            }
        }

        score
    }

    // TODO extract and test
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

impl SimplePlayer for SquareEvaluationPlayer {
    // TODO play one-eyed jacks if evaluation of a square for another team is above a threshold
    fn play_square(&self, team: &Team, hand: &[Card], board: &Board, _deck: &Deck) -> Square {
        // if there are no open squares, return a random square for a one-eyed jack to remove
        if board.is_full() {
            return rand_occupied_square_not_in_sequence(board, team).unwrap();
        }

        let normal_squares: HashSet<Square> = board.normally_playable_squares(hand);

        let two_eyed_jack_index: Option<usize> = hand.iter()
            .position(|card| card.is_two_eyed_jack());

        let playable_squares: Box<dyn Iterator<Item=Square>> =
            if two_eyed_jack_index.is_some() {
                Box::new(Square::playable_squares())
            } else {
                Box::new(normal_squares.clone().into_iter())
            };

        let square_evaluations: Vec<(Square, i32)> = playable_squares
            .filter(|square| board.chip_at(square).is_none())
            .map(|square| {
                (square, SquareEvaluationPlayer::evaluate_empty_square(&square, team, hand, board))
            })
            .collect();

        // set of squares with tied maximum evaluation
        let best_squares: HashSet<Square> = HashSet::from_iter(
            square_evaluations.iter()
                .filter(|(square, evaluation)| {
                    *evaluation >= self.two_eyed_jack_cutoff || normal_squares.contains(square)
                })
                .max_set_by_key(|(_, evaluation)| *evaluation)
                .into_iter()
                .map(|(square, _)| *square)
        );

        // if we could not find any squares to evaluate, all our normal cards are dead. then either:
        // 1. we have a two-eyed jack, but there are no squares with an evaluation above the
        //    threshold
        // 2. we have no two-eyed jack, and must play a one-eyed jack
        if best_squares.is_empty() {
            return if two_eyed_jack_index.is_some() {
                // (1) choose the bets square for the two-eyed jack
                square_evaluations.into_iter()
                    .max_by_key(|(_, evaluation)| *evaluation)
                    .map(|(square, _)| square)
                    .unwrap()
            } else {
                // (2) choose a random square for the one-eyed jack
                rand_occupied_square_not_in_sequence(board, team).unwrap()
            };
        }

        if let Some(square) = best_squares.iter().find(|square| normal_squares.contains(square)) {
            // if any of the best squares is playable by a normal card, play it
            *square
        } else {
            // otherwise, pick an arbitrary best square and play it (with a two-eyed jack)
            best_squares.into_iter().next().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::player::Player;
    use crate::core::rank::Rank;
    use crate::core::suit::Suit;
    use super::*;

    #[test]
    fn plays_card_near_existing_chip() {
        let mut board = Board::standard_board();
        let deck = Deck::default();
        let player = SquareEvaluationPlayer::default();

        board.add_chip(&Square::from_notation("e0"), Team::One);
        // 3 and 4 of clubs are never in line with a corner square
        let hand = vec![
            Card { rank: Rank::Three, suit: Suit::Clubs },
            Card { rank: Rank::Four, suit: Suit::Clubs },
        ];

        let (index, square) = player.play(&Team::One, &hand, &board, &deck);
        assert_eq!(index, 0);
        assert_eq!(square, Square::from_notation("d1"));
    }

    #[test]
    fn plays_card_near_corner() {
        let board = Board::standard_board();
        let deck = Deck::default();
        let player = SquareEvaluationPlayer::default();

        // 3 and 4 of clubs are never in line with a corner square
        let hand = vec![
            Card { rank: Rank::Three, suit: Suit::Clubs },
            Card { rank: Rank::Four, suit: Suit::Clubs },
            Card { rank: Rank::Three, suit: Suit::Diamonds },
        ];

        let (index, square) = player.play(&Team::One, &hand, &board, &deck);
        assert_eq!(index, 2);
        assert_eq!(square, Square::from_notation("j6"));
    }

    #[test]
    fn plays_card_with_more_nearby_chips() {
        let mut board = Board::standard_board();
        let deck = Deck::default();
        let player = SquareEvaluationPlayer::default();

        // run near the 3 of clubs on d1
        board.add_chip(&Square::from_notation("d3"), Team::One);
        board.add_chip(&Square::from_notation("d4"), Team::One);
        board.add_chip(&Square::from_notation("d5"), Team::One);

        // single chip near the 4 of clubs on c1
        board.add_chip(&Square::from_notation("b2"), Team::One);

        let hand = vec![
            Card { rank: Rank::Three, suit: Suit::Clubs },
            Card { rank: Rank::Four, suit: Suit::Clubs },
        ];

        let (index, square) = player.play(&Team::One, &hand, &board, &deck);
        assert_eq!(index, 0);
        assert_eq!(square, Square::from_notation("d1"));
    }

    #[test]
    fn does_not_play_card_near_blocked_run() {
        let mut board = Board::standard_board();
        let deck = Deck::default();
        let player = SquareEvaluationPlayer::default();

        // run up to the 3 of clubs on d1 is blocked
        board.add_chip(&Square::from_notation("d2"), Team::Two);
        board.add_chip(&Square::from_notation("d3"), Team::One);
        board.add_chip(&Square::from_notation("d4"), Team::One);
        board.add_chip(&Square::from_notation("d5"), Team::One);

        // single chip near the 4 of clubs on c1
        board.add_chip(&Square::from_notation("b2"), Team::One);

        // block the second option for the 4 of clubs
        board.add_chip(&Square::from_notation("e3"), Team::Three);

        let hand = vec![
            Card { rank: Rank::Three, suit: Suit::Clubs },
            Card { rank: Rank::Four, suit: Suit::Clubs },
        ];

        let (index, square) = player.play(&Team::One, &hand, &board, &deck);
        assert_eq!(index, 1);
        assert_eq!(square, Square::from_notation("c1"));
    }
}
