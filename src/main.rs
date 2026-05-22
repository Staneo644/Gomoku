mod ai;
mod board;
mod board_move;
mod directions;
// use ai::minimax::ia_move;
use board::{Board, NonEmptyCell};
use macroquad::prelude::*;
mod display;
mod eventHandler;
mod game;
mod game;
mod game;
mod menu;
mod player;
mod utils;
// mod menu;

#[macroquad::main("Gomoku")]
async fn main() {
    let mut game = game::Game::new();
    game.launch().await;
}
