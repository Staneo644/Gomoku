use crate::ai::minimax::ai_move_t;
use crate::board::{Board, NonEmptyCell};
use crate::display::display_message::*;
use crate::event_handler::event_handler::*;
use crate::menu::menu::Menu;
use crate::menu::new_game_menu::NewGameMenu;
use crate::player::*;
use macroquad::prelude::*;
use std::fmt;

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

#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    Playing,
    MainMenu,
    ResumeGame,
    NewGameMenu,
    Finished,
    Exiting,
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
    pub players: Option<[Player; 2]>,
    pub ai_thinking: bool,
    pub ai_timer: f32,
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
            ai_timer: 1.0,
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
        self.ai_timer = 1.0;

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
        }
        self.game_state = state;
    }

    pub fn draw_mouse_hover(&self) {
        if self.game_state != GameState::Playing {
            return;
        }
        let (x, y) = mouse_position();
        if x < screen_width() * 0.1
            || x > screen_width() * 0.9
            || y < screen_height() * 0.1
            || y > screen_height() * 0.9
        {
            return;
        }
        let line_x = screen_width() * 0.1;
        let line_y = screen_height() * 0.1;
        let cell_size_x = screen_width() * 0.8 / 18.;
        let cell_size_y = screen_height() * 0.8 / 18.;
        let cell_size = if cell_size_x < cell_size_y {
            cell_size_x
        } else {
            cell_size_y
        };
        let board_x = ((x - line_x) / cell_size_x - 0.5).floor() + 0.5;
        let board_y = ((y - line_y) / cell_size_y - 0.5).floor() + 0.5;
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
            line_x + (board_x as f32) * cell_size_x + cell_size_x / 2.,
            line_y + (board_y as f32) * cell_size_y + cell_size_y / 2.,
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
        request_new_screen_size(1000., 1000.);
        while self.game_state != GameState::Exiting {
            self.board.draw_board();
            self.board.draw_counters(self);
            self.board.place_all_stones();
            self.draw_mouse_hover();
            event_handler(self).await;
            self.display_message();

            match self.game_state {
                GameState::MainMenu => self.menu.draw(),
                GameState::NewGameMenu => self.new_game_menu.draw(),
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
            if self.is_current_player_ai() {
                self.ai_move().await;
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

    pub fn adapt_to_game_mode_and_variant(&mut self) {
        self.create_players();
        // insert more adaptations based on game mode and variant here
        match self.game_variant {
            GameVariant::Standard => {
                let rng = rand::gen_range(0, 2);
                if rng == 0 {
                    self.players.as_mut().unwrap()[0].assign_color(NonEmptyCell::White);
                    self.current_player = 1;
                } else {
                    self.players.as_mut().unwrap()[1].assign_color(NonEmptyCell::White);
                }
            }
            GameVariant::Swap2 => {
                // Implement Swap2 rules here
            }
            GameVariant::SingleSwap => {
                // Implement Single Swap rules here
            }
            GameVariant::Pro => {
                // Implement Pro rules here
            }
            GameVariant::None => {
                // No variant selected, do nothing
                return;
            }
        }
    }

    pub fn change_player(&mut self) {
        self.current_player = (self.current_player + 1) % 2;
    }

    pub fn get_current_player(&self) -> Option<&Player> {
        self.players
            .as_ref()
            .map(|players| &players[self.current_player])
    }

    pub fn is_current_player_ai(&self) -> bool {
        if self.get_current_player().is_some() {
            self.get_current_player().unwrap().is_ai()
        } else {
            return false;
        }
    }

    pub async fn ai_move(&mut self) {
        if self.ai_thinking {
            self.ai_timer -= get_frame_time();
            if self.ai_timer <= 0. {
                let current_color = self.get_current_player().unwrap().get_color();

                let (x, y) = ai_move_t(&mut self.board, current_color);

                place_stone_handler(self, x, y).await;

                self.ai_timer = 1.0;
                self.ai_thinking = false;
            }
        } else {
            self.ai_thinking = true;
        }
    }
}
