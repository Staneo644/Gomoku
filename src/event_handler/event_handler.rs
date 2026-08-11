use crate::board;
use crate::display::display_message::*;
use crate::game::{Game, GameMode, GameState, GameVariant};
use crate::menu::menu::MenuAction;
use macroquad::prelude::*;

async fn get_board_coordinates(x: &mut f32, y: &mut f32) -> (usize, usize) {
	let board_size = screen_width().min(screen_height()) * 0.8;
    let line_x = screen_width() / 2. - board_size / 2. * 0.9;
    let line_y = screen_height() / 2. - board_size / 2. * 0.9;
    let cell_size = board_size * 0.9 / 18.;
    let board_x = ((*x - line_x) / cell_size + 0.5).floor() as usize;
    let board_y = ((*y - line_y) / cell_size + 0.5).floor() as usize;
    *x = line_x + (board_x as f32) * cell_size + cell_size;
    *y = line_y + (board_y as f32) * cell_size + cell_size;
    (board_x, board_y)
}

pub fn menu_event_handler(game: &mut Game) {
    if is_mouse_button_released(MouseButton::Left) {
        let click_event = if GameState::MainMenu == game.game_state {
            game.menu.click()
        } else if GameState::NewGameMenu == game.game_state {
            game.new_game_menu.click()
        } else if GameState::SettingsMenu == game.game_state {
			game.settings_menu.click()
		} else if GameState::PickColor == game.game_state && !game.is_current_player_ai() {
			game.pick_color_menu.click(game.game_variant == GameVariant::Swap2)
		} else {
            return;
        };
        match click_event {
            Some(menu_action) => match menu_action {
                MenuAction::ChangeState(game_state) => game.set_game_state(game_state),
                MenuAction::SetGameMode(game_mode) => game.game_mode = game_mode,
                MenuAction::SetGameVariant(game_variant) => game.game_variant = game_variant,
				MenuAction::PickColor(player_color) => game.change_players_colors(player_color),
				MenuAction::ResizeWindow(resolution) => game.resize_window(resolution),
            },
            None => {}
        }
        if game.game_state == GameState::NewGameMenu
            && game.game_mode != GameMode::None
            && game.game_variant != GameVariant::None
        {
            game.adapt_to_game_mode_and_variant();
            game.game_state = GameState::Playing;
        }
        // selct menu item
    }
}

pub async fn place_stone_handler(game: &mut Game, board_x: usize, board_y: usize) {
    match game.board.set_and_check(
        board_x,
        board_y,
        game.players.as_ref().unwrap()[game.current_player].get_color(),
    ) {
        Ok(true) => {
            game.game_state = GameState::Finished;
        }
        Ok(false) => {
            game.change_player();
        }
        Err(e) => {
            if !game.is_current_player_ai() {
                game.message = Some(Message::new(e.to_string(), MessageType::Error));
            }
        }
    }
}

pub async fn mouse_play_event_handler(game: &mut Game) {
    if game.is_current_player_ai() {
        return;
    }
    if is_mouse_button_released(MouseButton::Left) {
        let (mut x, mut y) = mouse_position();
		let board_size = screen_width().min(screen_height()) * 0.8;
        if x < screen_width() / 2. - board_size / 2. * 0.95
			|| x > screen_width() / 2. + board_size / 2. * 0.95
			|| y < screen_height() / 2. - board_size / 2. * 0.95
			|| y > screen_height() / 2. + board_size / 2. * 0.95
        {
            game.message = Some(Message::new(
                "Click inside the board".to_string(),
                MessageType::Error,
            ));
            return;
        }
        let (board_x, board_y) = get_board_coordinates(&mut x, &mut y).await;
        place_stone_handler(game, board_x, board_y).await;
        // put_stone_on_board(board_x, board_y, 1);
    }
}

pub async fn event_handler(game: &mut Game) {
    if is_key_pressed(KeyCode::Escape) {
        if game.game_state == GameState::Playing {
            game.game_state = GameState::MainMenu;
        } else if game.game_state == GameState::MainMenu {
            if game.game_mode != GameMode::None && game.game_variant != GameVariant::None {
                game.game_state = GameState::Playing;
            } else {
                game.message = Some(Message::new(
                    "Please select a game mode and variant".to_string(),
                    MessageType::Error,
                ));
            }
        } else if game.game_state == GameState::NewGameMenu {
            game.game_state = GameState::MainMenu;
        }
		else if game.game_state == GameState::PickColor {
			game.message = Some(Message::new(
				"Please select a color".to_string(),
				MessageType::Error,
			));
			return;
		} else {
            game.game_state = GameState::MainMenu;
        }
    }
    if game.game_state == GameState::Playing {
        mouse_play_event_handler(game).await;
        if is_key_pressed(KeyCode::R) {
            game.reset();
        }
    }
    menu_event_handler(game);
}
