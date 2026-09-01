use crate::game::{GameMode, GameVariant};
use crate::menu::menu::{
    Button, MENU_HEIGHT, MENU_START_POINT, MENU_WIDTH, MenuAction, MenuOption, OPTION_HEIGHT,
    OPTION_WIDTH,
};
pub const MODE_OPTIONS: [&str; 2] = ["HUMAN VS HUMAN", "HUMAN VS AI"];
pub const VARIANT_OPTIONS: [&str; 4] = ["STANDARD", "SWAP2", "SINGLE SWAP", "PRO"];
use crate::utils::scale_to_resolution;
use macroquad::prelude::*;

pub struct NewGameMenu {
    mode_options: Vec<MenuOption>,
    variant_options: Vec<MenuOption>,
    mode_selected: bool,
    variant_selected: bool,
}

impl NewGameMenu {
    pub fn new() -> Self {
        let mut menu = Self {
            mode_options: Vec::new(),
            variant_options: Vec::new(),
            mode_selected: false,
            variant_selected: false,
        };

        let mut index = 0;
        for mode in MODE_OPTIONS {
            let y_position =
                MENU_START_POINT.y * 0.66 + 100. + index as f32 * (OPTION_HEIGHT + 10.);
            let x_position = MENU_START_POINT.x + (MENU_WIDTH - OPTION_WIDTH) / 2.;
            let action = match mode {
                "HUMAN VS HUMAN" => MenuAction::SetGameMode(GameMode::HumanVsHuman),
                "HUMAN VS AI" => MenuAction::SetGameMode(GameMode::HumanVsAI),
                _ => MenuAction::SetGameMode(GameMode::None), // default case, should not happen
            };
            menu.mode_options.push(MenuOption::new(
                mode.to_string(),
                action,
                (
                    Vec2::new(x_position, y_position),
                    Vec2::new(x_position + OPTION_WIDTH, y_position + OPTION_HEIGHT),
                ),
            ));
            index += 1;
        }
        index += 1;
        for variant in VARIANT_OPTIONS {
            let y_position =
                (MENU_START_POINT.y * 0.66) + 100. + index as f32 * (OPTION_HEIGHT + 10.);
            let x_position = MENU_START_POINT.x + (MENU_WIDTH - OPTION_WIDTH) / 2.;
            let action = match variant {
                "STANDARD" => MenuAction::SetGameVariant(GameVariant::Standard),
                "SWAP2" => MenuAction::SetGameVariant(GameVariant::Swap2),
                "SINGLE SWAP" => MenuAction::SetGameVariant(GameVariant::SingleSwap),
                "PRO" => MenuAction::SetGameVariant(GameVariant::Pro),
                _ => MenuAction::SetGameVariant(GameVariant::None), // default case, should not happen
            };
            menu.variant_options.push(MenuOption::new(
                variant.to_string(),
                action,
                (
                    Vec2::new(x_position, y_position),
                    Vec2::new(x_position + OPTION_WIDTH, y_position + OPTION_HEIGHT),
                ),
            ));
            index += 1;
        }
        menu
    }

    pub fn draw(&self) {
        let menu_to_scale = Vec2::new(
            scale_to_resolution(MENU_WIDTH, true),
            scale_to_resolution(MENU_HEIGHT * 1.5, false),
        );
        let menu_start = Vec2::new(
            scale_to_resolution(MENU_START_POINT.x, true),
            scale_to_resolution(MENU_START_POINT.y * 0.66, false),
        );
        let text_dimensions = measure_text(
            "NEW GAME",
            None,
            scale_to_resolution(40., false) as u16,
            0.8,
        );

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
            "NEW GAME",
            screen_width() * 0.5 - text_dimensions.width / 2.,
            screen_height() * 0.25,
            scale_to_resolution(40., false),
            WHITE,
        );
        if self.mode_selected == false {
            for option in &self.mode_options {
                option.draw();
            }
        }
        if self.variant_selected == false {
            for option in &self.variant_options {
                option.draw();
            }
        }
    }
    pub fn click(&mut self) -> Option<MenuAction> {
        for option in &mut self.mode_options {
            if let Some(menu_action) = option.click() {
                self.mode_selected = true;
                return Some(menu_action);
            }
        }
        for option in &mut self.variant_options {
            if let Some(menu_action) = option.click() {
                self.variant_selected = true;
                return Some(menu_action);
            }
        }
        None
    }
}
