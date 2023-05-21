use std::fmt::{Display, Formatter, Result};

use strum_macros::EnumIter;

#[derive(Clone, Copy, EnumIter, Eq, Hash, PartialEq)]
pub enum Team {
    One,
    Two,
    Three,
}

impl Team {
    pub fn with_team_color(&self, string: &str, background: bool) -> String {
        let color = match self {
            Team::One => if background { "44" } else { "34" }, // blue
            Team::Two => if background { "42" } else { "32" }, // green
            Team::Three => if background { "41" } else { "31" }, // red
        };

        format!("\x1b[{color}m{string}\x1b[39;49m")
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let team_name = match self {
            Team::One => "BLUE",
            Team::Two => "GREEN",
            Team::Three => "RED",
        };

        write!(f, "{}", team_name)
    }
}
