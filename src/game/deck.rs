use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::game::card::Card;

pub struct Deck {
    draw_pile: Vec<Card>,

    // note: the game rules specify that each player keeps their own (visible) discard pile, but
    // since they are all shuffled together into the new pile when exhausted, a single discard pile
    // appears to be equivalent (except perhaps for seeing which players played which cards, given
    // that dead cards could interrupt the order)
    discard_pile: Vec<Card>,
}

impl Deck {
    pub fn draw(&mut self) -> Card {
        // reshuffle the discard into the draw pile if empty
        if self.draw_pile.is_empty() {
            self.draw_pile.append(&mut self.discard_pile);
            self.draw_pile.shuffle(&mut thread_rng());
            self.discard_pile.clear();
        }

        if let Some(card) = self.draw_pile.pop() { card } else { panic!("empty deck") }
    }

    pub fn draw_pile_size(&self) -> usize {
        self.draw_pile.len()
    }

    pub fn discard_pile(&self) -> &[Card] {
        return &self.discard_pile;
    }

    pub fn discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }

    pub fn new() -> Deck {
        // sequence deck contains two copies of a standard deck, shuffled
        let mut draw_pile: Vec<Card> = Card::standard_deck().chain(Card::standard_deck()).collect();
        draw_pile.shuffle(&mut thread_rng());

        Deck { draw_pile, discard_pile: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Deck {
        fn assert_draw_pile_contains(&self, cards: &Vec<Card>) {
            let mut draw_pile_sorted = self.draw_pile.clone();
            draw_pile_sorted.sort();

            let mut cards_sorted = cards.clone();
            cards_sorted.sort();

            assert!(*draw_pile_sorted == cards_sorted);
        }
    }

    #[test]
    fn new_deck_has_two_of_each_card() {
        let deck = Deck::new();
        deck.assert_draw_pile_contains(
            &Card::standard_deck().chain(Card::standard_deck()).collect()
        );
        assert!(deck.discard_pile().is_empty());
    }

    #[test]
    fn deck_is_reshuffled_when_exhausted() {
        let mut deck = Deck::new();

        let mut discarded_cards = vec![];
        while deck.draw_pile_size() > 0 {
            let card = deck.draw();
            deck.discard(card);
            discarded_cards.push(card);
        }

        assert_eq!(0, deck.draw_pile_size());
        assert!(discarded_cards == deck.discard_pile);
        let last_card = deck.draw();

        let mut deck_minus_last_card = Deck::new().draw_pile;
        let last_card_index = deck_minus_last_card.iter().position(|c| c == &last_card).unwrap();
        deck_minus_last_card.remove(last_card_index);
        deck.assert_draw_pile_contains(&deck_minus_last_card);
        assert!(deck.discard_pile().is_empty());
    }
}
