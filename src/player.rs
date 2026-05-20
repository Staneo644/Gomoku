use crate::board::NonEmptyCell;

pub enum PlayerType {
	Human,
	AI,
}

pub struct Player {
	pub name: String,
	pub color: NonEmptyCell,
	pub player_type: PlayerType,
}

impl Player {
	pub fn new(name: String, player_type: PlayerType) -> Self {
		Self { name, color: NonEmptyCell::Black, player_type }
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
}