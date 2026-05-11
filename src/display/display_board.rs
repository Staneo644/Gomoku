use macroquad::{miniquad::EventHandler, prelude::*};

async fn player_color(player: NonEmptyCell) -> Color {
	match player {
		NonEmptyCell::Black => BLACK,
		NonEmptyCell::White => WHITE,
	}
}

async fn draw_board() {
	clear_background(WHITE);

	draw_rectangle(screen_width() * 0.075,screen_height() * 0.075, screen_width() * 0.85, screen_width() * 0.85, BEIGE);
	let line = screen_width() * 0.1;
	for i in 0..19 {
		draw_line(screen_width() * 0.1,
		 line + ((screen_width() * 0.8) / 18.) * (i as f32), 
		 screen_width() * 0.9, 
		 line + ((screen_width() * 0.8) / 18.) * (i as f32), 
		 1.0, DARKGRAY);}
	for i in 0..19 {
		draw_line(line + ((screen_width() * 0.8) / 18.) * (i as f32), 
		 screen_height() * 0.1, 
		 line + ((screen_width() * 0.8) / 18.) * (i as f32), 
		 screen_height() * 0.9, 
		 1.0, DARKGRAY);}
	for i in 0..19 {
		for j in 0..19 {
			let ray;
			if (i + 3) % 6 == 0 && (j + 3) % 6 == 0 {
				ray = 3.5;
			} else {
				ray = 2.0;
			}
			draw_circle(line + ((screen_width() * 0.8) / 18.) * (i as f32), 
			 line + ((screen_width() * 0.8) / 18.) * (j as f32), 
			 ray, DARKGRAY);
		}
	}
}

async fn place_stone(x: f32, y: f32, color: Color) {
	let ray = screen_width() * 0.8 / 18. / 2. - 2.;
	draw_circle(x, y, ray, color);
}