use dawalmaar::{
	cards::{Card, Rank, Suit},
	game::Game as IGame,
};

pub struct Game {
	i_game: IGame,
	players_map: Vec<usize>,
}

// impl Game {
// 	pub fn new() -> Self {
// 		Self {
// 			i_game: IGame::new(),
// 			players_map: vec![],
// 		}
// 	}
// }

// impl Game {
// 	pub fn add_player(&mut self){}

// 	pub fn get_hand(&mut self) {
// 	}

// 	pub fn play_card(&mut self){

// 	}

// 	pub fn start(&mut self){}
// }
