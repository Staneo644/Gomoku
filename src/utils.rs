use macroquad::prelude::*;

pub fn scale_to_resolution(value: f32, x_axis: bool) -> f32 {
    let base_resolution = 1000.;
    let current_resolution = if x_axis {
		screen_width()
	} else {
		screen_height()
	};
    value * (current_resolution / base_resolution)
}
