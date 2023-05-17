use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

use crate::game::board::{Board, standard_board};
use crate::game::card::Card;
use crate::game::deck::Deck;
use crate::game::player::Player;
use crate::game::team::Team;

pub struct Game {
    players: Vec<Box<dyn Player>>,
    num_teams: usize,
    // specified since 6/12 player games could have 2 or 3 teams
    turn_count: usize,
    up_index: usize,
    player_hands: Vec<Vec<Card>>,
    board: Board,
    deck: Deck,
}

fn generate_vector<T, F>(count: usize, f: F) -> Vec<T> where F: FnMut(usize) -> T {
    (0..count).map(f).collect()
}

impl Game {
    pub fn new(players: Vec<Box<dyn Player>>, num_teams: usize) -> Game {
        // TODO validate num_teams

        let hand_size = Game::hand_size(players.len());

        let mut deck = Deck::new();

        let player_hands = generate_vector(players.len(), |_| {
            generate_vector(hand_size, |_| deck.draw())
        });

        // choose a random player as the first one (in the rules, players cut cards (aces high) and
        // the lowest card deals; the player to the left of the dealer goes first)
        let up_index = Uniform::from(0..players.len()).sample(&mut thread_rng());

        let board = standard_board();

        Game {
            players,
            num_teams,
            up_index,
            player_hands,
            board,
            deck,
            turn_count: 0,
        }
    }

    pub fn run_turn(&mut self, print_turn: bool) -> Option<Team> {
        let player_team = self.player_team(self.up_index);
        let player = &self.players[self.up_index];
        let hand = &self.player_hands[self.up_index];
        let hand_size = hand.len();

        let dead_cards = hand.iter().filter(|card| self.board.is_dead(card)).count();
        let has_one_eyed_jack = hand.iter().any(|card| card.is_one_eyed_jack());
        if dead_cards > 0 {
            let replaced_card_choice = player.replace_dead_card(&player_team, &self.board, hand);
            if let Some(replaced_card_index) = replaced_card_choice {
                if replaced_card_index > hand.len() {
                    panic!(
                        "dead card choice out of bounds: {replaced_card_index} \
                        for hand size {}", hand.len()
                    );
                }

                let replaced_card = &self.player_hands[self.up_index].remove(replaced_card_index);
                self.deck.discard(*replaced_card);
                if !self.board.is_dead(replaced_card) {
                    panic!("replaced card was not dead");
                }

                if print_turn {
                    println!("Turn {}: player {} replaced dead card {}",
                             self.turn_count, self.up_index, replaced_card);
                }

                let new_card = self.deck.draw();
                self.player_hands[self.up_index].push(new_card);

                if (dead_cards == hand_size && self.board.is_dead(&new_card)) ||
                    (self.board.is_full() && !has_one_eyed_jack) {
                    // all cards were dead, and the new drawn card is as well; no playable cards
                    if print_turn {
                        println!("Turn {}: player {} has no playable cards; skipping turn",
                                 self.turn_count, self.up_index);
                    }
                    self.up_index = (self.up_index + 1) % self.players.len();
                    self.turn_count += 1;
                    return None; // skip turn
                }
            } else if dead_cards == hand_size {
                // all cards are dead and player chose not to replace one; no playable cards
                if print_turn {
                    println!("Turn {}: player {} has no playable cards; skipping turn",
                             self.turn_count, self.up_index);
                }
                self.up_index = (self.up_index + 1) % self.players.len();
                self.turn_count += 1;
                return None; // skip turn
            }
        }

        let (choice_index, choice_square) = player.play(
            &player_team,
            &self.board,
            &self.player_hands[self.up_index],
        );

        if choice_index > hand_size {
            panic!("card choice out of bounds: {choice_index} for hand size {}", hand_size)
        }

        if !Board::is_playable(&choice_square) {
            panic!("invalid square choice: {choice_square}")
        }

        let choice_card = &self.player_hands[self.up_index].remove(choice_index);
        self.deck.discard(*choice_card);

        let current_claim = self.board.chip_at(&choice_square);
        if choice_card.is_one_eyed_jack() {
            match current_claim {
                None => panic!("attempted to remove chip on unclaimed square {choice_square}"),
                Some(team) => {
                    if team == player_team {
                        panic!("attempted to remove chip on own team's square {choice_square}")
                    }
                }
            }

            self.board.remove_chip(&choice_square);
            if print_turn {
                println!(
                    "Turn {}: player {} played {}, a one-eyed Jack, and removed the chip on {}",
                    self.turn_count,
                    self.up_index,
                    choice_card,
                    choice_square,
                );
            }
        } else {
            match current_claim {
                Some(_) => panic!("attempted to put chip on claimed square {choice_square}"),
                None => {
                    if !choice_card.is_two_eyed_jack() {
                        if let Some(card) = self.board.card_at(&choice_square) {
                            if choice_card != &card {
                                panic!(
                                    "attempted to put chip on square not matching the \
                                    chosen card {choice_card}: {choice_square}"
                                )
                            }
                        }
                    }
                }
            }

            if print_turn {
                println!(
                    "Turn {}: player {} played {} on {}",
                    self.turn_count,
                    self.up_index,
                    choice_card,
                    choice_square,
                );
            }

            let new_sequence = self.board.add_chip(&choice_square, player_team);
            if let Some(sequences) = new_sequence {
                if sequences >= Game::winning_sequences(self.num_teams) {
                    return Some(player_team)
                }
            }
        }

        self.player_hands[self.up_index].push(self.deck.draw());

        self.up_index = (self.up_index + 1) % self.players.len();
        self.turn_count += 1;

        None
    }

    pub fn print_board(&self) {
        self.board.print();
    }

    fn player_team(&self, player_index: usize) -> Team {
        let team_index = player_index % self.num_teams;
        match team_index {
            0 => Team::ONE,
            1 => Team::TWO,
            2 => Team::THREE,
            _ => panic!("invalid team index (too many teams?)")
        }
    }

    fn winning_sequences(num_teams: usize) -> usize {
        match num_teams {
            2 => 2,
            3 => 1,
            _ => panic!("unsupported number of teams: {num_teams}")
        }
    }

    fn hand_size(num_players: usize) -> usize {
        match num_players {
            2 => 7,
            3 => 6,
            4 => 6,
            6 => 5,
            8 => 4,
            9 => 4,
            10 => 3,
            12 => 3,
            _ => panic!("unsupported number of players: {num_players}"),
        }
    }
}
