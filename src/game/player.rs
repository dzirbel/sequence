use crate::game::board::Board;
use crate::game::card::Card;
use crate::game::square::Square;
use crate::game::team::Team;

pub trait Player {
    // return <card index in hand, <row, col>>
    // where <row, col> is the square on the board being modified:
    // - for regular cards and two-eyed jacks, claims that square
    //   - for regular cards, requires that the card matches the square
    // - for one-eyed jacks, removes the claim on that square
    //   - requires that the square is claimed by a different team
    // TODO ideally pass in info about the deck as well (draw size, discard pile)
    fn play(&self, team: &Team, board: &Board, hand: &Vec<Card>) -> (usize, Square);

    // by default, just pick the first dead card and return it
    fn replace_dead_card(&self, team: &Team, board: &Board, hand: &Vec<Card>) -> Option<usize> {
        for (i, card) in hand.iter().enumerate() {
            if board.is_dead(card) {
                return Some(i)
            }
        }

        panic!("no dead card found")
    }
}
