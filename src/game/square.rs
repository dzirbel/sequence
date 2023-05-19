use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Square {
    pub row: usize,
    pub col: usize,
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let col_char = (('a' as u8) + self.col as u8) as char;
        write!(f, "{}{}", col_char, self.row)
    }
}

impl Square {
    pub fn from_notation(notation: &str) -> Square {
        if notation.len() != 2 { panic!("invalid notation string: {notation}") }

        let mut chars = notation.chars();
        let col_char = chars.next().unwrap();
        let row_char = chars.next().unwrap();
        Square {
            col: (col_char as usize).saturating_sub('a' as usize),
            row: (row_char as usize).saturating_sub('0' as usize),
        }
    }

    pub fn plus(&self, row_delta: i8, col_delta: i8) -> Square {
        Square {
            row: (self.row as i8 + row_delta) as usize,
            col: (self.col as i8 + col_delta) as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::square::Square;

    #[test]
    fn convert_a0_back_and_forth_from_notation() {
        let square = Square::from_notation("a0");
        assert_eq!(Square { row: 0, col: 0 }, square);
        assert_eq!(String::from("a0"), format!("{}", square));
    }

    #[test]
    fn convert_e4_back_and_forth_from_notation() {
        let square = Square::from_notation("e4");
        assert_eq!(Square { row: 4, col: 4 }, square);
        assert_eq!(String::from("e4"), format!("{}", square));
    }
}
