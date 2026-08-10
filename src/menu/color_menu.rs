use macroquad::{math::Vec2, window::screen_width};
use macroquad::prelude::*;
use crate::menu::menu::Button;
use crate::{board::NonEmptyCell, game::GameState, menu::menu::{MenuAction, MenuOption}, utils::scale_to_resolution};

const COLOR_OPTIONS: [&str; 3] = ["BLACK", "PLAY 2", "WHITE"];
const COLOR_OPTION_SIZE: Vec2 = Vec2::new(75., 75.);

// let rect_pos = vec2(scale_to_resolution(screen_width() * 0.45), scale_to_resolution(screen_height() * 0.4625));


pub struct ColorMenu {
	start_point: Vec2,
	color_options: Vec<MenuOption>,
}

impl ColorMenu {
	pub fn new() -> Self {
		let mut menu = Self {
			color_options: Vec::new(),
			start_point: Vec2::new(
				500. - ((COLOR_OPTION_SIZE.x + 10.) * 1.5),
				500. - (COLOR_OPTION_SIZE.y + 10.) / 2.
			)
		};
		let mut optionIter = 0.;
		for option in COLOR_OPTIONS {
			let start_location = Vec2::new(
				menu.start_point.x + (COLOR_OPTION_SIZE.x + 10.) * optionIter,
				menu.start_point.y
			);
			let end_location = Vec2::new(
				start_location.x + COLOR_OPTION_SIZE.x,
				start_location.y + COLOR_OPTION_SIZE.y
			);
			menu.color_options.push(MenuOption::new(
				option.to_string(), 
				match option {
					"BLACK" => MenuAction::PickColor(NonEmptyCell::Black),
					"WHITE" => MenuAction::PickColor(NonEmptyCell::White),
					_ => MenuAction::ChangeState(GameState::Swap2),
				},
				(start_location, end_location),
			));
			optionIter += 1.;
		}

		menu
	}

	pub fn draw(&self, swap2: bool) {
		let menu_to_scale = Vec2::new(
            scale_to_resolution(COLOR_OPTION_SIZE.x * self.color_options.len() as f32 + (10. * self.color_options.len() as f32 + 10.), true),
            scale_to_resolution(COLOR_OPTION_SIZE.y + 20., false),
        );
        let menu_start = Vec2::new(
            scale_to_resolution(self.start_point.x - 10., true),
            scale_to_resolution(self.start_point.y - 10., false),
        );
        let text_dimensions = measure_text("PICK COLOR", None, scale_to_resolution(40., false) as u16, 0.8);

		println!("window size: {},{}\n menu_size: {}\nmenu_start_scale: {}\nmenu_start: {}",
				screen_width(), screen_height(),
				menu_to_scale,
				menu_start,
				self.start_point
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
            "PICK COLOR",
            screen_width() * 0.5 - text_dimensions.width / 2.,
            screen_height() * 0.35,
            scale_to_resolution(40., false),
            WHITE,
        );
		let mut optionIter = 0;
        for option in &self.color_options {
			optionIter += 1;
			if !swap2 && optionIter == 2 {
				continue;
			}
			else {
				option.draw();
			}
        }
	}
}