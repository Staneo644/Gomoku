mod ai;
mod board;
mod board_move;
mod directions;
mod display;
mod event_handler;
mod game;
mod menu;
mod player;
mod test;
mod utils;

use macroquad::prelude::*;
use std::env;

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
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--test")) {
        _ = test::launch();
    } else {
        let mut game = game::Game::new();
        game.launch().await;
    }
}
