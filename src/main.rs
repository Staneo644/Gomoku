mod board;
mod board_move;
mod directions;
mod evaluate;
mod scoring;
use board::{Board, NonEmptyCell};

pub fn main() {
    let mut test = Board::new();
    println!("{}", test.set_and_check(3, 5, NonEmptyCell::White));
    println!("{}", test.set_and_check(3, 4, NonEmptyCell::White));
    println!("{}", test.set_and_check(4, 2, NonEmptyCell::White));
    println!("{}", test.set_and_check(5, 2, NonEmptyCell::White));
    println!("{}", test.set_and_check(3, 2, NonEmptyCell::White));
    println!("{}", test.evaluate(NonEmptyCell::White));
    println!("{}", test.evaluate(NonEmptyCell::Black));

    println!("{}", test);
}
