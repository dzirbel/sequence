use crate::game::player::Player;
use crate::players::random_player::RandomPlayer;

mod game;
mod players;

fn main() {
    for i in 0..10000 {
        let players: Vec<Box<dyn Player>> = vec!(
            Box::new(RandomPlayer {}),
            Box::new(RandomPlayer {}),
        );

        let mut game = game::game::Game::new(players, 2);

        loop {
            if let Some(_) = game.run_turn(false) {
                break
            }
        }

        println!("Finished game {}", i + 1);
    }
}
