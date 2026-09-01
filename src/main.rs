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
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_resizable: true,
        fullscreen: true,
        window_title: String::from("Gomoku"),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = game::Game::new();
    game.launch().await;
}
