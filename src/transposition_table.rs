use crate::board::Move;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TTEntry {
    hash: u64,
    value: i32,
    depth: u8,
    flag: TTFlag,
    best_move: Option<Move>,
}

const TT_SIZE: usize = 1 << 20;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
    Empty,
}

pub struct TranspositionTable {
    table: Box<[TTEntry]>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: vec![
                TTEntry {
                    hash: 0,
                    value: 0,
                    depth: 0,
                    flag: TTFlag::Empty,
                    best_move: None,
                };
                TT_SIZE
            ]
            .into_boxed_slice(),
        }
    }
}
