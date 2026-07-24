use crate::{
	board::NonEmptyCell, game::GameState, menu::menu::{
		MENU_HEIGHT, MENU_START_POINT, MENU_WIDTH, OPTION_HEIGHT, OPTION_WIDTH, MenuAction, MenuOption
	}, utils::scale_to_resolution};
use macroquad::prelude::*;
use crate::game::{GameMode, GameVariant};
use std::fmt;

pub struct SettingsMenu {
    input: String,
	inputMenuOption: InputMenuOption,
    text_box_focused: bool,
    apply_button: MenuOption,
	back_button: MenuOption,
    error: Option<String>,
}

impl SettingsMenu {
    pub fn new() -> Self {
       let menu = Self {
			input: String::new(),
			text_box_focused: false,
			error: None,
		}
		
		let start_location
		menu.apply_button: MenuOption::new("APPLY".to_string(), MenuAction::ResizeWindow(1000), (Vec2::ZERO, Vec2::ZERO)),
		menu.back_button: MenuOption::new("BACK".to_string(), MenuAction::ChangeState(GameState::MainMenu), (Vec2::ZERO, Vec2::ZERO)),
	   
    }

    pub fn draw(&self) {
        let menu_to_scale = Vec2::new(
            scale_to_resolution(MENU_WIDTH),
            scale_to_resolution(MENU_HEIGHT),
        );
        let menu_start = Vec2::new(
            scale_to_resolution(MENU_START_POINT.x),
            scale_to_resolution(MENU_START_POINT.y),
        );
        let text_dimensions = measure_text("MENU", None, scale_to_resolution(40.) as u16, 0.8);

        draw_rectangle(
            0.,
            0.,
            screen_width(),
            screen_height(),
            Color {
                r: (0.),
                g: (0.),
                b: (0.),
                a: (0.5),
            },
        );
        draw_rectangle(
            menu_start.x,
            menu_start.y,
            menu_to_scale.x,
            menu_to_scale.y,
            Color {
                r: (0.),
                g: (0.),
                b: (0.),
                a: (0.7),
            },
        );
        draw_text(
            "MENU",
            screen_width() * 0.5 - text_dimensions.width / 2.,
            screen_height() * 0.35,
            scale_to_resolution(40.),
            WHITE,
        );
        for option in &self.options {
            option.draw();
        }
    }
    pub fn click(&self) -> Option<MenuAction> {
        if let Some(menu_action) = self.apply_button {

		}
		for option in &self.options {
            if let Some(menu_action) = option.click() {
                return Some(menu_action);
            }
        }
        None
    }
}

pub struct InputMenuOption {
	inputName: String,
	input: String,
	location: (Vec2, Vec2),
	action: MenuAction,
}

impl InputMenuOption {
	pub fn new() -> Self {}
	pub fn draw(&self) {}
}