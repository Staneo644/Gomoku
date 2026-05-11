pub const FIVE: i32 = 1000000;

pub const OPEN_FOUR: i32 = 100000;
pub const HALF_OPEN_FOUR: i32 = 50000;
pub const CLOSED_FOUR: i32 = 10000;

pub const OPEN_THREE: i32 = 1000;
pub const HALF_OPEN_THREE: i32 = 500;
pub const CLOSED_THREE: i32 = 100;

pub const OPEN_TWO: i32 = 10;
pub const HALF_OPEN_TWO: i32 = 5;
pub const CLOSED_TWO: i32 = 1;

pub const CAPTURE_THREAT: i32 = 10000;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ScoringState {
    Open,
    HalfOpen,
    Closed,
}

use std::fmt;

impl fmt::Display for ScoringState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScoringState::Open => write!(f, "Open"),
            ScoringState::HalfOpen => write!(f, "HalfOpen"),
            ScoringState::Closed => write!(f, "Closed"),
        }
    }
}

pub fn capture_score(pairs: usize) -> i32 {
    match pairs {
        0 => 0,
        1 => 10000,
        2 => 20000,
        3 => 25000,
        4 => 50000,
        5 => 100000,
        6 => 100000,
        7 => 200000,
        8 => 300000,
        _ => FIVE,
    }
}

pub const SCORING_TABLE: [[i32; 3]; 5] = [
    [0, 0, 0],
    [OPEN_TWO, HALF_OPEN_TWO, CLOSED_TWO],
    [OPEN_THREE, HALF_OPEN_THREE, CLOSED_THREE],
    [OPEN_FOUR, HALF_OPEN_FOUR, CLOSED_FOUR],
    [FIVE, FIVE, FIVE],
];
