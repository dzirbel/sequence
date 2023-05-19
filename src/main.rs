use crate::game::player::Player;
use crate::players::random_player::RandomPlayer;

mod game;
mod lib;
mod players;

const N: i32 = 100_000;
const PRINT_TURNS: bool = false;

fn main() {
    for i in 0..N {
        let players: Vec<Box<dyn Player>> = vec!(
            Box::new(RandomPlayer {}),
            Box::new(RandomPlayer {}),
        );

        let mut game = game::game::Game::new(players, 2);

        while game.run_turn(PRINT_TURNS).is_none() {}

        println!("Finished game {}/{}", i + 1, N);
    }
}
