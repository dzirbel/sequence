use std::fmt::{Display, Formatter};
use itertools::iproduct;
use crate::core::board::BOARD_SIZE;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Square {
    pub row: u8,
    pub col: u8,
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let col_char = (b'a' + self.col) as char;
        write!(f, "{}{}", col_char, self.row)
    }
}

impl Square {
    pub fn playable_squares() -> impl Iterator<Item=Square> {
        iproduct!(0..BOARD_SIZE, 0..BOARD_SIZE)
            .filter_map(|(row, col)| {
                let square = Square { row, col };
                if square.is_corner() { None } else { Some(square) }
            })
    }

    pub fn is_valid(&self) -> bool {
        self.row < BOARD_SIZE && self.col < BOARD_SIZE
    }

    pub fn is_playable(&self) -> bool {
        self.is_valid() && !self.is_corner()
    }

    pub fn is_corner(&self) -> bool {
        (self.row == 0 || self.row == BOARD_SIZE - 1) &&
            (self.col == 0 || self.col == BOARD_SIZE - 1)
    }

    pub fn plus(&self, row_delta: i8, col_delta: i8) -> Square {
        Square {
            row: (self.row as i8 + row_delta) as u8,
            col: (self.col as i8 + col_delta) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::square::Square;

    impl Square {
        pub fn from_notation(notation: &str) -> Square {
            if notation.len() != 2 { panic!("invalid notation string: {notation}") }

            let mut chars = notation.chars();
            let col_char = chars.next().unwrap();
            let row_char = chars.next().unwrap();
            Square {
                col: (col_char as usize).saturating_sub('a' as usize) as u8,
                row: (row_char as usize).saturating_sub('0' as usize) as u8,
            }
        }
    }

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
