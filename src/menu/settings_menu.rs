use crate::{board::NonEmptyCell, game::GameState, utils::scale_to_resolution};
use macroquad::prelude::*;
use crate::game::{GameMode, GameVariant};
use std::fmt;

pub struct SettingsMenu {
    options: Vec<MenuOption>,
}

impl SettingsMenu {
    pub fn new() -> Self {
        let mut menu = Self {
            options: Vec::new(),
        };

        let mut index = 0;
        for name in MENU_OPTIONS {
            let y_position = MENU_START_POINT.y + 100. + index as f32 * (OPTION_HEIGHT + 10.);
            let x_position = MENU_START_POINT.x + (MENU_WIDTH - OPTION_WIDTH) / 2.;
            menu.add_option(
                name.to_string(),
                (
                    Vec2::new(x_position, y_position),
                    Vec2::new(x_position + OPTION_WIDTH, y_position + OPTION_HEIGHT),
                ),
            );
            index += 1;
        }

        menu
    }

    fn add_option(&mut self, text: String, location: (Vec2, Vec2)) {
        let action = match text.as_str() {
            "NEW GAME" => MenuAction::ChangeState(GameState::NewGameMenu),
            "RESUME GAME" => MenuAction::ChangeState(GameState::ResumeGame),
			"SETTINGS" => MenuAction::ChangeState(GameState::SettingsMenu),
            "EXIT" => MenuAction::ChangeState(GameState::Exiting),
            _ => MenuAction::ChangeState(GameState::MainMenu), // default case, should not happen
        };
        self.options.push(MenuOption::new(text, action, location));
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
        for option in &self.options {
            if let Some(menu_action) = option.click() {
                return Some(menu_action);
            }
        }
        None
    }
}