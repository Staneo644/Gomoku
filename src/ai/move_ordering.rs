use super::scoring::{self, SCORING_TABLE};
use crate::{
    ai::scoring::CAPTURE_THREAT,
    board::{BOARD_SIZE, Board, Cell, NonEmptyCell},
    board_move::{get_cell_i32, is_used},
    directions::PRIMARY_DIRECTIONS,
};

fn calculate_score(cell_start: Cell, cell_end: Cell) -> scoring::ScoringState {
    let scoring_state: scoring::ScoringState;

    match cell_start {
        Cell::Empty => {
            if cell_end == Cell::Empty {
                scoring_state = scoring::ScoringState::Open;
            } else {
                scoring_state = scoring::ScoringState::HalfOpen;
            }
        }
        _ => {
            if cell_end == Cell::Empty {
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
    cell: NonEmptyCell,
) -> i32 {
    let mut cell_type_start_positive: Cell = Cell::Empty;
    let mut count_positive = 0;
    let mut cell_type_end_positive: Cell = Cell::Invalid;

    let mut x1 = x as i32 + dx;
    let mut y1 = y as i32 + dy;

    if is_used(grid, x1, y1) {
        cell_type_start_positive = grid[x1 as usize][y1 as usize];
        x1 += dx;
        y1 += dy;
        count_positive += 1;

        while get_cell_i32(grid, x1, y1) == cell_type_start_positive {
            count_positive += 1;
            x1 += dx;
            y1 += dy;
        }

        cell_type_end_positive = get_cell_i32(grid, x1, y1);
    }

    let mut x1 = x as i32 - dx;
    let mut y1 = y as i32 - dy;
    let mut cell_type_start_negative: Cell = Cell::Empty;
    let mut cell_type_end_negative: Cell = Cell::Invalid;
    let mut count_negative = 0;

    if is_used(grid, x1, y1) {
        cell_type_start_negative = grid[x1 as usize][y1 as usize];
        x1 -= dx;
        y1 -= dy;
        count_negative += 1;

        while get_cell_i32(grid, x1, y1) == cell_type_start_negative {
            count_negative += 1;
            x1 -= dx;
            y1 -= dy;
        }

        cell_type_end_negative = get_cell_i32(grid, x1, y1);
    }

    let scoring_state_positive = calculate_score(cell_type_start_positive, cell_type_end_positive);

    let can_be_captured_positive = (cell_type_start_positive.get_opposite()
        == cell_type_end_positive)
        && (count_positive == 2);
    if can_be_captured_positive {
        if cell_type_start_positive == cell.get() {
            return -CAPTURE_THREAT;
        }
        return CAPTURE_THREAT;
    }

    let can_be_captured_negative = (cell_type_start_negative.get_opposite()
        == cell_type_end_negative)
        && (count_negative == 2);
    if can_be_captured_negative {
        if cell_type_start_negative == cell.get() {
            return -CAPTURE_THREAT;
        }
        return CAPTURE_THREAT;
    }

    let scoring_state_negative = calculate_score(cell_type_start_negative, cell_type_end_negative);

    let mut score_positive =
        SCORING_TABLE[count_positive as usize][scoring_state_positive as usize];
    if cell_type_start_positive == cell.get() {
        score_positive = -score_positive;
    }
    let mut score_negative =
        SCORING_TABLE[count_negative as usize][scoring_state_negative as usize];
    if cell_type_start_negative == cell.get() {
        score_negative = -score_negative;
    }

    let mut result = score_positive + score_negative;

    if cell_type_start_positive == cell_type_start_negative {
        result *= 2
    }

    result
}

impl Board {
    pub fn move_ordering(&self, cell: NonEmptyCell) -> Vec<(usize, usize, i32)> {
        let mut moves = Vec::new();
        let mut count;
        for coo in self.neighboring_empty_cells.keys() {
            count = 0;
            for (dx, dy) in PRIMARY_DIRECTIONS {
                count += count_direction_move_ordering(&self.grid, coo.0, coo.1, dx, dy, cell);
            }
            moves.push((coo.0, coo.1, count));
        }

        moves.sort_by(|a, b| b.2.cmp(&a.2));
        moves
    }
}
