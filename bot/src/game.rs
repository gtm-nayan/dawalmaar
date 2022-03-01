use dawalmaar::game::Game as IGame;

pub struct Game {
	i_game: IGame,
	players_map: Vec<usize>, // todo!() Replace with a bimap
}

impl Default for Game {
	fn default() -> Self {
		Self {
			i_game: IGame::new(),
			players_map: vec![],
		}
	}
}

impl Game {
	pub fn add_player(
		&mut self,
	) -> (
		/* The text for the message to send */ String,
		/* Whether it should be ephemeral */ bool,
	) {
		if let Ok(player_idx) = self.i_game.add_player() {
			self.players_map.push(player_idx);
			(format!("Player {} joined the game", player_idx), false)
		} else {
			(String::from("Game is full"), true)
		}
	}

	// 	pub fn get_hand(&mut self) {
	// 	}

	// 	pub fn play_card(&mut self){

	// 	}

	// 	pub fn start(&mut self){}
}
