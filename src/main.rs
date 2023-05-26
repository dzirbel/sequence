use std::collections::HashMap;
use std::time::Instant;

use crate::core::game::Game;
use crate::core::player::Player;
use crate::log::{LogLevel, LogOptions};
use crate::players::random_player::RandomPlayer;
use crate::players::square_evaluation_player::SquareEvaluationPlayer;

pub mod core;
mod log;
mod players;
pub mod util;

const N: usize = 100;
const LOG_OPTIONS: LogOptions = LogOptions {
    level: LogLevel::Results,
    every_n: None,
    every_percent: Some(5.0),
};

fn main() {
    let start = Instant::now();
    let mut winners = HashMap::new();

    for i in 0..N {
        let players: Vec<Box<dyn Player>> = vec!(
            Box::new(RandomPlayer {}),
            Box::new(SquareEvaluationPlayer { ..Default::default() }),
        );

        let result = Game::new(players, 2).run();
        winners.entry(result.winner).and_modify(|count| *count += 1).or_insert(1);

        LogLevel::on_result(i + 1, N, &start);
    }

    LogLevel::Results.log("");
    println!("Done in {:?}!", start.elapsed());
    for (team, win_count) in winners {
        println!("{} : {}", team, win_count);
    }
}
