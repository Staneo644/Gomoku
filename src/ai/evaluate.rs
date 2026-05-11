use super::scoring::{self, CAPTURE_THREAT, SCORING_TABLE, capture_score};
use crate::{
    board::{BOARD_SIZE, Board, Cell, NonEmptyCell},
    directions::PRIMARY_DIRECTIONS,
};

pub fn count_direction_evaluate(
    grid: &[[Cell; BOARD_SIZE]; BOARD_SIZE],
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    cell: Cell,
    opposite_cell: Cell,
) -> i32 {
    if grid[x][y] != cell {
        return 0;
    }

    let x_1 = x as i32 - dx;
    let y_1 = y as i32 - dy;
    let mut cell_start: Cell = cell;

    if x_1 < BOARD_SIZE as i32 && y_1 < BOARD_SIZE as i32 && x_1 >= 0 && y_1 >= 0 {
        if grid[x_1 as usize][y_1 as usize] == cell {
            return 0;
        }
        cell_start = grid[x_1 as usize][y_1 as usize];
    }

    let mut count = 0;
    let mut cell_end: Cell = cell;

    let mut x = x as i32 + dx;
    let mut y = y as i32 + dy;
    while x < BOARD_SIZE as i32 && y < BOARD_SIZE as i32 && x >= 0 && y >= 0 {
        if grid[x as usize][y as usize] == cell {
            count += 1;
        } else {
            cell_end = grid[x as usize][y as usize];
            break;
        }
        x = x as i32 + dx;
        y = y as i32 + dy;
    }

    let scoring_state;
    match cell_start {
        Cell::Empty => {
            if cell_end == Cell::Empty {
                scoring_state = scoring::ScoringState::Open;
            } else {
                if cell_end == opposite_cell && count == 2 {
                    return -CAPTURE_THREAT;
                }
                scoring_state = scoring::ScoringState::HalfOpen;
            }
        }
        other => {
            if cell_end == Cell::Empty {
                if other == opposite_cell && count == 2 {
                    return -CAPTURE_THREAT;
                }
                scoring_state = scoring::ScoringState::HalfOpen;
            } else {
                scoring_state = scoring::ScoringState::Closed;
            }
        }
    }
    if count > 4 {
        return scoring::FIVE;
    }
    SCORING_TABLE[count as usize][scoring_state as usize]
}

impl Board {
    pub fn evaluate(&self, cell: NonEmptyCell) -> i32 {
        let opposite_cell = cell.get_opposite_non_empty();
        let mut count = capture_score(self.captured_by_user[cell as usize])
            - capture_score(self.captured_by_user[opposite_cell as usize]);
        let cell: Cell = cell.get();
        let opposite_cell = opposite_cell.get();

        for coo in self.occupied_positions.keys() {
            for (dx, dy) in PRIMARY_DIRECTIONS {
                count +=
                    count_direction_evaluate(&self.grid, coo.0, coo.1, dx, dy, cell, opposite_cell);
            }
        }
        count
    }
}
