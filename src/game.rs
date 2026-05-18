use macroquad::prelude::*;
use crate::board::{Board, NonEmptyCell};
use crate::display::display_message::*;
use crate::utils::scale_to_resolution;
use crate::eventHandler::eventHandler::*;

#[derive(PartialEq)]
pub enum GameMode {
	None,
	HumanVsHuman,
	HumanVsAI,
	// insert more modes here
}

#[derive(PartialEq)]
pub enum GameVariant {
	None,
	Standard,
	// insert more variants here
}

#[derive(PartialEq)]
pub enum GameState {
	Playing,
	Paused,
	GameOver,
}

pub struct Game {
	pub board: Board,
	pub current_player: NonEmptyCell,
	game_mode: GameMode,
	game_variant: GameVariant,
	pub game_state: GameState,
	pub message: Option<Message>,
	//menu: Menu,
}

impl Game {
	pub fn new() -> Self {
		Game {
			board: Board::new(),
			current_player: NonEmptyCell::Black,
			game_mode: GameMode::None,
			game_variant: GameVariant::None,
			game_state: GameState::Playing,
			message: None,
			//menu: Menu::new(),
		}
		// display window to pick game mode and variant
	}
	
	pub fn reset(&mut self) {
		self.board = Board::new();
		self.current_player = NonEmptyCell::Black;
		self.game_mode = GameMode::None;
		self.game_variant = GameVariant::None;
		self.game_state = GameState::Paused;
		// display window to pick game mode and variant

	}

	pub fn draw_mouse_hover(&self) {
		if self.game_state != GameState::Playing {
			return;
		}
		let (x, y) = mouse_position();
		if x < screen_width() * 0.1 || x > screen_width() * 0.9 || y < screen_height() * 0.1 || y > screen_height() * 0.9 {
			return;
		}
		let line_x = screen_width() * 0.1;
		let line_y = screen_height() * 0.1;
		let cell_size_x = screen_width() * 0.8 / 18.;
		let cell_size_y = screen_height() * 0.8 / 18.;
		let cell_size = if cell_size_x < cell_size_y { cell_size_x } else { cell_size_y };
		let board_x = ((x - line_x) / cell_size_x - 0.5).floor() + 0.5;
		let board_y = ((y - line_y) / cell_size_y - 0.5).floor() + 0.5;
		let color = if self.current_player == NonEmptyCell::Black { Color { r: (0.), g: (0.), b: (0.), a: (0.5) } }
			else { Color { r: (1.), g: (1.), b: (1.), a: (0.5) } };
		draw_circle(line_x + (board_x as f32) * cell_size_x + cell_size_x / 2., line_y + (board_y as f32) * cell_size_y + cell_size_y / 2., cell_size / 2. - 2., color);
	}

	pub async fn launch(&mut self) {
		request_new_screen_size(1000., 1000.);
		while self.game_state != GameState::GameOver {
			self.board.draw_board();
			self.board.place_all_stones();
			self.draw_mouse_hover();
			event_handler(self).await;
			if let Some(message) = &mut self.message {
				message.display_message();
				message.timer -= get_frame_time();
				if message.timer <= 0. {
					self.message = None;
				}
			}

			next_frame().await;
		}
	}
}