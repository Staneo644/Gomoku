use smallvec::SmallVec;
use std::{collections::HashMap, fmt};
pub const BOARD_SIZE: usize = 19;
use crate::utils::scale_to_resolution;
use macroquad::prelude::*;

use crate::game::{Game, GameMode, GameVariant};

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
    Invalid,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::White => write!(f, "0"),
            Cell::Black => write!(f, "X"),
            Cell::Invalid => write!(f, ""),
        }
    }
}

impl Cell {
    pub fn get_opposite(&self) -> Cell {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            Cell::Invalid => Cell::Invalid,
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

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub struct Board {
    pub(crate) grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    pub moves: Vec<Move>,
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
            captured_by_user: [0; 2],
            neighboring_empty_cells: HashMap::new(),
            occupied_positions: HashMap::new(),
        }
    }
}

impl Board {
    pub fn draw_board(&self) {
        clear_background(Color {
            r: (0.5),
            g: (0.2),
            b: (0.2),
            a: (1.),
        });

        let size = screen_width().min(screen_height());
        let board_size = size * 0.8;
        let board_start_x = (screen_width() - board_size) / 2.;
        let board_start_y = (screen_height() - board_size) / 2.;
        draw_rectangle(board_start_x, board_start_y, board_size, board_size, BEIGE);
        let line_thickness = 1.;
        let first_height_line = board_start_y + 0.05 * board_size;
        for i in 0..BOARD_SIZE {
            draw_line(
                board_start_x + 0.05 * board_size,
                first_height_line + ((board_size * 0.9) / (BOARD_SIZE - 1) as f32) * (i as f32),
                board_start_x + 0.95 * board_size,
                first_height_line + ((board_size * 0.9) / (BOARD_SIZE - 1) as f32) * (i as f32),
                line_thickness,
                DARKGRAY,
            );
        }
        let first_width_line = board_start_x + 0.05 * board_size;
        for i in 0..BOARD_SIZE {
            draw_line(
                first_width_line + ((board_size * 0.9) / (BOARD_SIZE - 1) as f32) * (i as f32),
                board_start_y + 0.05 * board_size,
                first_width_line + ((board_size * 0.9) / (BOARD_SIZE - 1) as f32) * (i as f32),
                board_start_y + board_size * 0.95,
                line_thickness,
                DARKGRAY,
            );
        }
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let ray;
                let cell_size = board_size * 0.8 / 18.;
                if (i + 3) % 6 == 0 && (j + 3) % 6 == 0 {
                    ray = cell_size * 0.1;
                } else {
                    ray = cell_size * 0.05;
                }
                draw_circle(
                    first_width_line + (board_size * 0.9 / (BOARD_SIZE - 1) as f32) * (i as f32),
                    first_height_line + (board_size * 0.9 / (BOARD_SIZE - 1) as f32) * (j as f32),
                    ray,
                    DARKGRAY,
                );
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
            screen_width() * 0.5 - scale_to_resolution(200., true)
        } else {
            screen_width() * 0.5 + scale_to_resolution(200., true)
        };
        let y_position = scale_to_resolution(25., false);
        draw_circle(x_position, y_position, 20., color);

        let text_dimensions = measure_text(
            &player.name,
            None,
            scale_to_resolution(20., false) as u16,
            1.,
        );
        draw_text(
            &player.name,
            x_position + 40.,
            y_position,
            scale_to_resolution(20., false),
            BLACK,
        );

        let capture_index = if player.get_color() == NonEmptyCell::Black {
            0
        } else {
            1
        };
        let captured_text = format!("Captured: {}", self.captured_by_user[capture_index]);
        draw_text(
            &captured_text,
            x_position + 40.,
            y_position + text_dimensions.height,
            scale_to_resolution(20., false),
            BLACK,
        );
    }

    pub fn draw_counters(&self, game: &Game) {
        if game.game_mode == GameMode::None || game.game_variant == GameVariant::None {
            return;
        }
        self.draw_player_counter(0, game);
        self.draw_player_counter(1, game);
    }

    pub fn place_stone(&self, x: f32, y: f32, color: Color) {
        let size = screen_width().min(screen_height());
        let board_size = size * 0.8;
        let ray = board_size * 0.8 / (BOARD_SIZE - 1) as f32 / 2. - 2.;

        draw_circle(x, y, ray, color);
    }

    pub fn place_all_stones(&self) {
        let board_size = screen_width().min(screen_height()) * 0.8;
        let line_x = screen_width() / 2. - board_size / 2. * 0.9;
        let line_y = screen_height() / 2. - board_size / 2. * 0.9;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.grid[i][j] != Cell::Empty {
                    let color = if self.grid[i][j] == Cell::Black {
                        BLACK
                    } else {
                        WHITE
                    };
                    self.place_stone(
                        line_x + ((board_size * 0.9) / (BOARD_SIZE - 1) as f32) * (i as f32),
                        line_y + ((board_size * 0.9) / (BOARD_SIZE - 1) as f32) * (j as f32),
                        color,
                    );
                }
            }
        }
    }

    pub fn draw_ai_timer(&self, game: &Game) {
        if game.game_mode == GameMode::HumanVsHuman || game.game_variant == GameVariant::None {
            return;
        }
        let elapsed_time = if let Some(start_time) = game.ai_start_time {
            start_time.elapsed().as_millis() as f32
        } else {
            0.0
        };
        let timer_text = format!("AI thinking: {:.3}ms", elapsed_time);
        let text_dimensions = measure_text(
            &timer_text,
            None,
            scale_to_resolution(40., false) as u16,
            1.,
        );
        draw_rectangle(
            screen_width() * 0.5 - text_dimensions.width / 2.,
            scale_to_resolution(40., false),
            text_dimensions.width,
            text_dimensions.height,
            Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.5,
            },
        );
        draw_text(
            &timer_text,
            screen_width() * 0.5 - text_dimensions.width / 2.,
            scale_to_resolution(50., false),
            scale_to_resolution(40., false),
            WHITE,
        );
        if game.ai_start_time.is_some() {
            println!("AI thinking: {:.3}ms", elapsed_time);
        }
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
