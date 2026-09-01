use crate::game::{GameMode, GameVariant};
use crate::{
    game::GameState,
    menu::menu::{
        Button, MENU_HEIGHT, MENU_START_POINT, MENU_WIDTH, MenuAction, MenuOption, OPTION_HEIGHT,
        OPTION_WIDTH,
    },
    utils::scale_to_resolution,
};
use macroquad::prelude::*;

pub struct ButtonsHolder {
    buttons: Vec<Box<dyn Button>>,
}

pub struct SettingsMenu {
    input: String,
    buttons: ButtonsHolder,
    text_box_focused: bool,
    error: Option<String>,
}

impl SettingsMenu {
    pub fn new() -> Self {
        let mut menu = Self {
            input: String::new(),
            buttons: ButtonsHolder {
                buttons: Vec::new(),
            },
            text_box_focused: false,
            error: None,
        };

        // let start_location
        menu.buttons.buttons.push(Box::new(MenuOption::new(
            "RES 800x800".to_string(),
            MenuAction::ResizeWindow(800),
            (
                Vec2::new(
                    500. - OPTION_WIDTH / 2. as f32,
                    MENU_START_POINT.y + 50. + (OPTION_HEIGHT + 10.) * 1.,
                ),
                Vec2::new(
                    500. + OPTION_WIDTH / 2. as f32,
                    MENU_START_POINT.y + 50. + (OPTION_HEIGHT + 10.) * 1. + OPTION_HEIGHT,
                ),
            ),
        )));
        menu.buttons.buttons.push(Box::new(MenuOption::new(
            "RES 1000x1000".to_string(),
            MenuAction::ResizeWindow(1000),
            (
                Vec2::new(
                    500. - OPTION_WIDTH / 2. as f32,
                    MENU_START_POINT.y + 50. + (OPTION_HEIGHT + 10.) * 2.,
                ),
                Vec2::new(
                    500. + OPTION_WIDTH / 2. as f32,
                    MENU_START_POINT.y + 50. + (OPTION_HEIGHT + 10.) * 2. + OPTION_HEIGHT,
                ),
            ),
        )));
        menu.buttons.buttons.push(Box::new(MenuOption::new(
            "RES 1200x1200".to_string(),
            MenuAction::ResizeWindow(1200),
            (
                Vec2::new(
                    500. - OPTION_WIDTH / 2. as f32,
                    MENU_START_POINT.y + 50. + (OPTION_HEIGHT + 10.) * 3.,
                ),
                Vec2::new(
                    500. + OPTION_WIDTH / 2. as f32,
                    MENU_START_POINT.y + 50. + (OPTION_HEIGHT + 10.) * 3. + OPTION_HEIGHT,
                ),
            ),
        )));
        menu.buttons.buttons.push(Box::new(MenuOption::new(
            "BACK".to_string(),
            MenuAction::ChangeState(GameState::MainMenu),
            (
                Vec2::new(
                    500. - OPTION_WIDTH / 2. as f32,
                    MENU_START_POINT.y + 50. + (OPTION_HEIGHT + 10.) * 4.,
                ),
                Vec2::new(
                    500. + OPTION_WIDTH / 2. as f32,
                    MENU_START_POINT.y + 50. + (OPTION_HEIGHT + 10.) * 4. + OPTION_HEIGHT,
                ),
            ),
        )));

        menu
    }

    pub fn draw(&self) {
        let menu_to_scale = Vec2::new(
            scale_to_resolution(MENU_WIDTH, true),
            scale_to_resolution(MENU_HEIGHT, false),
        );
        let menu_start = Vec2::new(
            scale_to_resolution(MENU_START_POINT.x, true),
            scale_to_resolution(MENU_START_POINT.y, false),
        );
        let text_dimensions = measure_text(
            "RESOLUTION",
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
            "RESOLUTION",
            scale_to_resolution(500., true) - text_dimensions.width / 2.,
            scale_to_resolution(350., false),
            scale_to_resolution(40., false),
            WHITE,
        );
        for option in &self.buttons.buttons {
            option.draw();
        }
    }
    pub fn click(&mut self) -> Option<MenuAction> {
        for option in &mut self.buttons.buttons {
            if let Some(menu_action) = option.click() {
                return Some(menu_action);
            }
        }
        None
    }
}
