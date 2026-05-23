use crate::{game::GameState, utils::scale_to_resolution};
use macroquad::prelude::*;
pub const MENU_WIDTH: f32 = 300.;
pub const MENU_HEIGHT: f32 = 400.;
pub const MENU_START_POINT: Vec2 = Vec2::new(350., 300.);
pub const MENU_OPTIONS: [&str; 3] = ["NEW GAME", "RESUME GAME", "EXIT"];
pub const OPTION_HEIGHT: f32 = 50.;
pub const OPTION_WIDTH: f32 = 250.;
use crate::game::{GameMode, GameVariant};
use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum MenuAction {
    ChangeState(GameState),
    SetGameMode(GameMode),
    SetGameVariant(GameVariant),
}

impl fmt::Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuAction::ChangeState(_) => write!(f, "ChangeState"),
            MenuAction::SetGameMode(_) => write!(f, "SetGameMode"),
            MenuAction::SetGameVariant(_) => write!(f, "SetGameVariant"),
        }
    }
}

pub struct Menu {
    options: Vec<MenuOption>,
}

impl Menu {
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

pub struct MenuOption {
    text: String,
    action: MenuAction,
    location: (Vec2, Vec2),
}

impl MenuOption {
    pub fn new(text: String, action: MenuAction, location: (Vec2, Vec2)) -> Self {
        Self {
            text,
            action: action,
            location,
        }
    }
    pub fn is_hovered(&self, mouse_pos: Vec2) -> bool {
        let (start, end) = self.location;
        mouse_pos.x >= start.x
            && mouse_pos.x <= end.x
            && mouse_pos.y >= start.y
            && mouse_pos.y <= end.y
    }

    pub fn draw(&self) {
        let color = if self.is_hovered(mouse_position().into()) {
            LIGHTGRAY
        } else {
            GRAY
        };
        draw_rectangle(
            self.location.0.x,
            self.location.0.y,
            self.location.1.x - self.location.0.x,
            self.location.1.y - self.location.0.y,
            color,
        );
        let text_size = 20.;
        let text_dimensions = measure_text(&self.text, None, text_size as u16, 1.);
        let text_x = self.location.0.x
            + (self.location.1.x - self.location.0.x - text_dimensions.width) / 2.;
        let text_y = self.location.0.y
            + (self.location.1.y - self.location.0.y + text_dimensions.height) / 2.;
        draw_text(&self.text, text_x, text_y, text_size, BLACK);
    }

    pub fn click(&self) -> Option<MenuAction> {
        if self.is_hovered(mouse_position().into()) {
            Some(self.action)
        } else {
            None
        }
    }
}

/* Main Menu :
- New Game :
    - Player vs Player
    - Player vs AI
    &&
    - game starts variants
- Resume Game
- Settings (? usefull ?)
- Exit
*/
