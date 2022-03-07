use dawalmaar::{
	enums::{
		PlayCardError,
		StartError::{GameAlreadyStarted, GameNotFull},
		TurnEndResult,
	},
	game::Game as IGame,
};
use indexmap::IndexSet;
use poise::serenity_prelude::UserId;

use crate::utils::parse_card::parse_card;
pub struct Game {
	i_game: IGame,
	players: IndexSet<UserId>, // todo!() Replace with a bimap
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
			players: IndexSet::new(),
		}
	}
}

impl Game {
	pub fn add_player(&mut self, id: UserId) -> BasicResponse {
		if self.players.contains(&id) {
			BasicResponse {
				message: "You have already joined the game.".into(),
				ephemeral: true,
			}
		} else if let Ok(_) = self.i_game.add_player() {
			self.players.insert(id);

			BasicResponse {
				message: format!("<@{id}> has joined the game"),
				ephemeral: false,
			}
		} else {
			BasicResponse {
				message: "Game is already full".into(),
				ephemeral: true,
			}
		}
	}

	pub fn get_hand(&mut self, id: UserId) -> BasicResponse {
		if !self.i_game.has_started() {
			return BasicResponse {
				message: "The game hasn't started yet".into(),
				ephemeral: true,
			};
		}

		let player_idx = match self.players.get_index_of(&id) {
			Some(idx) => idx,
			_ => {
				return BasicResponse {
					message: "You're not in the game.".into(),
					ephemeral: true,
				}
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
			.for_each(|ref card| message.push_str(card));

		BasicResponse {
			message,
			ephemeral: true,
		}
	}

	pub fn play_card(&mut self, player_id: UserId, card: String) -> (bool, BasicResponse) {
		let player_idx = match self.players.get_index_of(&player_id) {
			Some(idx) => idx,
			_ => {
				return (
					false,
					BasicResponse {
						message: "You're not in the game.".into(),
						ephemeral: true,
					},
				);
			}
		};

		let card = match parse_card(card) {
			Ok(card) => card,
			_ => {
				return (
					false,
					BasicResponse {
						message: "Please provide a valid card.".into(),
						ephemeral: true,
					},
				);
			}
		};

		return (
			false,
			match self.i_game.play_card(player_idx, card) {
				Ok(TurnEndResult::NextTurn(next_turn)) => {
					let next_player = self.players.get_index(next_turn).unwrap();

					BasicResponse {
						message: format!(
							"<@{player_id}> played {card}. It is now <@{next_player}>'s turn."
						),
						ephemeral: false,
					}
				}
				Err(variant) => BasicResponse {
					message: match variant {
						PlayCardError::NotThisPlayersTurn => "It's not your turn.",
						PlayCardError::GameNotStarted => "The game hasn't started.",
						PlayCardError::CardNotInHand => "You don't have that card.",
						PlayCardError::CantPlaySuit => "Can't play that suit here.",
					}
					.into(),
					ephemeral: true,
				},

				Ok(TurnEndResult::GameOver(scores)) => {
					// todo!() clean up this jank
					let team1 = format!(
						"<@{}> and <@{}>",
						self.players.get_index(0).unwrap(),
						self.players.get_index(2).unwrap()
					);
					let team2 = format!(
						"<@{}> and <@{}>",
						self.players.get_index(1).unwrap(),
						self.players.get_index(3).unwrap()
					);

					let (team1_tens, team2_tens) = (scores[0].get_tens(), scores[1].get_tens());
					let (team1_total, team2_total) = (
						scores[0].get_total_captured(),
						scores[1].get_total_captured(),
					);

					let winning_team = if team1_tens > team2_tens {
						// Team 1 has more tens
						(team1, team1_tens, team1_total)
					} else if team2_tens > team1_tens {
						// Team 2 has more tens
						(team2, team2_tens, team2_total)
					} else if team1_total > team2_total {
						// Both have 2 tens but team 1 has more total tricks.
						(team1, team1_tens, team1_total)
					} else {
						// Both have 2 tens but team 2 has more total tricks.
						(team2, team2_tens, team2_total)
					};

					return (
						true,
						BasicResponse {
							message: format!(
								"Game over. {} won with {} tens and {} total captured.",
								winning_team.0, winning_team.1, winning_team.2
							),
							ephemeral: false,
						},
					);
				}
			},
		);
	}

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
			_ => {
				let current_turn = self.i_game.get_turn();
				let player_id = self.players.get_index(current_turn).unwrap();
				BasicResponse {
					message: format!(
						"The game has been started. <@{player_id}> run `/hand` to see your hand."
					),
					ephemeral: true,
				}
			}
		}
	}
}
