use std::fmt::{Display, Formatter, Result};

use strum_macros::EnumIter;

#[derive(Clone, Copy, EnumIter, Eq, Hash, PartialEq)]
pub enum Team {
    ONE,
    TWO,
    THREE,
}

impl Team {
    pub fn with_team_color(&self, string: &str, background: bool) -> String {
        let color = match self {
            Team::ONE => if background { "44" } else { "34" }, // blue
            Team::TWO => if background { "42" } else { "32" }, // green
            Team::THREE => if background { "41" } else { "31" }, // red
        };

        format!("\x1b[{color}m{string}\x1b[39;49m")
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let team_name = match self {
            Team::ONE => "BLUE",
            Team::TWO => "GREEN",
            Team::THREE => "RED",
        };

        write!(f, "{}", team_name)
    }
}
