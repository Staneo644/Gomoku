use macroquad::{miniquad::EventHandler, prelude::*};

pub fn display_error_message(message: String) {
	display_message(message, Some(Color { r: (0.9), g: (0.16), b: (0.22), a: (0.8) }));
}

pub fn display_info_message(message: String) {
	display_message(message, Some(Color { r: (0.16), g: (0.9), b: (0.22), a: (0.8) }));
}

pub fn display_message(message: String, rectangle_color: Option<Color>) {
	let font_size = 30;
	let text_dimensions = measure_text(&message, None, font_size, 0.8);
	let x = (screen_width() * 0.99 - text_dimensions.width);
	let y = screen_height() * 0.05;
	draw_rectangle(x - text_dimensions.width * 0.05, y - text_dimensions.height * 1.5, text_dimensions.width * 1.1, text_dimensions.height * 2., rectangle_color.unwrap_or(Color { r: (0.9), g: (0.16), b: (0.22), a: (0.8) }));
	draw_text(&message, x, y, font_size as f32, BLACK);
}