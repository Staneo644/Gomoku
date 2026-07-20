use macroquad::prelude::*;

pub fn scale_to_resolution(value: f32) -> f32 {
    let base_resolution = 1000.;
    let current_resolution = screen_width().max(screen_height());
    value * (current_resolution / base_resolution)
}
