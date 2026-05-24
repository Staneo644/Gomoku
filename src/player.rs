use crate::board::NonEmptyCell;

pub enum PlayerType {
	Human,
	AI,
}

pub struct Player {
	pub name: String,
	pub color: NonEmptyCell,
	pub player_type: PlayerType,
	turn_to_play: i32,
}

impl Player {
	pub fn new(name: String, player_type: PlayerType) -> Self {
		Self { name, color: NonEmptyCell::Black, player_type, turn_to_play: 1 }
	}

	pub fn assign_color(&mut self, color: NonEmptyCell) {
		self.color = color;
	}

	pub fn get_color(&self) -> NonEmptyCell {
		self.color
	}

	pub fn is_human(&self) -> bool {
		matches!(self.player_type, PlayerType::Human)
	}

	pub fn is_ai(&self) -> bool {
		matches!(self.player_type, PlayerType::AI)
	}

	pub fn set_number_of_turn(&mut self, turn: i32) {
		self.turn_to_play = turn;
	}

	pub fn get_number_of_turn_mut(&mut self) -> &mut i32 {
		&mut self.turn_to_play
	}

	pub fn get_number_of_turn(&self) -> i32 {
		self.turn_to_play
	}
}