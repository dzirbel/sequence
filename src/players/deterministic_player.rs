use crate::core::board::Board;
use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::player::Player;
use crate::core::square::Square;
use crate::core::team::Team;

pub struct DeterministicPlayer {}

impl Player for DeterministicPlayer {
    fn play(&self, team: &Team, hand: &[Card], board: &Board, _deck: &Deck) -> (u8, Square) {
        let (index, card) = hand.iter()
            .enumerate()
            .find(|(_, card)| board.can_be_played(card, team))
            .unwrap();

        let square = if card.is_one_eyed_jack() {
            Square::playable_squares()
                .find(|square| board.chip_at(square).map_or(false, |t| t != *team))
                .unwrap()
        } else if card.is_two_eyed_jack() {
            Square::playable_squares()
                .find(|square| board.chip_at(square).is_none())
                .unwrap()
        } else {
            *board.squares_for_card(card)
                .unwrap()
                .iter()
                .find(|square| board.chip_at(square).is_none())
                .unwrap()
        };

        (index as u8, square)
    }
}
