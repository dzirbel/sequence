use crate::core::board::{Board, SEQUENCE_LENGTH};
use crate::core::square::Square;
use crate::core::team::Team;

pub const DIRECTIONS: [(i8, i8); 4] = [
    (0i8, 1i8),
    (1i8, 1i8),
    (1i8, 0i8),
    (1i8, -1i8),
];

pub fn runs_for_team(board: &Board, origin: &Square, team: &Team) -> [Vec<Square>; 4] {
    find_runs(origin, SEQUENCE_LENGTH, |square| {
        board.chip_at(&square).map_or(true, |chip| { chip != *team })
    })
}

pub fn open_runs_for_team(board: &Board, origin: &Square, team: &Team) -> [Vec<Square>; 4] {
    find_runs(origin, SEQUENCE_LENGTH, |square| {
        board.chip_at(&square).map_or(false, |chip| { chip != *team })
    })
}

pub fn find_runs<F>(
    origin: &Square,
    max_distance: u8,
    stop: F,
) -> [Vec<Square>; 4] where F: Fn(Square) -> bool {
    DIRECTIONS.map(|(row_delta, col_delta)| {
        let mut run = vec![];

        // go forward along this direction
        for i in 1..=max_distance {
            let square = origin.plus((i as i8) * row_delta, (i as i8) * col_delta);
            if !square.is_valid() || stop(square) {
                break;
            } else {
                run.push(square);
            }
        }

        // go backward along this direction
        for i in 1..=max_distance {
            let square = origin.plus(-(i as i8) * row_delta, -(i as i8) * col_delta);
            if !square.is_valid() || stop(square) {
                break;
            } else {
                run.push(square);
            }
        }

        run
    })
}

pub fn traverse<F>(
    origin: &Square,
    max_distance: u8,
    on_square: &mut F,
) where F: FnMut(Square) -> bool {
    let directions = [
        (0i8, 1i8),
        (1i8, 1i8),
        (1i8, 0i8),
        (1i8, -1i8),
    ];

    for (row_delta, col_delta) in directions {
        traverse_in_direction(origin, row_delta, col_delta, max_distance, on_square);
        traverse_in_direction(origin, -row_delta, -col_delta, max_distance, on_square);
    }
}

pub fn traverse_in_direction<F>(
    origin: &Square,
    row_delta: i8,
    col_delta: i8,
    max_distance: u8,
    on_square: &mut F,
) where F: FnMut(Square) -> bool {
    for i in 1u8..=max_distance {
        let square = origin.plus((i as i8) * row_delta, (i as i8) * col_delta);
        if !square.is_valid() || !on_square(square) {
            break
        }
    }
}
