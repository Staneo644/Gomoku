use smallvec::SmallVec;
use std::fmt;
pub const BOARD_SIZE: usize = 19;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
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

impl NonEmptyCell {
    pub fn get_opposite(&self) -> Cell {
        match self {
            NonEmptyCell::Black => Cell::White,
            NonEmptyCell::White => Cell::Black,
        }
    }
}

impl fmt::Display for NonEmptyCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NonEmptyCell::Black => write!(f, "X"),
            NonEmptyCell::White => write!(f, "0"),
        }
    }
}

pub struct Move {
    pub x: usize,
    pub y: usize,
    pub cell: NonEmptyCell,

    pub captured: SmallVec<[(usize, usize); 4]>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{} - {} - Captured: {:?}",
            self.x, self.y, self.cell, self.captured
        )
    }
}

pub struct Board {
    pub(crate) grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    pub moves: Vec<Move>,
    hash: u64,
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
            moves: vec![],
            hash: 0,
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
