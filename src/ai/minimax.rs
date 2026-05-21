use crate::board::{Board, NonEmptyCell};

const CELLS_TO_CHECK: usize = 3;
const OPPOSITE_CELL_TO_CHECK: usize = 3;
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

    let mut best_eval = MoveWithScore {
        position: (0, 0),
        score: if is_maximizing { i32::MIN } else { i32::MAX },
    };
    let best_move = board.move_ordering(cell);
    // let best_opposite_move = board.move_ordering(cell.get_opposite_non_empty());
    let mut i = 0;
    if depth == 10 {
        println!("Best move: {:?}", best_move);
        // println!("Best opposite move: {:?}", best_opposite_move);
    }
    for (x, y, _) in best_move {
        if i >= CELLS_TO_CHECK {
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
                if (is_maximizing && current_move.score > best_eval.score)
                    || (!is_maximizing && current_move.score < best_eval.score)
                {
                    best_eval = MoveWithScore {
                        position: (x, y),
                        score: current_move.score,
                    };
                }
                i += 1;
                let _ = board.unset();
            }
        }
    }
    best_eval
}

pub fn ia_move(board: &mut Board, cell: NonEmptyCell) -> (usize, usize) {
    let Result = minimax(board, 10, true, cell).position;
    println!("Best move: {:?}", Result);
    Result
}
