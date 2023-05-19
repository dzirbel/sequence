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

    pub fn size(&self) -> usize {
        self.draw_pile.len()
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

