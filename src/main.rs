use std::collections::HashMap;
use crate::core::game::Game;
use crate::core::player::Player;
use crate::players::random_player::RandomPlayer;

pub mod core;
mod players;
pub mod util;

const N: i32 = 1_000;
const PRINT_TURNS: bool = false;

fn main() {
    let mut winners = HashMap::new();

    for i in 0..N {
        let players: Vec<Box<dyn Player>> = vec!(
            Box::new(RandomPlayer {}),
            Box::new(RandomPlayer {}),
        );

        let winner = Game::new(players, 2).run(PRINT_TURNS);
        winners.entry(winner).and_modify(|count| *count += 1).or_insert(1);

        println!("Finished game {}/{} : {} won", i + 1, N, winner);
    }

    println!();
    println!("Done!");
    for (team, win_count) in winners {
        println!("{} : {}", team, win_count);
    }
}
