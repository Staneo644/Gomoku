use super::scoring::{self, CAPTURE_THREAT, SCORING_TABLE};
use crate::{
    board::{BOARD_SIZE, Board, Cell, NonEmptyCell},
    board_move::valid_move,
    directions::PRIMARY_DIRECTIONS,
};

fn calculate_score(
    cell_start: Cell,
    cell_end: Cell,
    opposite_cell: Cell,
    count: usize,
) -> scoring::ScoringState {
    let scoring_state: scoring::ScoringState;

    match cell_start {
        Cell::Empty => {
            if cell_end == Cell::Empty {
                scoring_state = scoring::ScoringState::Open;
            } else {
                // if cell_end == opposite_cell && count == 2 {
                // return -CAPTURE_THREAT;
                // }
                scoring_state = scoring::ScoringState::HalfOpen;
            }
        }
        other => {
            if cell_end == Cell::Empty {
                // if other == opposite_cell && count == 2 {
                // return -CAPTURE_THREAT;
                // }
                scoring_state = scoring::ScoringState::HalfOpen;
            } else {
                scoring_state = scoring::ScoringState::Closed;
            }
        }
    }
    scoring_state
}

pub fn count_direction_move_ordering(
    grid: &[[Cell; BOARD_SIZE]; BOARD_SIZE],
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
) -> i32 {
    let mut cell_start_positive: Cell = Cell::Empty;
    let mut count_positive = 0;
    let mut cell_end_positive: Cell = Cell::Empty;
    let scoring_state: scoring::ScoringState;

    let mut x1 = x as i32 + dx;
    let mut y1 = y as i32 + dy;

    if valid_move(x1, y1) && grid[x1 as usize][y1 as usize] != Cell::Empty {
        cell_start_positive = grid[x1 as usize][y1 as usize];
        x1 += dx;
        y1 += dy;
        count_positive += 1;

        while valid_move(x1, y1) && grid[x1 as usize][y1 as usize] == cell_start_positive {
            count_positive += 1;
            x1 += dx;
            y1 += dy;
        }

        if valid_move(x1, y1) {
            cell_end_positive = grid[x1 as usize][y1 as usize];
        }
    }

    let mut x1 = x as i32 - dx;
    let mut y1 = y as i32 - dy;
    let mut cell_start_negative: Cell = Cell::Empty;
    let mut cell_end_negative: Cell = Cell::Empty;
    let mut count_negative = 0;

    if valid_move(x1, y1) && grid[x1 as usize][y1 as usize] != Cell::Empty {
        cell_start_negative = grid[x1 as usize][y1 as usize];
        x1 -= dx;
        y1 -= dy;
        count_negative += 1;

        while valid_move(x1, y1) && grid[x1 as usize][y1 as usize] == cell_start_negative {
            count_negative += 1;
            x1 -= dx;
            y1 -= dy;
        }

        if valid_move(x1, y1) {
            cell_end_negative = grid[x1 as usize][y1 as usize];
        }
    }

    let count = count_negative + count_positive;
    let scoring_state = calculate_score(
        cell_start_positive,
        cell_end_positive,
        cell_start_positive.get_opposite(),
        count_positive,
    );
    //  calculate_score(
    //     cell_start_negative,
    //     cell_end_negative,
    //     cell_start_negative.get_opposite(),
    //     count_negative,
    // );
    if count >= 5 {
        return super::scoring::FIVE;
    }
    if count >= 5 {
        return scoring::FIVE;
    }
    SCORING_TABLE[count as usize][scoring_state as usize]
    // count as i32
}

impl Board {
    pub fn move_ordering(&self, cell: NonEmptyCell) -> Vec<(usize, usize, i32)> {
        let mut moves = Vec::new();
        let mut count;
        for coo in self.neighboring_empty_cells.keys() {
            count = 0;
            for (dx, dy) in PRIMARY_DIRECTIONS {
                count += count_direction_move_ordering(
                    &self.grid, coo.0, coo.1, dx,
                    dy,
                    // cell.get(),
                    // cell.get_opposite(),
                );
            }
            moves.push((coo.0, coo.1, count));
        }

        moves.sort_by(|a, b| b.2.cmp(&a.2));
        moves
    }
}
