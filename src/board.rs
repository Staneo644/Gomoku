use smallvec::SmallVec;
use std::{collections::HashMap, fmt};
pub const BOARD_SIZE: usize = 19;
use macroquad::prelude::*;
use crate::utils::scale_to_resolution;

use crate::game::{Game, GameMode, GameVariant};

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::White => write!(f, "0"),
            Cell::Black => write!(f, "X"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum NonEmptyCell {
    Black,
    White,
}

impl NonEmptyCell {
    pub fn get(&self) -> Cell {
        match self {
            NonEmptyCell::Black => Cell::Black,
            NonEmptyCell::White => Cell::White,
        }
    }
}

impl NonEmptyCell {
    pub fn get_opposite(&self) -> Cell {
        match self {
            NonEmptyCell::Black => Cell::White,
            NonEmptyCell::White => Cell::Black,
        }
    }

    pub fn get_opposite_non_empty(&self) -> NonEmptyCell {
        match self {
            NonEmptyCell::Black => NonEmptyCell::White,
            NonEmptyCell::White => NonEmptyCell::Black,
        }
    }
}

impl fmt::Display for NonEmptyCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NonEmptyCell::Black => write!(f, "X"),
            NonEmptyCell::White => write!(f, "0"),
        }
    }
}

pub struct Move {
    pub x: usize,
    pub y: usize,
    pub cell: NonEmptyCell,

    pub captured: SmallVec<[(usize, usize); 4]>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{} - {} - Captured: {:?}",
            self.x, self.y, self.cell, self.captured
        )
    }
}

pub struct Board {
    pub(crate) grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    pub moves: Vec<Move>,
    hash: u64,
    pub captured_by_user: [usize; 2],
    // key: (x, y), value: number of neighboring pieces
    pub neighboring_empty_cells: HashMap<(usize, usize), usize>,
    // key: (x, y), value: number of neighboring pieces
    pub occupied_positions: HashMap<(usize, usize), usize>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
            moves: vec![],
            hash: 0,
            captured_by_user: [0; 2],
            neighboring_empty_cells: HashMap::new(),
            occupied_positions: HashMap::new(),
        }
    }
}

impl Board {
	pub fn draw_board(&self) {
		clear_background(Color { r: (0.5), g: (0.2), b: (0.2), a: (1.) });

		draw_rectangle(screen_width() * 0.075,screen_height() * 0.075, screen_width() * 0.85, screen_height() * 0.85, BEIGE);
		let line_thickness = 1.;
		let first_height_line = screen_height() * 0.1;
		for i in 0..BOARD_SIZE {
			draw_line(screen_width() * 0.1,
			first_height_line + ((screen_height() * 0.8) / (BOARD_SIZE - 1) as f32) * (i as f32), 
			screen_width() * 0.9, 
			first_height_line + ((screen_height() * 0.8) / (BOARD_SIZE - 1) as f32) * (i as f32), 
			line_thickness, DARKGRAY);}
		let first_width_line = screen_width() * 0.1;
		for i in 0..BOARD_SIZE {
			draw_line(first_width_line + ((screen_width() * 0.8) / (BOARD_SIZE - 1) as f32) * (i as f32), 
				screen_height() * 0.1, 
				first_width_line + ((screen_width() * 0.8) / (BOARD_SIZE - 1) as f32) * (i as f32), 
				screen_height() * 0.9, 
				line_thickness, DARKGRAY);}
		for i in 0..BOARD_SIZE {
			for j in 0..BOARD_SIZE {
				let ray;
				let cell_size = if screen_width() > screen_height() {
					screen_height() * 0.8 / (BOARD_SIZE - 1) as f32
				}
				else {
					screen_width() * 0.8 / (BOARD_SIZE - 1) as f32
				};
				if (i + 3) % 6 == 0 && (j + 3) % 6 == 0 {
					ray = cell_size * 0.1;
				}
				else {
					ray = cell_size * 0.05;
				}
				draw_circle(first_width_line + ((screen_width() * 0.8) / (BOARD_SIZE - 1) as f32) * (i as f32), 
					first_height_line + ((screen_height() * 0.8) / (BOARD_SIZE - 1) as f32) * (j as f32), 
					ray, DARKGRAY);
			}
		}
	}

	fn draw_player_counter(&self, player_index: usize, game: &Game) {
		let player = &game.players.as_ref().unwrap()[player_index];
		let color = if player.get_color() == NonEmptyCell::Black {
			BLACK
		} else {
			WHITE
		};
		let x_position = if player_index == 0 {
			screen_width() * 0.5 - scale_to_resolution(200.)
		} else {
			screen_width() * 0.5 + scale_to_resolution(200.)
		};
		let y_position = screen_height() * 0.025;
		draw_circle(x_position, y_position, 20., color);
		let text_dimensions = measure_text(&player.name, None, 20, 1.);
		draw_text(&player.name, x_position + 40., y_position, 20., BLACK);
		let captured_text = format!("Captured: {}", self.captured_by_user[player_index]);
		draw_text(&captured_text, x_position + 40., y_position + text_dimensions.height, 20., BLACK);
	}

	pub fn draw_counters(&self, game: &Game) {
		if game.game_mode == GameMode::None || game.game_variant == GameVariant::None {
			return;
		}
		self.draw_player_counter(0, game);
		self.draw_player_counter(1, game);
	}

	pub fn place_stone(&self, x: f32, y: f32, color: Color) {
		let ray;
		if screen_width() > screen_height(){
			ray = screen_height() * 0.8 / (BOARD_SIZE - 1) as f32 / 2. - 2.;}
		else{
			ray = screen_width() * 0.8 / (BOARD_SIZE - 1) as f32 / 2. - 2.;}
		draw_circle(x, y, ray, color);
	}

	pub fn place_all_stones(&self) {
		// unsafe {
			for i in 0..BOARD_SIZE {
				for j in 0..BOARD_SIZE {
					if self.grid[i][j] != Cell::Empty {
						let color = if self.grid[i][j] == Cell::Black { BLACK } else { WHITE };
						self.place_stone(screen_width() * 0.1 + ((screen_width() * 0.8) / (BOARD_SIZE - 1) as f32) * (i as f32), 
							screen_height() * 0.1 + ((screen_height() * 0.8) / (BOARD_SIZE - 1) as f32) * (j as f32), 
							color);
					}
				}
			}
		// }
	}
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "┌───────────────────┐")?;
        for line in self.grid {
            write!(f, "│")?;
            for cell in line {
                write!(f, "{}", cell)?;
            }

            writeln!(f, "│")?;
        }
        writeln!(f, "└───────────────────┘")?;
        Ok(())
    }
}
