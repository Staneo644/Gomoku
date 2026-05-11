mod ai;
mod board;
mod board_move;
mod directions;
// use ai::minimax::ia_move;
use board::{Board, NonEmptyCell};

pub fn main() {
    let mut test = Board::new();
    test.set_and_check(3, 5, NonEmptyCell::White);
    test.set_and_check(3, 4, NonEmptyCell::White);
    test.set_and_check(4, 2, NonEmptyCell::White);
    test.set_and_check(5, 2, NonEmptyCell::White);
    test.set_and_check(3, 2, NonEmptyCell::White);
    println!("{}", test.evaluate(NonEmptyCell::White));
    println!("{}", test.evaluate(NonEmptyCell::Black));
    // let ret = ai::minimax::ia_move(&mut test, NonEmptyCell::Black);
    // println!("({}, {})", ret.0, ret.1);
    let moves = test.move_ordering(NonEmptyCell::White);
    for (x, y, score) in moves {
        println!("({}, {}): {}", x, y, score);
    }
    println!("{}", test);
}
