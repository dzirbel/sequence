use std::collections::HashMap;

use crate::core::game::Game;
use crate::core::player::Player;
use crate::log::Log;
use crate::players::random_player::RandomPlayer;
use crate::players::square_evaluation_player::SquareEvaluationPlayer;

pub mod core;
mod log;
mod players;
pub mod util;

const N: i32 = 100;
pub const LOG_LEVEL: Log = Log::Results;

fn main() {
    let mut winners = HashMap::new();

    for i in 0..N {
        let players: Vec<Box<dyn Player>> = vec!(
            Box::new(RandomPlayer {}),
            Box::new(SquareEvaluationPlayer { two_eyed_jack_cutoff: 50 }),
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
