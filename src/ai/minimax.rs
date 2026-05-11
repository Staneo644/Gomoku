use crate::board::{Board, NonEmptyCell};

struct MoveWithScore {
    position: (usize, usize),
    score: i32,
}

fn final_eval(board: &Board, cell: NonEmptyCell) -> i32 {
    board.evaluate(cell) - board.evaluate(cell.get_opposite_non_empty())
}

fn minimax(
    board: &mut Board,
    depth: i32,
    is_maximizing: bool,
    cell: NonEmptyCell,
) -> MoveWithScore {
    if depth == 0 {
        return MoveWithScore {
            position: (0, 0),
            score: final_eval(board, cell),
        };
    }

    if is_maximizing {
        let mut max_eval = MoveWithScore {
            position: (0, 0),
            score: i32::MIN,
        };
        let best_move = board.move_ordering(cell);
        let mut i = 0;
        for (x, y, _) in best_move {
            if i >= 3 {
                break;
            }
            match board.set_and_check(x, y, cell) {
                Err(_) => continue,
                Ok(true) => {
                    let _ = board.unset();
                    return MoveWithScore {
                        position: (x, y),
                        score: super::scoring::FIVE,
                    };
                }
                Ok(false) => {
                    let current_move = minimax(board, depth - 1, !is_maximizing, cell);
                    if current_move.score > max_eval.score {
                        max_eval = MoveWithScore {
                            position: (x, y),
                            score: current_move.score,
                        };
                    }
                    i += 1;
                    let _ = board.unset();
                }
            }
        }
        max_eval
    } else {
        let mut min_eval = MoveWithScore {
            position: (0, 0),
            score: i32::MAX,
        };
        let best_move = board.move_ordering(cell);
        let mut i = 0;
        for (x, y, _) in best_move {
            if i >= 3 {
                break;
            }
            match board.set_and_check(x, y, cell) {
                Err(_) => continue,
                Ok(true) => {
                    let _ = board.unset();
                    return MoveWithScore {
                        position: (x, y),
                        score: -super::scoring::FIVE,
                    };
                }
                Ok(false) => {
                    let current_move = minimax(board, depth - 1, !is_maximizing, cell);
                    if current_move.score < min_eval.score {
                        min_eval = MoveWithScore {
                            position: (x, y),
                            score: current_move.score,
                        };
                    }
                    i += 1;
                    let _ = board.unset();
                }
            }
        }
        min_eval
    }
}

pub fn ia_move(board: &mut Board, cell: NonEmptyCell) -> (usize, usize) {
    minimax(board, 10, true, cell).position
}
