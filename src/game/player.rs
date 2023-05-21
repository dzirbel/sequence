use crate::game::board::Board;
use crate::game::card::Card;
use crate::game::deck::Deck;
use crate::game::square::Square;
use crate::game::team::Team;

pub trait Player {
    // return <card index in hand, square on which to play the card>
    // - for regular cards and two-eyed jacks, claims that square
    //   - for regular cards, requires that the card matches the square
    // - for one-eyed jacks, removes the claim on that square
    //   - requires that the square is claimed by a different team
    // note that the board and deck are passed as immutable so they cannot be improperly used
    fn play(&self, team: &Team, hand: &Vec<Card>, board: &Board, deck: &Deck) -> (u8, Square);

    fn replace_dead_card(&self, board: &Board, hand: &Vec<Card>) -> Option<usize> {
        // by default, just pick the first dead card and return it
        hand.iter()
            .enumerate()
            .find_map(|(index, card)| if board.is_dead(card) { Some(index) } else { None })
    }
}
