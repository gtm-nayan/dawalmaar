use crate::{
	cards::{Card, Rank, Suit},
	deck::Deck,
	enums::{PlayCardError, StartError, TurnEndResult},
	player::Player,
	teams::Team,
};

pub struct Game {
	players: Vec<Player>,
	started: bool,
	suit_in_play: Option<Suit>,
	trump_suit: Option<Suit>,
	turn: usize,
	previous_trick_winner: usize,
	table: [Option<Card>; 4],
}

impl Game {
	pub fn new() -> Self {
		Self {
			players: Vec::new(),
			started: false,
			suit_in_play: None,
			trump_suit: None,
			turn: 0,
			previous_trick_winner: 0,
			table: [None; 4],
		}
	}
}

impl Game {
	pub fn add_player(&mut self) -> Result<usize, ()> {
		if self.is_full() {
			return Err(());
		}
		self.players.push(Player::new());
		Ok(self.players.len() - 1)
	}

	fn deal_cards(&mut self) {
		let mut deck = Deck::new_shuffled();

		for player in self.players.iter_mut() {
			for _ in 0..13 {
				player.add_card(deck.deal_card().unwrap());
			}
		}
	}

	fn end_trick(&mut self) -> usize {
		let mut highest_card_value = i32::MIN;
		let mut trick_winner = 0;
		let mut unwrapped = Vec::new();
		for (i, _card) in self.table.iter_mut().enumerate() {
			let card = _card
				.take()
				.expect("Table should have all cards by the end of the trick");

			let value = card.get_value(self.suit_in_play.unwrap(), self.trump_suit);

			if value > highest_card_value {
				highest_card_value = value;
				trick_winner = i;
			}
			unwrapped.push(card);
		}
		self.suit_in_play = None;
		self.players[trick_winner].capture(unwrapped);
		self.previous_trick_winner = trick_winner;
		trick_winner
	}

	/// Returns the hand of the player at the given index as a collection of tuples
	/// where the first element is a boolean indicating whether the player can play
	/// the card and the second element is the card itself.
	pub fn get_hand(&self, player_idx: usize) -> impl Iterator<Item = (bool, Card)> + '_ {
		let player = &self.players[player_idx];
		player
			.get_hand()
			.iter()
			.map(|card| (player.can_play(card, self.suit_in_play), *card))
	}

	pub fn has_started(&self) -> bool {
		self.started
	}

	pub fn is_full(&self) -> bool {
		self.players.len() == 4
	}

	pub fn is_over(&self) -> bool {
		self.players.iter().all(|player| player.is_empty())
	}

	fn next_turn(&mut self) -> TurnEndResult {
		let temp = (self.turn + 1) % 4;

		self.turn = if temp == self.previous_trick_winner {
			self.end_trick()
		} else {
			temp
		};

		if self.is_over() {
			TurnEndResult::GameOver(self.tally_scores())
		} else {
			TurnEndResult::NextTurn(self.turn)
		}
	}

	pub fn play_card(
		&mut self,
		player_idx: usize,
		card: Card,
	) -> Result<TurnEndResult, PlayCardError> {
		if !self.started {
			return Err(PlayCardError::GameNotStarted);
		}

		if player_idx != self.turn {
			return Err(PlayCardError::NotThisPlayersTurn);
		}

		let player = &mut self.players[player_idx];
		let card = player.play_card(card, self.suit_in_play)?;
		let played_suit = card.get_suit();

		if let Some(suit_in_play) = self.suit_in_play {
			if self.trump_suit.is_none() && played_suit != suit_in_play {
				self.trump_suit = Some(played_suit);
			}
		} else {
			self.suit_in_play = Some(played_suit);
		}

		self.table[player_idx] = Some(card);
		Ok(self.next_turn())
	}

	pub fn start(&mut self) -> Result<(), StartError> {
		if !self.is_full() {
			Err(StartError::GameNotFull)
		} else if self.started {
			Err(StartError::GameAlreadyStarted)
		} else {
			self.deal_cards();
			self.started = true;
			Ok(())
		}
	}

	fn tally_scores(&self) -> [Team; 2] {
		let mut scores = [Team::new(), Team::new()];
		for (i, player) in self.players.iter().enumerate() {
			let captured_cards = player.get_captured();

			let team = &mut scores[i % 2];
			team.add_to_captured(captured_cards.len() as u8);

			for card in captured_cards {
				if card.get_rank() == Rank::Ten {
					team.increment_tens();
				}
			}
		}
		scores
	}
}
