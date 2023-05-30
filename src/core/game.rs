use crate::core::board::Board;
use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::player::Player;
use crate::core::square::Square;
use crate::core::team::Team;
use crate::log::LogLevel;
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

pub struct GameResult {
    pub winner: Team,
    pub turns: usize,
}

impl Game {
    // creates a new game
    // TODO pass through same RNG source to deck
    pub fn new(players: Vec<Box<dyn Player>>, num_teams: usize) -> Game {
        debug_assert!(
            num_teams % players.len() == 0,
            "invalid number of teams: {} for {} players", num_teams, players.len(),
        );

        let mut deck = Deck::new();

        let hand_size = Game::hand_size(players.len());
        let player_hands = generate_vector(players.len(), |_| {
            generate_vector(hand_size, |_| deck.draw())
        });

        Game {
            players,
            num_teams,
            up_index: 0, // use given player order
            player_hands,
            board: Board::standard_board(),
            deck,
            turn_count: 0,
        }
    }

    pub fn run(&mut self) -> GameResult {
        loop {
            if let Some(winner) = self.run_turn() {
                return winner;
            }
        }
    }

    pub fn run_turn(&mut self) -> Option<GameResult> {
        self.turn_count += 1;

        // optionally replace a dead card
        let has_playable_card = self.replace_dead_card();
        if !has_playable_card {
            LogLevel::Turn.log(&format!("Turn {}: player {} has no playable cards; skipping turn",
                                        self.turn_count, self.up_index));

            self.up_index = (self.up_index + 1) % self.players.len();

            return None;
        }

        // choose a card, validate it, remove it from the player's hand, and discard it
        let (choice_card, choice_square) = self.play_card();

        // place the chip on the board and check victory conditions
        let result = self.place_chip(choice_card, choice_square);

        LogLevel::Board.if_logged(|| self.board.print());

        // likely moot, but don't finish the turn count when the game is over
        if let Some(winner) = result {
            return Some(GameResult { winner, turns: self.turn_count });
        }

        // draw a new card
        self.player_hands[self.up_index].push(self.deck.draw());

        // advance turn counters
        self.up_index = (self.up_index + 1) % self.players.len();

        None
    }

    // returns true if the player has any playable cards
    fn replace_dead_card(&mut self) -> bool {
        let hand = &self.player_hands[self.up_index];
        if hand.iter().any(|card| self.board.is_dead(card)) {
            let replaced_card_choice = self.up_player().replace_dead_card(&self.board, hand);
            if let Some(replaced_card_index) = replaced_card_choice {
                debug_assert!(replaced_card_index <= hand.len(), "dead card choice out of bounds");

                let replaced_card = &self.player_hands[self.up_index].remove(replaced_card_index);
                debug_assert!(self.board.is_dead(replaced_card), "replaced card was not dead");

                self.deck.discard(*replaced_card);

                LogLevel::Turn.log(&format!("Turn {}: player {} replaced dead card {}",
                                            self.turn_count, self.up_index, replaced_card));

                let new_card = self.deck.draw();
                self.player_hands[self.up_index].push(new_card);
            }
        }

        let team = Game::player_team(self.num_teams, self.up_index);
        self.player_hands[self.up_index].iter()
            .any(|card| self.board.can_be_played(card, &team))
    }

    fn play_card(&mut self) -> (Card, Square) {
        let player_team = Game::player_team(self.num_teams, self.up_index);
        let (choice_index, choice_square) = self.up_player().play(
            &player_team,
            &self.player_hands[self.up_index],
            &self.board,
            &self.deck,
        );

        debug_assert!(
            choice_index as usize <= self.player_hands[self.up_index].len(),
            "card choice out of bounds",
        );
        debug_assert!(
            choice_square.is_playable(),
            "invalid square choice: {choice_square}",
        );

        let choice_card = self.player_hands[self.up_index].remove(choice_index as usize);
        self.deck.discard(choice_card);

        (choice_card, choice_square)
    }

    fn place_chip(&mut self, card: Card, square: Square) -> Option<Team> {
        let player_team = Game::player_team(self.num_teams, self.up_index);
        if card.is_one_eyed_jack() {
            debug_assert!(
                self.board.chip_at(&square).unwrap() != player_team,
                "attempted to remove chip on own team's square {}", square,
            );

            self.board.remove_chip(&square);
            LogLevel::Turn.log(
                &format!(
                    "Turn {}: player {} played {}, a one-eyed Jack, and removed the chip on {}",
                    self.turn_count,
                    self.up_index,
                    card,
                    square,
                ),
            );
        } else {
            debug_assert!(
                card.is_two_eyed_jack() || self.board.card_at(&square).unwrap() == card,
                "attempted to put chip on square not matching the chosen card {}: {}", card, square,
            );

            LogLevel::Turn.log(
                &format!(
                    "Turn {}: player {} played {} on {}",
                    self.turn_count,
                    self.up_index,
                    card,
                    square,
                ),
            );

            if let Some(sequences) = self.board.add_chip(&square, player_team) {
                if sequences >= Game::winning_sequences(self.num_teams) {
                    return Some(player_team);
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

#[cfg(test)]
mod tests {
    use crate::players::deterministic_player::DeterministicPlayer;
    use super::*;

    // TODO avoid use of RNG in the game deck
    #[test]
    fn deterministic_game_runs_without_panics() {
        for _ in 0..100 {
            let players: Vec<Box<dyn Player>> = vec![
                Box::new(DeterministicPlayer {}),
                Box::new(DeterministicPlayer {}),
            ];

            let mut game = Game::new(players, 2);
            game.run();
        }
    }
}
