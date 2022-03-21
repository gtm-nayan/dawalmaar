use dawalmaar::{
	enums::{
		PlayCardError,
		StartError::{GameAlreadyStarted, GameNotFull},
		TurnEndResult,
	},
	game::Game as IGame,
};
use indexmap::IndexSet;
use poise::{serenity_prelude::UserId, CreateReply};

use crate::utils::parse_card::parse_card;
pub struct Game {
	i_game: IGame,
	players: IndexSet<UserId>, // todo!() Replace with a bimap
}

impl Default for Game {
	fn default() -> Self {
		Self {
			i_game: IGame::new(),
			players: IndexSet::new(),
		}
	}
}

impl Game {
	pub fn add_player(&mut self, id: UserId, reply: &mut CreateReply) {
		if self.players.contains(&id) {
			reply
				.content("You are already in this game!")
				.ephemeral(true);
		} else if let Ok(_) = self.i_game.add_player() {
			self.players.insert(id);

			reply.content(format!("<@{id}> has joined the game!"));
		} else {
			reply.content("The game is full!").ephemeral(true);
		}
	}

	pub fn get_hand(&mut self, id: UserId, reply: &mut CreateReply) {
		if !self.i_game.has_started() {
			reply
				.content("The game has not started yet.")
				.ephemeral(true);
			return;
		}

		let player_idx = match self.players.get_index_of(&id) {
			Some(idx) => idx,
			_ => {
				reply.content("You are not in the game.").ephemeral(true);
				return;
			}
		};
		let mut message = String::from(
			"You have the following cards in your hand. You can play the boldened ones.\n",
		);

		self.i_game
			.get_hand(player_idx)
			.map(|(can_play, card)| {
				if can_play {
					format!("**{card}**\n")
				} else {
					format!("{card}\n")
				}
			})
			.for_each(|card| message.push_str(&card));

		reply.content(message).ephemeral(true);
	}

	pub fn play_card(&mut self, player_id: UserId, card: String, reply: &mut CreateReply) -> bool {
		let player_idx = match self.players.get_index_of(&player_id) {
			Some(idx) => idx,
			_ => {
				reply.content("You are not in the game.").ephemeral(true);
				return false;
			}
		};

		let card = match parse_card(card) {
			Ok(card) => card,
			_ => {
				reply.content("Invalid card.").ephemeral(true);
				return false;
			}
		};

		match self.i_game.play_card(player_idx, card) {
			Err(variant) => {
				reply
					.content(match variant {
						PlayCardError::NotThisPlayersTurn => "It's not your turn.",
						PlayCardError::GameNotStarted => "The game hasn't started.",
						PlayCardError::CardNotInHand => "You don't have that card.",
						PlayCardError::CantPlaySuit => "Can't play that suit here.",
					})
					.ephemeral(true);
				false
			}

			Ok(TurnEndResult::NextTurn(next_turn)) => {
				let next_player = self.players.get_index(next_turn).unwrap();

				reply.content(format!(
					"<@{player_id}> has played the card. It is now <@{next_player}>'s turn.",
				));
				false
			}

			Ok(TurnEndResult::GameOver(scores)) => {
				let t1 = &scores[0];
				let t2 = &scores[1];

				let winning_team_idx = match t1.get_tens().cmp(&t2.get_tens()) {
					std::cmp::Ordering::Greater => 0,
					std::cmp::Ordering::Less => 1,
					_ if t1.get_total_captured() > t2.get_total_captured() => 0,
					_ => 1,
				};

				reply
					.content(format!(
						"Game over. {} won with {} tens and {} total captured.",
						self.team_members(winning_team_idx),
						scores[winning_team_idx].get_tens(),
						scores[winning_team_idx].get_total_captured()
					))
					.ephemeral(false);
				true
			}
		}
	}

	pub fn team_members(&self, team_idx: usize) -> String {
		format!(
			"<@{}> and <@{}>",
			self.players.get_index(0 + team_idx).unwrap(),
			self.players.get_index(2 + team_idx).unwrap()
		)
	}

	pub fn start(&mut self, reply: &mut CreateReply) {
		match self.i_game.start() {
			Err(GameAlreadyStarted) => reply.content("Game has already started").ephemeral(true),
			Err(GameNotFull) => reply
				.content("Game isn't full yet. You need exactly 4 players to start.")
				.ephemeral(true),

			_ => {
				let current_turn = self.i_game.get_turn();
				let player_id = self.players.get_index(current_turn).unwrap();
				reply
					.content(format!(
						"The game has been started. <@{player_id}> run `/hand` to see your hand."
					))
					.ephemeral(true)
			}
		};
	}
}
