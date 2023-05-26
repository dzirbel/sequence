use rand::thread_rng;
use rand::seq::IteratorRandom;

use crate::core::board::Board;
use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::player::Player;
use crate::core::square::Square;
use crate::core::team::Team;

pub struct RandomPlayer {}

fn rand_unoccupied_square(board: &Board) -> Square {
    Square::playable_squares()
        .filter(|square| board.chip_at(square).is_none())
        .choose(&mut thread_rng())
        .unwrap()
}

// returns a random square among those owned by teams other than the given excluding_team and which
// is not in a sequence
// i.e. squares which have a chip that can be removed by the given team
pub fn rand_occupied_square_not_in_sequence(board: &Board, excluding_team: &Team) -> Option<Square> {
    Square::playable_squares()
        .filter(|square| {
            board.chip_at(square).map_or(false, |team| &team != excluding_team) &&
                !board.in_sequence(square)
        })
        .choose(&mut thread_rng())
}

impl Player for RandomPlayer {
    // TODO avoid recursion for invalid cases for performance
    fn play(&self, team: &Team, hand: &[Card], board: &Board, _deck: &Deck) -> (u8, Square) {
        // choose a random card
        let (card_index, card) = hand.iter().enumerate().choose(&mut thread_rng()).unwrap();

        if card.is_one_eyed_jack() {
            if board.is_empty() {
                // attempted to play a one-eyed jack on an empty board; try again
                return self.play(team, hand, board, _deck);
            }

            if let Some(square) = rand_occupied_square_not_in_sequence(board, team) {
                (card_index as u8, square)
            } else {
                // board is not empty but only has chips from this team; try again
                self.play(team, hand, board, _deck)
            }
        } else if card.is_two_eyed_jack() {
            if board.is_full() {
                // attempted to play a two-eyed jack on a full board; try again
                return self.play(team, hand, board, _deck);
            }
            (card_index as u8, rand_unoccupied_square(board))
        } else {
            let squares = board.squares_for_card(card).unwrap();
            let square_choice = squares
                .iter()
                .filter(|square| board.chip_at(square).is_none())
                .choose(&mut thread_rng());
            if let Some(square) = square_choice {
                (card_index as u8, *square)
            } else {
                // attempted to play a dead card; try again
                self.play(team, hand, board, _deck)
            }
        }
    }
}
