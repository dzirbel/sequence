use crate::core::board::Board;
use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::player::Player;
use crate::core::square::Square;
use crate::core::team::Team;
use crate::util::generate_vector;

pub struct Game {
    players: Vec<Box<dyn Player>>,
    // specified since 6/12 player games could have 2 or 3 teams
    num_teams: usize,
    turn_count: usize,
    up_index: usize,
    player_hands: Vec<Vec<Card>>,
    board: Board,
    deck: Deck,
}

impl Game {
    // creates a new game
    // TODO pass through same RNG source to deck
    pub fn new(players: Vec<Box<dyn Player>>, num_teams: usize) -> Game {
        if num_teams % players.len() != 0 {
            panic!("invalid number of teams: {} for {} players", num_teams, players.len())
        }

        let hand_size = Game::hand_size(players.len());

        let mut deck = Deck::new();

        let player_hands = generate_vector(players.len(), |_| {
            generate_vector(hand_size, |_| deck.draw())
        });

        Game {
            players,
            num_teams,
            up_index: 0, // use given player order
            player_hands,
            board: Board::new(),
            deck,
            turn_count: 0,
        }
    }

    pub fn run(&mut self, print_turns: bool) -> Team {
        loop {
            if let Some(winner) = self.run_turn(print_turns) {
                return winner;
            }
        }
    }

    pub fn run_turn(&mut self, print_turn: bool) -> Option<Team> {
        // optionally replace a dead card
        let has_playable_card = self.replace_dead_card(print_turn);
        if !has_playable_card {
            if print_turn {
                println!("Turn {}: player {} has no playable cards; skipping turn",
                         self.turn_count, self.up_index);
            }

            self.up_index = (self.up_index + 1) % self.players.len();
            self.turn_count += 1;

            return None;
        }

        // choose a card, validate it, remove it from the player's hand, and discard it
        let (choice_card, choice_square) = self.play_card();

        // place the chip on the board and check victory conditions
        let result = self.place_chip(choice_card, choice_square, print_turn);

        if print_turn {
            self.board.print();
        }

        if result.is_some() {
            return result;
        }

        // draw a new card
        self.player_hands[self.up_index].push(self.deck.draw());

        // advance turn counters
        self.up_index = (self.up_index + 1) % self.players.len();
        self.turn_count += 1;

        None
    }

    // returns true if the player has any playable cards
    fn replace_dead_card(&mut self, print: bool) -> bool {
        let hand = &self.player_hands[self.up_index];
        if hand.iter().any(|card| self.board.is_dead(card)) {
            let replaced_card_choice = self.up_player().replace_dead_card(&self.board, hand);
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

                if print {
                    println!("Turn {}: player {} replaced dead card {}",
                             self.turn_count, self.up_index, replaced_card);
                }

                let new_card = self.deck.draw();
                self.player_hands[self.up_index].push(new_card);
            }
        }

        self.player_hands[self.up_index].iter()
            .any(|card| self.board.can_be_played(card))
    }

    fn play_card(&mut self) -> (Card, Square) {
        let player_team = Game::player_team(self.num_teams, self.up_index);
        let (choice_index, choice_square) = self.up_player().play(
            &player_team,
            &self.player_hands[self.up_index],
            &self.board,
            &self.deck,
        );

        let hand_size = self.player_hands[self.up_index].len();
        if choice_index as usize > hand_size {
            panic!("card choice out of bounds: {choice_index} for hand size {}", hand_size)
        }

        if !choice_square.is_playable() {
            panic!("invalid square choice: {choice_square}")
        }

        let choice_card = self.player_hands[self.up_index].remove(choice_index as usize);
        self.deck.discard(choice_card);

        (choice_card, choice_square)
    }

    fn place_chip(&mut self, card: Card, square: Square, print: bool) -> Option<Team> {
        let player_team = Game::player_team(self.num_teams, self.up_index);
        let current_claim = self.board.chip_at(&square);
        if card.is_one_eyed_jack() {
            match current_claim {
                None => panic!("attempted to remove chip on unclaimed square {square}"),
                Some(team) => {
                    if team == player_team {
                        panic!("attempted to remove chip on own team's square {square}")
                    }
                }
            }

            self.board.remove_chip(&square);
            if print {
                println!(
                    "Turn {}: player {} played {}, a one-eyed Jack, and removed the chip on {}",
                    self.turn_count,
                    self.up_index,
                    card,
                    square,
                );
            }
        } else {
            match current_claim {
                Some(_) => panic!("attempted to put chip on claimed square {square}"),
                None => {
                    if !card.is_two_eyed_jack() && Some(card) != self.board.card_at(&square) {
                        panic!(
                            "attempted to put chip on square not matching the \
                            chosen card {card}: {square}"
                        )
                    }
                }
            }

            if print {
                println!(
                    "Turn {}: player {} played {} on {}",
                    self.turn_count,
                    self.up_index,
                    card,
                    square,
                );
            }

            let new_sequence = self.board.add_chip(&square, player_team);
            if let Some(sequences) = new_sequence {
                if sequences >= Game::winning_sequences(self.num_teams) {
                    return Some(player_team)
                }
            }
        }

        None
    }

    fn up_player(&self) -> &dyn Player {
        self.players[self.up_index].as_ref()
    }

    // gets the team that the player at the given index belongs to
    pub fn player_team(num_teams: usize, player_index: usize) -> Team {
        let team_index = player_index % num_teams;
        match team_index {
            0 => Team::One,
            1 => Team::Two,
            2 => Team::Three,
            _ => panic!("invalid team index (too many teams?)")
        }
    }

    // gets the number of sequences required to win, for the given number of teams
    pub fn winning_sequences(num_teams: usize) -> usize {
        match num_teams {
            2 => 2,
            3 => 1,
            _ => panic!("unsupported number of teams: {num_teams}")
        }
    }

    // gets the size of each player's hand for the given number of players in the game
    pub fn hand_size(num_players: usize) -> usize {
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
