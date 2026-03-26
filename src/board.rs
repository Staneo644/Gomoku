use std::fmt;
const BOARD_SIZE: usize = 19;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
}

#[derive(Debug)]
pub enum BoardError {
    InvalidMove,
    OccupiedCell,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::White => write!(f, "0"),
            Cell::Black => write!(f, "X"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum NonEmptyCell {
    Black,
    White,
}

impl NonEmptyCell {
    pub fn get(&self) -> Cell {
        match self {
            NonEmptyCell::Black => Cell::Black,
            NonEmptyCell::White => Cell::White,
        }
    }
}

pub struct Board {
    pub grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

impl Board {
    pub fn set(&mut self, x: usize, y: usize, cell: NonEmptyCell) -> Result<(), BoardError> {
        let cell = cell.get();
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return Err(BoardError::InvalidMove);
        }

        if self.grid[x][y] == Cell::Empty {
            self.grid[x][y] = cell;
            return Ok(());
        }
        return Err(BoardError::OccupiedCell);
    }
}

impl Board {
    pub fn check(&self, x: usize, y: usize, cell: NonEmptyCell) -> bool {
        fn count_direction(
            grid: &[[Cell; BOARD_SIZE]; BOARD_SIZE],
            x: i32,
            y: i32,
            dx: i32,
            dy: i32,
            cell: Cell,
        ) -> i32 {
            let mut count = 0;
            let mut nx = x + dx;
            let mut ny = y + dy;

            while nx >= 0
                && nx < BOARD_SIZE as i32
                && ny >= 0
                && ny < BOARD_SIZE as i32
                && grid[nx as usize][ny as usize] == cell
            {
                count += 1;
                nx += dx;
                ny += dy;
            }

            count
        }

        let x = x as i32;
        let y = y as i32;
        let cell: Cell = cell.get();

        let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];

        for (dx, dy) in directions {
            let count = 1
                + count_direction(&self.grid, x, y, dx, dy, cell)
                + count_direction(&self.grid, x, y, -dx, -dy, cell);

            if count >= 5 {
                return true;
            }
        }

        false
    }
}

impl Board {
    pub fn set_and_check(&mut self, x: usize, y: usize, cell: NonEmptyCell) -> bool {
        match self.set(x, y, cell) {
            Ok(()) => self.check(x, y, cell),
            Err(e) => {
                println!("Error: {:?}", e);
                false
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "┌───────────────────┐")?;
        for line in self.grid {
            write!(f, "│")?;
            for cell in line {
                write!(f, "{}", cell)?;
            }

            writeln!(f, "│")?;
        }
        writeln!(f, "└───────────────────┘")?;
        Ok(())
    }
}
