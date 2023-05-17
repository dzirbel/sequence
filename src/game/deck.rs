use strum::IntoEnumIterator;

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::game::card::Card;
use crate::game::rank::Rank;
use crate::game::suit::Suit;

pub struct Deck {
    draw_pile: Vec<Card>,

    // note: the game rules specify that each player keeps their own (visible) discard pile, but
    // since they are all shuffled together into the new pile when exhausted, a single discard pile
    // appears to be equivalent
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

    pub fn size(self) -> usize {
        self.draw_pile.len()
    }

    pub fn discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }

    pub fn new() -> Deck {
        Deck {
            draw_pile: Suit::iter()
                .flat_map(|suit| Rank::iter().map(move |rank| Card { suit, rank }))
                // copy each card twice in the deck
                .flat_map(|card| vec![card, card])
                .collect(),
            discard_pile: Vec::new(),
        }
    }
}

