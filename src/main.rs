mod board;

use board::{Board, NonEmptyCell};

pub fn main() {
    let mut test = Board::new();
    let _ = test.set(3, 3, NonEmptyCell::Black);
    println!("{}", test.set_and_check(3, 2, NonEmptyCell::White));
    println!("{}", test.set_and_check(4, 2, NonEmptyCell::White));
    println!("{}", test.set_and_check(5, 2, NonEmptyCell::White));
    println!("{}", test.set_and_check(6, 2, NonEmptyCell::White));
    println!("{}", test.set_and_check(7, 2, NonEmptyCell::White));

    println!("{}", test);
}
