use crate::core::board::Board;
use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::player::Player;
use crate::core::square::Square;
use crate::core::team::Team;

// convenience wrapper around Player which can be implemented instead to only return the square on
// which to play
// this adds logic to find the correct card to play based on the given square (i.e. play a normal
// card if possible, otherwise the right kind of jack), at the cost of some performance
pub trait SimplePlayer {
    fn play_square(&self, team: &Team, hand: &[Card], board: &Board, deck: &Deck) -> Square;
}

impl<T> Player for T where T: SimplePlayer {
    fn play(&self, team: &Team, hand: &[Card], board: &Board, deck: &Deck) -> (u8, Square) {
        let square = self.play_square(team, hand, board, deck);

        let index = if board.chip_at(&square).is_some() {
            // if there is a chip on the square, find a one-eyed jack to remove it (assumes that
            // it is a chip from another team; if not this will be caught by game logic)
            hand.into_iter().position(|card| card.is_one_eyed_jack()).unwrap()
        } else {
            hand.iter()
                // find index of regular card which can be played on the requested square
                .position(|card| {
                    board.squares_for_card(card).map_or(false, |squares| squares.contains(&square))
                })
                // if no such regular card, find the index of a two-eyed jack
                .unwrap_or_else(|| {
                    hand.iter().position(|card| card.is_two_eyed_jack()).unwrap()
                })
        };

        (index as u8, square)
    }
}
