use crate::{
    board::{BOARD_SIZE, Board, Cell, NonEmptyCell},
    directions::PRIMARY_DIRECTIONS,
    scoring::{self, CAPTURE_THREAT, SCORING_TABLE},
};

impl Board {
    pub fn evaluate(&self, cell: NonEmptyCell) -> i32 {
        fn count_direction_evaluate(
            grid: &[[Cell; BOARD_SIZE]; BOARD_SIZE],
            mut x: usize,
            mut y: usize,
            dx: i32,
            dy: i32,
            cell: Cell,
            opposite_cell: Cell,
        ) -> i32 {
            if grid[x][y] != cell {
                return 0;
            }

            let x_1 = x - dx as usize;
            let y_1 = y - dy as usize;
            let mut cell_start: Cell = cell;

            if x_1 < BOARD_SIZE && y_1 < BOARD_SIZE && x_1 >= 0 && y_1 >= 0 {
                if grid[x_1][y_1] == cell {
                    return 0;
                }
                cell_start = grid[x_1][y_1];
            }

            let mut count = 0;
            let mut cell_end: Cell = cell;

            while x < BOARD_SIZE && y < BOARD_SIZE && x >= 0 && y >= 0 {
                if grid[x][y] == cell {
                    count += 1;
                } else {
                    cell_end = grid[x][y];
                    break;
                }
                x = (x as i32 + dx) as usize;
                y = (y as i32 + dy) as usize;
            }

            let scoring_state;
            match cell_start {
                Cell::Empty => {
                    if cell_end == Cell::Empty {
                        scoring_state = scoring::SCORING_STATE::Open;
                    } else {
                        if cell_end == opposite_cell && count == 2 {
                            return -CAPTURE_THREAT;
                        }
                        scoring_state = scoring::SCORING_STATE::HalfOpen;
                    }
                }
                other => {
                    if cell_end == Cell::Empty {
                        if other == opposite_cell && count == 2 {
                            return -CAPTURE_THREAT;
                        }
                        scoring_state = scoring::SCORING_STATE::HalfOpen;
                    } else {
                        scoring_state = scoring::SCORING_STATE::Closed;
                    }
                }
            }
            println!("Count: {}, Scoring State: {:?}", count, scoring_state);
            SCORING_TABLE[count - 1 as usize][scoring_state as usize]
        }
        let opposite_cell = cell.get_opposite();

        let cell: Cell = cell.get();
        let mut count = 0;

        for one_move in self.moves.iter() {
            for (dx, dy) in PRIMARY_DIRECTIONS {
                count += count_direction_evaluate(
                    &self.grid,
                    one_move.x,
                    one_move.y,
                    dx,
                    dy,
                    cell,
                    opposite_cell,
                );
            }
        }
        count
    }
}
