use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use rand::seq::IteratorRandom;

use crate::game::board::{Board, BOARD_SIZE};
use crate::game::card::Card;
use crate::game::player::Player;
use crate::game::square::Square;
use crate::game::team::Team;

pub struct RandomPlayer {}

fn rand(max: u8) -> u8 {
    return Uniform::from(0..max).sample(&mut thread_rng())
}

fn rand_square() -> Square {
    let row = rand(BOARD_SIZE);
    let col = rand(BOARD_SIZE);
    let square = Square { row, col };

    // if a corner was chosen, try again
    // TODO just choose (uniformly!) among the playable squares
    if !square.is_playable() {
        return rand_square()
    }

    square
}

fn rand_unoccupied_square(board: &Board) -> Square {
    loop {
        let square = rand_square();
        if let None = board.chip_at(&square) {
            return square
        }
    }
}

fn rand_occupied_square_not_in_sequence(board: &Board, excluding_team: &Team) -> Square {
    loop {
        let square = rand_square();
        if let Some(team) = board.chip_at(&square) {
            if &team != excluding_team && !board.in_sequence(&square) {
                return square
            }
        }
    }
}

impl Player for RandomPlayer {
    fn play(&self, team: &Team, board: &Board, hand: &Vec<Card>) -> (u8, Square) {
        // choose a random card
        let card_index = rand(hand.len() as u8);
        let card = hand[card_index as usize];

        if card.is_one_eyed_jack() {
            if board.is_empty() {
                // attempted to play a one-eyed jack on an empty board; try again
                return self.play(team, board, hand);
            }
            (card_index, rand_occupied_square_not_in_sequence(board, team))
        } else if card.is_two_eyed_jack() {
            if board.is_full() {
                // attempted to play a two-eyed jack on a full board; try again
                return self.play(team, board, hand);
            }
            (card_index, rand_unoccupied_square(board))
        } else {
            let squares = board.unoccupied_squares_for_card(&card);
            let square_choice = squares.choose(&mut thread_rng());
            if let Some(square) = square_choice {
                (card_index, square)
            } else {
                // attempted to play a dead card; try again
                self.play(team, board, hand)
            }
        }
    }
}
