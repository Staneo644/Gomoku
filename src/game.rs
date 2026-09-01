use crate::ai::minimax::ai_move_t;
use crate::board::{Board, NonEmptyCell};
use crate::display::display_message::*;
use crate::event_handler::event_handler::*;
use crate::menu::color_menu::ColorMenu;
use crate::menu::menu::Menu;
use crate::menu::new_game_menu::NewGameMenu;
use crate::menu::settings_menu::SettingsMenu;
use crate::player::*;
use macroquad::prelude::*;
use std::fmt;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Instant;

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameMode::None => write!(f, "None"),
            GameMode::HumanVsHuman => write!(f, "Human vs Human"),
            GameMode::HumanVsAI => write!(f, "Human vs AI"),
        }
    }
}

impl fmt::Display for GameVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameVariant::None => write!(f, "None"),
            GameVariant::Standard => write!(f, "Standard"),
            GameVariant::Swap2 => write!(f, "Swap2"),
            GameVariant::SingleSwap => write!(f, "SingleSwap"),
            GameVariant::Pro => write!(f, "Pro"),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameMode {
    None,
    HumanVsHuman,
    HumanVsAI,
    // insert more modes here
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameVariant {
    None,
    Standard,
    Swap2,
    SingleSwap,
    Pro,
    // insert more variants here
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameState {
    Playing,
    MainMenu,
    ResumeGame,
    NewGameMenu,
    SettingsMenu,
    Finished,
    Exiting,
    PickColor,
    Swap1,
    Swap2,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::Playing => write!(f, "Playing"),
            GameState::MainMenu => write!(f, "MainMenu"),
            GameState::ResumeGame => write!(f, "ResumeGame"),
            GameState::NewGameMenu => write!(f, "NewGameMenu"),
            GameState::SettingsMenu => write!(f, "SettingsMenu"),
            GameState::Finished => write!(f, "Finished"),
            GameState::Exiting => write!(f, "Exiting"),
            GameState::PickColor => write!(f, "PickColor"),
            GameState::Swap1 => write!(f, "Swap1"),
            GameState::Swap2 => write!(f, "Swap2"),
        }
    }
}

