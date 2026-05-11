use super::scoring::{self, CAPTURE_THREAT, SCORING_TABLE};
use crate::{
    board::{BOARD_SIZE, Board, Cell, NonEmptyCell},
    directions::PRIMARY_DIRECTIONS,
};

pub fn count_direction_move_ordering(
    grid: &[[Cell; BOARD_SIZE]; BOARD_SIZE],
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    cell: Cell,
    opposite_cell: Cell,
) -> i32 {
    let mut cell_start: Cell = cell;
    let mut count = 0;
    let mut cell_end: Cell = cell;
    let scoring_state: scoring::ScoringState;

    let mut x1 = x as i32 + dx;
    let mut y1 = y as i32 + dy;
    while x1 < BOARD_SIZE as i32 && y1 < BOARD_SIZE as i32 && x1 >= 0 && y1 >= 0 {
        if grid[x1 as usize][y1 as usize] == cell {
            count += 1;
        } else {
            cell_end = grid[x as usize][y as usize];
            break;
        }
        x1 = x1 as i32 + dx;
        y1 = y1 as i32 + dy;
    }

    let mut x1 = x as i32 - dx;
    let mut y1 = y as i32 - dy;

    while x1 < BOARD_SIZE as i32 && y1 < BOARD_SIZE as i32 && x1 >= 0 && y1 >= 0 {
        if grid[x1 as usize][y1 as usize] == cell {
            count += 1;
        } else {
            cell_start = grid[x as usize][y as usize];
            break;
        }
        x1 = x1 as i32 - dx;
        y1 = y1 as i32 - dy;
    }

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
    if count >= 5 {
        return scoring::FIVE;
    }
    SCORING_TABLE[count as usize][scoring_state as usize]
}

impl Board {
    pub fn move_ordering(&self, cell: NonEmptyCell) -> Vec<(usize, usize, i32)> {
        let mut moves = Vec::new();
        let mut count;
        for coo in self.neighboring_empty_cells.keys() {
            count = 0;
            for (dx, dy) in PRIMARY_DIRECTIONS {
                count += count_direction_move_ordering(
                    &self.grid,
                    coo.0,
                    coo.1,
                    dx,
                    dy,
                    cell.get(),
                    cell.get_opposite(),
                );
            }
            moves.push((coo.0, coo.1, count));
        }

        moves.sort_by(|a, b| b.2.cmp(&a.2));
        moves
    }
}
