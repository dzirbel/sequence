use std::collections::HashMap;

use crate::core::game::Game;
use crate::core::player::Player;
use crate::log::Log;
use crate::players::random_player::RandomPlayer;

pub mod core;
mod log;
mod players;
pub mod util;

const N: i32 = 5;
pub const LOG_LEVEL: Log = Log::None;

fn main() {
    let mut winners = HashMap::new();

    for i in 0..N {
        let players: Vec<Box<dyn Player>> = vec!(
            Box::new(RandomPlayer {}),
            Box::new(RandomPlayer {}),
        );

        let result = Game::new(players, 2).run();
        winners.entry(result.winner).and_modify(|count| *count += 1).or_insert(1);

        Log::Results.log(
            &format!("Finished game {}/{} : {} won in {} turns",
                     i + 1, N, result.winner, result.turns)
        );
    }

    Log::Results.log("");
    println!("Done!");
    for (team, win_count) in winners {
        println!("{} : {}", team, win_count);
    }
}