pub struct Game {
    pub board: Board,
    pub current_player: usize,
    pub game_mode: GameMode,
    pub game_variant: GameVariant,
    pub game_state: GameState,
    pub message: Option<Message>,
    pub menu: Menu,
    pub new_game_menu: NewGameMenu,
    pub settings_menu: SettingsMenu,
    pub players: Option<[Player; 2]>,
    pub ai_thinking: bool,
    pub ai_delay: f32,
    pub pick_color_menu: ColorMenu,
    target_resolution: u16,
    pub ai_start_time: Option<Instant>,
    ai_result: Option<Receiver<(i32, i32)>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: 0,
            game_mode: GameMode::None,
            game_variant: GameVariant::None,
            game_state: GameState::MainMenu,
            message: None,
            menu: Menu::new(),
            new_game_menu: NewGameMenu::new(),
            players: None,
            ai_thinking: false,
            ai_delay: 1.0,
            settings_menu: SettingsMenu::new(),
            pick_color_menu: ColorMenu::new(),
            target_resolution: 1000,
            ai_result: None,
            ai_start_time: None,
        }
        // display window to pick game mode and variant
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = 0;
        self.game_mode = GameMode::None;
        self.game_variant = GameVariant::None;
        self.game_state = GameState::MainMenu;
        self.new_game_menu = NewGameMenu::new();
        self.players = None;
        self.ai_thinking = false;
        self.ai_delay = 1.0;

        // display window to pick game mode and variant
    }

    pub fn set_game_state(&mut self, state: GameState) {
        if state == GameState::ResumeGame && self.game_mode == GameMode::None {
            self.message = Some(Message::new(
                "No Game to Resume".to_string(),
                MessageType::Error,
            ));
            return;
        } else if state == GameState::ResumeGame {
            self.game_state = GameState::Playing;
            return;
        }
        if state == GameState::NewGameMenu {
            self.reset();
            self.game_state = GameState::NewGameMenu;
            return;
        } else if state == GameState::Swap2 {
            self.get_current_player_mut().unwrap().set_number_of_turn(2);
            self.players.as_mut().unwrap()[(self.current_player + 1) % 2].set_number_of_turn(-1);
        }

        self.game_state = state;
    }

    pub fn draw_mouse_hover(&self) {
        if self.game_state != GameState::Playing {
            return;
        }
        let (x, y) = mouse_position();
        let board_size = screen_width().min(screen_height()) * 0.8;
        if x < screen_width() / 2. - board_size / 2. * 0.95
            || x > screen_width() / 2. + board_size / 2. * 0.95
            || y < screen_height() / 2. - board_size / 2. * 0.95
            || y > screen_height() / 2. + board_size / 2. * 0.95
        {
            return;
        }
        let line_x = screen_width() / 2. - board_size / 2. * 0.9;
        let line_y = screen_height() / 2. - board_size / 2. * 0.9;
        let cell_size = board_size * 0.9 / 18.;
        let board_x = ((x - line_x) / cell_size - 0.5).floor() + 0.5;
        let board_y = ((y - line_y) / cell_size - 0.5).floor() + 0.5;
        let color = if self.get_current_player().unwrap().get_color() == NonEmptyCell::Black {
            Color {
                r: (0.),
                g: (0.),
                b: (0.),
                a: (0.5),
            }
        } else {
            Color {
                r: (1.),
                g: (1.),
                b: (1.),
                a: (0.5),
            }
        };
        draw_circle(
            line_x + (board_x as f32) * cell_size + cell_size / 2.,
            line_y + (board_y as f32) * cell_size + cell_size / 2.,
            cell_size / 2. - 2.,
            color,
        );
    }

    fn display_message(&mut self) {
        if let Some(message) = &mut self.message {
            message.display_message();
            message.timer -= get_frame_time();
            if message.timer <= 0. {
                self.message = None;
            }
        }
    }

    pub async fn launch(&mut self) {
        while self.game_state != GameState::Exiting {
            self.board.draw_board();
            self.board.draw_counters(self);
            self.board.place_all_stones();
            self.board.draw_ai_timer(self);
            self.draw_mouse_hover();
            event_handler(self).await;
            self.display_message();
            match self.game_state {
                GameState::MainMenu => self.menu.draw(),
                GameState::NewGameMenu => self.new_game_menu.draw(),
                GameState::SettingsMenu => self.settings_menu.draw(),
                GameState::PickColor => {
                    if !self.is_current_player_ai() {
                        self.pick_color_menu
                            .draw(self.game_variant == GameVariant::Swap2)
                    } else {
                        // put function to make AI pick color here
                        //for now AI will always pick black
                        self.change_players_colors(NonEmptyCell::Black);
                    }
                }
                GameState::Swap2 => {
                    self.game_state = GameState::Playing;
                    self.get_current_player_mut().unwrap().set_number_of_turn(2);
                    self.players.as_mut().unwrap()[(self.current_player + 1) % 2]
                        .set_number_of_turn(-1);
                    self.game_variant = GameVariant::SingleSwap;
                }
                GameState::Finished => {
                    let winner = self.players.as_ref().unwrap()[self.current_player]
                        .name
                        .clone();
                    self.message = Some(Message::new(
                        format!("{} wins! Starting a new game...", winner),
                        MessageType::Info,
                    ));
                    self.reset();
                }
                _ => {}
            }

            if self.is_current_player_ai() && self.game_state == GameState::Playing {
                self.update_ai().await;
            }
            if self.target_resolution != screen_width() as u16
                || self.target_resolution != screen_height() as u16
            {
                request_new_screen_size(
                    self.target_resolution as f32,
                    self.target_resolution as f32,
                );
            }
            next_frame().await;
        }
    }

    pub fn create_players(&mut self) {
        match self.game_mode {
            GameMode::HumanVsHuman => {
                self.players = Some([
                    Player::new("Player 1".to_string(), PlayerType::Human),
                    Player::new("Player 2".to_string(), PlayerType::Human),
                ]);
            }
            GameMode::HumanVsAI => {
                self.players = Some([
                    Player::new("Player 1".to_string(), PlayerType::Human),
                    Player::new("AI".to_string(), PlayerType::AI),
                ]);
            }
            _ => {
                self.players = None;
                return;
            }
        }
    }

    pub fn set_random_first_player(&mut self) {
        rand::srand(macroquad::miniquad::date::now() as _);
        let rng = rand::gen_range(0, 2);
        if rng == 0 {
            self.players.as_mut().unwrap()[0].assign_color(NonEmptyCell::White);
            self.current_player = 1;
        } else {
            self.players.as_mut().unwrap()[1].assign_color(NonEmptyCell::White);
        }
    }

    pub fn adapt_to_game_mode_and_variant(&mut self) {
        self.create_players();
        self.set_random_first_player();
        match self.game_variant {
            GameVariant::Standard => {
                return;
            }
            GameVariant::Swap2 | GameVariant::SingleSwap => {
                if self.get_current_player().is_some() {
                    self.get_current_player_mut().unwrap().set_number_of_turn(3);
                    self.players.as_mut().unwrap()[(self.current_player + 1) % 2]
                        .set_number_of_turn(-1);
                }
            }
            GameVariant::Pro => {
                return;
            }
            GameVariant::None => {
                // No variant selected, do nothing
                return;
            }
        }
    }

    pub fn change_player(&mut self) {
        if self.get_current_player().unwrap().get_number_of_turn() > 1 {
            let color = self.get_current_player().unwrap().get_color();
            self.get_current_player_mut()
                .unwrap()
                .assign_color(color.get_opposite_non_empty());
            self.players.as_mut().unwrap()[(self.current_player + 1) % 2].assign_color(color);
            *self
                .get_current_player_mut()
                .unwrap()
                .get_number_of_turn_mut() -= 1;
        } else if self.get_current_player().unwrap().get_number_of_turn() == 1 {
            self.current_player = (self.current_player + 1) % 2;
        }
        if self.get_current_player().unwrap().get_number_of_turn() == -1 {
            self.get_current_player_mut().unwrap().set_number_of_turn(1);
            self.game_state = GameState::PickColor;
        }
    }

    pub fn change_players_colors(&mut self, color: NonEmptyCell) {
        let current_color = self.get_current_player().unwrap().get_color();
        self.get_current_player_mut().unwrap().assign_color(color);
        self.players.as_mut().unwrap()[(self.current_player + 1) % 2]
            .assign_color(color.get_opposite_non_empty());
        self.message = Some(Message::new(
            format!(
                "{} picked {} color",
                self.get_current_player().unwrap().name,
                if color == NonEmptyCell::Black {
                    "Black"
                } else {
                    "White"
                }
            ),
            MessageType::Info,
        ));
        if current_color != color {
            self.current_player = (self.current_player + 1) % 2;
        }
        self.game_state = GameState::Playing;
    }

    pub fn get_current_player(&self) -> Option<&Player> {
        self.players
            .as_ref()
            .map(|players| &players[self.current_player])
    }

    pub fn get_current_player_mut(&mut self) -> Option<&mut Player> {
        self.players
            .as_mut()
            .map(|players| &mut players[self.current_player])
    }

    pub fn is_current_player_ai(&self) -> bool {
        if self.get_current_player().is_some() {
            self.get_current_player().unwrap().is_ai()
        } else {
            return false;
        }
    }

    // pub async fn ai_move(&mut self) {
    //     if self.ai_thinking {
    //         self.ai_delay -= get_frame_time();
    //         if self.ai_delay <= 0. {
    // 			let start_time = std::time::Instant::now();
    //             let current_color = self.get_current_player().unwrap().get_color();

    //             let (x, y) = ai_move_t(&mut self.board, current_color, self.game_variant);

    //             place_stone_handler(self, x, y).await;
    // 			println!("AI move took: {:?}", start_time.elapsed());

    //             self.ai_delay = 1.0;
    //             self.ai_thinking = false;
    //         }
    //     } else {
    //         self.ai_thinking = true;
    //     }
    // }

    pub async fn update_ai(&mut self) {
        if let Some(receiver) = &self.ai_result {
            if let Ok((x, y)) = receiver.try_recv() {
                place_stone_handler(self, x as usize, y as usize).await;
                self.message = Some(Message::new(
                    format!(
                        "AI took {} ms to make a move",
                        self.ai_start_time.unwrap().elapsed().as_millis()
                    ),
                    MessageType::Info,
                ));
                self.ai_result = None;
                self.ai_start_time = None;
                self.ai_thinking = false;
                self.ai_delay = 1.0;
            }
        } else if self.ai_thinking && self.ai_delay > 0. {
            self.ai_delay -= get_frame_time();
        } else if self.ai_thinking && self.ai_delay <= 0. && self.ai_start_time.is_none() {
            let (tx, rx) = mpsc::channel();
            let mut board_clone = self.board.clone();
            let current_color = self.get_current_player().unwrap().get_color();
            let game_variant = self.game_variant;
            self.ai_start_time = Some(Instant::now());
            thread::spawn(move || {
                let (x, y) = ai_move_t(&mut board_clone, current_color, game_variant);
                tx.send((x as i32, y as i32)).unwrap();
            });
            self.ai_result = Some(rx);
        } else if !self.ai_thinking {
            self.ai_thinking = true;
        }
    }

    pub fn resize_window(&mut self, resolution: u16) {
        request_new_screen_size(resolution as f32, resolution as f32);
        self.target_resolution = resolution;
    }
}
