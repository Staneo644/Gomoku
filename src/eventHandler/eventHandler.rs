use macroquad::{miniquad::EventHandler, prelude::*};
use crate::game::*;
use crate::display::display_message::*;
use crate::utils::scale_to_resolution;

async fn get_board_coordinates(x: &mut f32, y: &mut f32) -> (usize, usize) {
	let line_x = screen_width() * 0.1;
	let line_y = screen_height() * 0.1;
	let cell_size_x = screen_width() * 0.8 / 18.;
	let cell_size_y = screen_height() * 0.8 / 18.;
	let board_x = ((*x - line_x) / cell_size_x + 0.5).floor() as usize;
	let board_y = ((*y - line_y) / cell_size_y + 0.5).floor() as usize;
	*x = line_x + (board_x as f32) * cell_size_x + cell_size_x;
	*y = line_y + (board_y as f32) * cell_size_y + cell_size_y;
	(board_x, board_y)
}

pub async fn event_handler(game: &mut Game) {
		if is_key_pressed(KeyCode::Escape)
		{
			if game.game_state == GameState::Playing {
				game.game_state = GameState::Menu;
			}
			else if game.game_state == GameState::Menu {
				game.game_state = GameState::Playing;
			}
		}
		if is_mouse_button_released(MouseButton::Left) && game.game_state == GameState::Playing {
			let (mut x, mut y) = mouse_position();
			if x < screen_width() * 0.075 || x > screen_width() * 0.925 || y < screen_height() * 0.075 || y > screen_height() * 0.925 {
				game.message = Some(Message::new("Click inside the board".to_string(), MessageType::Error));
				return;
			}
			let (board_x, board_y) = get_board_coordinates(&mut x, &mut y).await;
			match game.board.set_and_check(board_x, board_y, game.current_player) {
				Ok(true) => {
					game.current_player = game.current_player.get_opposite_non_empty();
					game.game_state = GameState::Finished;
				},
				Ok(false) => {
					game.current_player = game.current_player.get_opposite_non_empty();
				},
				Err(e) => {
					game.message = Some(Message::new(e.to_string(), MessageType::Error));
				}
			}
			// put_stone_on_board(board_x, board_y, 1);
		}
		if is_mouse_button_released(MouseButton::Left) && game.game_state == GameState::Menu {
			let (mut x, mut y) = mouse_position();
			println!("Mouse released at: ({}, {})", x, y);
			// selct menu item
		}
		if game.game_state == GameState::Playing {
			if is_key_pressed(KeyCode::R){
				game.reset();
			}
		}
		// next_frame().await;
	// }
}