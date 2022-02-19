use crate::{
	cards::{Card, Suit},
	deck::Deck,
	enums::{PlayCardError, TrickEndResult, StartError},
	player::Player,
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

	pub fn get_hand(&self, player_idx: usize) -> Vec<(bool, Card)> {
		let player = &self.players[player_idx];
		player
			.get_hand()
			.iter()
			.map(|c| (player.can_play(c, self.suit_in_play), *c))
			.collect()
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

	pub fn new() -> Game {
		Game {
			players: Vec::new(),
			started: false,
			suit_in_play: None,
			trump_suit: None,
			turn: 0,
			previous_trick_winner: 0,
			table: [None, None, None, None],
		}
	}

	fn next_turn(&mut self) -> usize {
		let temp = (self.turn + 1) % 4;
		let mut highest_card = i32::MIN;
		let mut hand_winner = 0;
		let mut unwrapped = Vec::new();

		if temp == self.previous_trick_winner {
			// Next trick
			for (i, _card) in self.table.iter().enumerate() {
				let card = _card.unwrap();
				let value = card.get_value(self.suit_in_play.unwrap(), self.trump_suit);
				if value > highest_card {
					highest_card = value;
					hand_winner = i;
				}
				unwrapped.push(card);
			}
			self.suit_in_play = None;
			self.players[hand_winner].capture(unwrapped);
			self.turn = hand_winner;
			self.previous_trick_winner = hand_winner;
		} else {
			self.turn = temp;
		};
		self.turn
	}

	pub fn play_card(
		&mut self,
		player_idx: usize,
		card: Card,
	) -> Result<TrickEndResult, PlayCardError> {
		if !self.started {
			return Err(PlayCardError::GameNotStarted);
		}

		if player_idx != self.turn {
			return Err(PlayCardError::NotThisPlayersTurn);
		}

		let player = &mut self.players[player_idx];
		let card = player.play_card(card, self.suit_in_play)?;

		if self.suit_in_play.is_none() {
			self.suit_in_play = Some(card.get_suit());
		} else if self.trump_suit.is_none() && card.get_suit() != self.suit_in_play.unwrap() {
			self.trump_suit = Some(card.get_suit());
		}
		self.table[player_idx] = Some(card);
		let next_turn = self.next_turn();

		if self.is_over() {
			Ok(TrickEndResult::GameOver)
		} else {
			Ok(TrickEndResult::NextTrick(next_turn))
		}
	}

	pub fn start(&mut self) -> Result<(), StartError> {
		if !self.is_full() {
			Err(StartError::GameNotFull)
		} else if self.started {
			Err(StartError::GameAlreadyStarted)
		} else {
			self.started = true;
			self.deal_cards();
			Ok(())
		}
	}
}
