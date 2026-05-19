use crate::menu::menu::{Menu, MenuOption, MENU_HEIGHT, MENU_WIDTH, OPTION_HEIGHT, OPTION_WIDTH, MENU_START_POINT};
pub const MODE_OPTIONS: [&str; 2] = ["HUMAN VS HUMAN", "HUMAN VS AI"];
pub const VARIANT_OPTIONS: [&str; 4] = ["STANDARD", "SWAP2", "SINGLE SWAP", "PRO"];


pub struct NewGameMenu {
	mode_options: Vec<MenuOption>,
	variant_options: Vec<MenuOption>,
	game: &mut Game,
}

impl NewGameMenu {
	fn new(game: &mut Game) -> Self {
		let menu =Self {
			mode_options: Vec::new(),
			variant_options: Vec::new(),
			game,
		};

		let mut index = 0;
		for mode in MODE_OPTIONS {
			let y_position = MENU_START_POINT.y + 100. + index as f32 * (OPTION_HEIGHT + 10.);
			let x_position = MENU_START_POINT.x + (MENU_WIDTH - OPTION_WIDTH) / 2.;
			menu.mode_options.push(MenuOption {
				text: mode.to_string(),
				action: || {},
				location: (Vec2::new(x_position, y_position), Vec2::new(x_position + OPTION_WIDTH, y_position + OPTION_HEIGHT))
			});
			index += 1;
		}
		index += 1;
		for variant in VARIANT_OPTIONS {
			let y_position = MENU_START_POINT.y + 100. + index as f32 * (OPTION_HEIGHT + 10.);
			let x_position = MENU_START_POINT.x + (MENU_WIDTH - OPTION_WIDTH) / 2.;
			menu.variant_options.push(MenuOption {
				text: variant.to_string(),
				action: || {},
				location: (Vec2::new(x_position, y_position), Vec2::new(x_position + OPTION_WIDTH, y_position + OPTION_HEIGHT))
			});
			index += 1;
		}
		menu
	}
}