use macroquad::{miniquad::EventHandler, prelude::*};
mod display_board;
mod board_move;
mod board;

async fn event_handler(board: &mut board::Board) {
	while !is_key_down(KeyCode::Escape) {
		if is_mouse_button_released(MouseButton::Left) {
			let (x, y) = mouse_position();
			if board.set_and_check_move(x, y) {
				place_stone(x, y, player_color(board.player_to_move)).await;
				board.player_to_move = board.player_to_move.get_opposite_non_empty();
			}
			
		}
		next_frame().await;
	}
}