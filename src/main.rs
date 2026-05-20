mod ai;
mod board;
mod board_move;
mod directions;
mod game;
mod display;
mod eventHandler;
mod utils;
mod menu;
mod player;
// mod menu;

#[macroquad::main("Gomoku")]
async fn main() {
	let mut game = game::Game::new();
	game.launch().await;
}
