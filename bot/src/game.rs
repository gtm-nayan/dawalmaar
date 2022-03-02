use dawalmaar::enums::StartError::{GameAlreadyStarted, GameNotFull};
use dawalmaar::game::Game as IGame;
pub struct Game {
	i_game: IGame,
	players_map: Vec<usize>, // todo!() Replace with a bimap
}

pub struct BasicResponse {
	/// The text content for the message to send.
	pub message: String,
	pub ephemeral: bool,
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
	pub fn add_player(&mut self) -> BasicResponse {
		if let Ok(player_idx) = self.i_game.add_player() {
			self.players_map.push(player_idx);

			BasicResponse {
				message: format!("Player {} has joined the game", player_idx),
				ephemeral: false,
			}
		} else {
			BasicResponse {
				message: "Game is already full".into(),
				ephemeral: true,
			}
		}
	}

	// 	pub fn get_hand(&mut self) {
	// 	}

	// 	pub fn play_card(&mut self){

	// 	}

	pub fn start(&mut self) -> BasicResponse {
		match self.i_game.start() {
			Err(GameAlreadyStarted) => BasicResponse {
				message: "Game has already started.".into(),
				ephemeral: true,
			},
			Err(GameNotFull) => BasicResponse {
				message: "Game isn't full yet. You need exactly 4 players to start.".into(),
				ephemeral: false,
			},
			Ok(_) => BasicResponse {
				message: "The game has started".into(),
				ephemeral: true,
			},
		}
	}
}
