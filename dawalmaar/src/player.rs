use crate::{
	cards::{Card, Suit},
	enums::PlayCardError::{self, *},
};
use std::collections::BTreeSet;

pub struct Player {
	// Game is played with only one deck, so a set is used to store the cards.
	// Additional benefit is that we get automatic sorting of the cards.
	hand: BTreeSet<Card>,
	captured: Vec<Card>
}

impl Player {
	pub fn add_card(&mut self, card: Card) {
		self.hand.insert(card);
	}

	pub fn can_play(&self, card: &Card, suit_in_play: Option<Suit>) -> bool {
		match suit_in_play {
			Some(suit) => {
				card.get_suit() == suit || !self.hand.iter().any(|c| c.get_suit() == suit)
			}
			None => true,
		}
	}

	pub fn get_hand(&self) -> &BTreeSet<Card> {
		&self.hand
	}

	pub fn is_empty(&self) -> bool {
		self.hand.is_empty()
	}

	pub fn new() -> Player {
		Player {
			hand: BTreeSet::new(),
			captured: Vec::new(),
		}
	}

	pub fn play_card(
		&mut self,
		card: Card,
		suit_in_play: Option<Suit>,
	) -> Result<Card, PlayCardError> {
		if !(self.can_play(&card, suit_in_play)) {
			Err(CantPlaySuit)
		} else if self.hand.remove(&card) {
			Ok(card)
		} else {
			Err(CardNotInHand)
		}
	}

	pub fn capture(&mut self, cards: Vec<Card>) {
		self.captured.extend(cards);
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
	fn test_hand_sort() {
		let mut player = Player::new();
		player.add_card(Card::new(Suit::Spades, Rank::Three));
		player.add_card(Card::new(Suit::Spades, Rank::Two));
		player.add_card(Card::new(Suit::Clubs, Rank::Three));
		player.get_hand().iter().for_each(|c| {
			dbg!(c);
		});
	}

	#[test]
	fn test_play_card() {
		let mut player = Player::new();

		player.add_card(Card::new(Suit::Spades, Rank::Two));
		player.add_card(Card::new(Suit::Spades, Rank::Three));
		player.add_card(Card::new(Suit::Clubs, Rank::Three));
		player.add_card(Card::new(Suit::Clubs, Rank::Four));
		player.add_card(Card::new(Suit::Diamonds, Rank::Four));

		// Suit in play is None, so player can play any card.
		assert_eq!(
			player.play_card(Card::new(Suit::Spades, Rank::Two), None),
			Ok(Card::new(Suit::Spades, Rank::Two))
		);

		// provided they have it
		assert_eq!(
			player.play_card(Card::new(Suit::Spades, Rank::Four), None),
			Err(CardNotInHand)
		);

		// Suit in play is Spades, so player can only play Spades cards.
		assert_eq!(
			player.play_card(Card::new(Suit::Spades, Rank::Three), Some(Suit::Spades)),
			Ok(Card::new(Suit::Spades, Rank::Three))
		);

		// Suit in play is Hearts, but player doesn't have any Hearts cards, so can play any card.
		assert_eq!(
			player.play_card(Card::new(Suit::Clubs, Rank::Three), Some(Suit::Hearts)),
			Ok(Card::new(Suit::Clubs, Rank::Three))
		);

		// Suit in play is Diamonds, but player tries to play a Clubs card while holding a Diamonds card, so can't play.
		assert_eq!(
			player.play_card(Card::new(Suit::Clubs, Rank::Four), Some(Suit::Diamonds)),
			Err(CantPlaySuit)
		);

		// Suit in play is Diamonds, but player tries to play a Diamonds card they don't have.
		assert_eq!(
			player.play_card(Card::new(Suit::Diamonds, Rank::Five), Some(Suit::Diamonds)),
			Err(CardNotInHand)
		);
	}
}
