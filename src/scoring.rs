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

pub const CAPTURE_PAIR: i32 = 20000;
pub const CAPTURE_THREAT: i32 = 10000;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SCORING_STATE {
    Open,
    HalfOpen,
    Closed,
}

use std::fmt;

impl fmt::Display for SCORING_STATE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SCORING_STATE::Open => write!(f, "Open"),
            SCORING_STATE::HalfOpen => write!(f, "HalfOpen"),
            SCORING_STATE::Closed => write!(f, "Closed"),
        }
    }
}

pub const SCORING_TABLE: [[i32; 3]; 5] = [
    [0, 0, 0],
    [OPEN_TWO, HALF_OPEN_TWO, CLOSED_TWO],
    [OPEN_THREE, HALF_OPEN_THREE, CLOSED_THREE],
    [OPEN_FOUR, HALF_OPEN_FOUR, CLOSED_FOUR],
    [FIVE, FIVE, FIVE],
];
