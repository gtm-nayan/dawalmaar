use crate::cards::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

pub struct Deck {
	cards: Vec<Card>,
}

impl Deck {
	pub fn new() -> Deck {
		let mut cards = Vec::new();
		for suit in Suit::iter() {
			for rank in Rank::iter() {
				cards.push(Card::new(suit, rank));
			}
		}
		Deck { cards }
	}

	pub fn shuffle(&mut self) {
		let mut rng = rand::thread_rng();
		self.cards.shuffle(&mut rng);
	}

	pub fn deal_card(&mut self) -> Option<Card> {
		self.cards.pop()
	}

	pub fn is_empty(&self) -> bool {
		self.cards.is_empty()
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_deck_new() {
		let deck = Deck::new();
		assert_eq!(deck.cards.len(), 52);
	}

	#[test]
	fn test_deck_shuffle() {
		let mut deck = Deck::new();
		deck.shuffle();
		assert_eq!(deck.cards.len(), 52);
	}

	#[test]
	fn test_deck_deal_card() {
		let mut deck = Deck::new();
		deck.shuffle();
		deck.deal_card().unwrap();
		assert_eq!(deck.cards.len(), 51);
	}

	#[test]
	fn test_deck_is_empty() {
		let mut deck = Deck::new();

		for _ in 0..52 {
			deck.deal_card().unwrap();
		}
		assert!(deck.is_empty());
	}
}
