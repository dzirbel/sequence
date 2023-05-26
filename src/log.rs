use std::time;
use crate::LOG_OPTIONS;

pub struct LogOptions {
    pub level: LogLevel,

    // when logging results, restrict logging when either of the two is provided (preferring every_n
    // if both are):

    // // log results after every n games, n > 0
    pub every_n: Option<usize>,
    // log results after each percent of the total games is complete, 0 <= percent <= 100
    pub every_percent: Option<f32>,
}

pub enum LogLevel {
    #[allow(dead_code)]
    None,

    // log nothing from each game
    Results,
    // log the result of each game
    Turn,
    // log the card played in each turn
    Board, // log the board after each turn
}

impl LogLevel {
    pub fn log(&self, message: &str) {
        self.if_logged(|| println!("{}", message));
    }

    pub fn if_logged<F>(&self, block: F) where F: FnOnce() {
        if LOG_OPTIONS.level.ord() >= self.ord() {
            block();
        }
    }

    pub fn on_result(game: usize, total: usize, start: &time::Instant) {
        LogLevel::Results.if_logged(|| {
            // TODO log result every time if the log level is strictly greater than only logging
            //  results
            let log_result = LOG_OPTIONS.every_n
                .map(|n| n as f32)
                .or(LOG_OPTIONS.every_percent.map(|p| { (p / 100.0) * total as f32 }))
                .map(|n| {
                    // TODO doesn't work quite correctly for non-integer n
                    game % (n as usize) == 0
                })
                .unwrap_or(true);

            if log_result {
                let percent = (game as f32 / total as f32) * 100.0;
                // TODO improve formatting of the duration
                // TODO print expected remaining time
                println!("Finished game {}/{} ({:.2}%) in {:?}",
                         game, total, percent, start.elapsed());
            }
        });
    }

    fn ord(&self) -> i8 {
        match self {
            LogLevel::None => 0,
            LogLevel::Results => 1,
            LogLevel::Turn => 2,
            LogLevel::Board => 3,
        }
    }
}
