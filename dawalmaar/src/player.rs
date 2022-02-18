use crate::cards::{Card, Suit};

pub struct Player {
	hand: Vec<Card>,
}

impl Player {
	pub fn new() -> Player {
		Player { hand: Vec::new() }
	}

	pub fn add_card(&mut self, card: Card) {
		self.hand.push(card);
	}

	pub fn play_card(&mut self, card: Card) -> Result<Card, ()> {
		let len_before = self.hand.len();
		self.hand.retain(|c| *c != card);
		if len_before == self.hand.len() {
			return Err(());
		}
		Ok(card)
	}

	pub fn can_play(&self, card: Card, suit_in_play: Option<Suit>) -> bool {
		match suit_in_play {
			Some(suit) => {
				// If a suit is in play, the player can only play a card of that suit if they have it.
				card.get_suit() == suit || !self.hand.iter().any(|c| c.get_suit() == suit)
			}
			None => true,
		}
	}

	pub fn sort_hand(&mut self) {
		self.hand.sort();
	}

	pub fn get_hand(&self) -> &Vec<Card> {
		&self.hand
	}

	pub fn is_empty(&self) -> bool {
		self.hand.is_empty()
	}
}

#[cfg(test)]
mod tests {
	use crate::cards::{Rank, Suit};

	use super::*;

	#[test]
	fn test_hand_new() {
		let player = Player::new();
		assert_eq!(player.hand.len(), 0);
	}

	#[test]
	fn test_hand_add_card() {
		let mut player = Player::new();
		player.add_card(Card::new(Suit::Spades, Rank::Two));
		assert_eq!(player.hand.len(), 1);
	}

	#[test]
	fn test_hand_remove_card() {
		let mut player = Player::new();
		player.add_card(Card::new(Suit::Spades, Rank::Two));

		assert!(player
			.play_card(Card::new(Suit::Spades, Rank::Three))
			.is_err());

		assert_eq!(
			player
				.play_card(Card::new(Suit::Spades, Rank::Two))
				.unwrap(),
			Card::new(Suit::Spades, Rank::Two)
		);
		assert!(player.is_empty());

		assert!(player
			.play_card(Card::new(Suit::Spades, Rank::Two))
			.is_err());
	}

	#[test]
	fn test_hand_sort() {
		let mut player = Player::new();
		player.add_card(Card::new(Suit::Spades, Rank::Three));
		player.add_card(Card::new(Suit::Spades, Rank::Two));
		player.add_card(Card::new(Suit::Spades, Rank::Three));
		player.sort_hand();
		assert_eq!(player.hand[0], Card::new(Suit::Spades, Rank::Two));
		assert_eq!(player.hand[1], Card::new(Suit::Spades, Rank::Three));
		assert_eq!(player.hand[2], Card::new(Suit::Spades, Rank::Three));
	}

	#[test]
	fn test_can_play() {
		let mut player = Player::new();
		player.add_card(Card::new(Suit::Spades, Rank::Two));

		// Without a suit in play, they can play any card.
		assert!(player.can_play(Card::new(Suit::Spades, Rank::Three), None));
		assert!(player.can_play(Card::new(Suit::Hearts, Rank::Three), None));

		// With a suit in play and they have that suit, they can only play that suit.
		assert!(player.can_play(Card::new(Suit::Spades, Rank::Three), Some(Suit::Spades)));
		assert!(!player.can_play(Card::new(Suit::Hearts, Rank::Three), Some(Suit::Spades)));

		// With a suit in play and they don't have it they can play any suit.
		assert!(player.can_play(Card::new(Suit::Hearts, Rank::Three), Some(Suit::Clubs)));
		assert!(player.can_play(Card::new(Suit::Spades, Rank::Three), Some(Suit::Clubs)));
	}
}
