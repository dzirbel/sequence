use crate::LOG_LEVEL;

pub enum Log {
    None, // log nothing from each game
    Results, // log the result of each game
    Turn, // log the card played in each turn
    Board, // log the board after each turn
}

impl Log {
    pub fn log(&self, message: &str) {
        self.if_logged(|| println!("{}", message));
    }

    pub fn if_logged<F>(&self, block: F) where F: FnOnce() {
        if LOG_LEVEL.ord() >= self.ord() {
            block();
        }
    }

    fn ord(&self) -> i8 {
        match self {
            Log::None => 0,
            Log::Results => 1,
            Log::Turn => 2,
            Log::Board => 3,
        }
    }
}
