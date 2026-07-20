mod ai;
mod board;
mod board_move;
mod directions;
mod display;
mod event_handler;
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
