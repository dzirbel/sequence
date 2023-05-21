use rand::thread_rng;
use rand::seq::IteratorRandom;

use crate::game::board::Board;
use crate::game::card::Card;
use crate::game::deck::Deck;
use crate::game::player::Player;
use crate::game::square::Square;
use crate::game::team::Team;

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
fn rand_occupied_square_not_in_sequence(board: &Board, excluding_team: &Team) -> Square {
    Square::playable_squares()
        .filter(|square| {
            board.chip_at(square).map_or(false, |team| &team != excluding_team) &&
                !board.in_sequence(square)
        })
        .choose(&mut thread_rng())
        .unwrap()
}

impl Player for RandomPlayer {
    fn play(&self, team: &Team, hand: &Vec<Card>, board: &Board, deck: &Deck) -> (u8, Square) {
        // choose a random card
        let (card_index, card) = hand.iter().enumerate().choose(&mut thread_rng()).unwrap();

        if card.is_one_eyed_jack() {
            if board.is_empty() {
                // attempted to play a one-eyed jack on an empty board; try again
                return self.play(team, hand, board, deck);
            }
            (card_index as u8, rand_occupied_square_not_in_sequence(board, team))
        } else if card.is_two_eyed_jack() {
            if board.is_full() {
                // attempted to play a two-eyed jack on a full board; try again
                return self.play(team, hand, board, deck);
            }
            (card_index as u8, rand_unoccupied_square(board))
        } else {
            let squares = board.unoccupied_squares_for_card(&card);
            let square_choice = squares.choose(&mut thread_rng());
            if let Some(square) = square_choice {
                (card_index as u8, square)
            } else {
                // attempted to play a dead card; try again
                self.play(team, hand, board, deck)
            }
        }
    }
}
