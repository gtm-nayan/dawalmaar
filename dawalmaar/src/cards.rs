use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter)]
pub enum Suit {
	Spades,
	Clubs,
	Diamonds,
	Hearts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter)]
pub enum Rank {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
	suit: Suit,
	rank: Rank,
}

impl Card {
	pub fn new(suit: Suit, rank: Rank) -> Card {
		Card { suit, rank }
	}

	pub fn get_suit(&self) -> Suit {
		self.suit
	}

	pub fn get_rank(&self) -> Rank {
		self.rank
	}
}

impl std::fmt::Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?} of {:?}", self.rank, self.suit)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_card_new() {
		let card = Card::new(Suit::Spades, Rank::Two);
		assert_eq!(card.get_suit(), Suit::Spades);
		assert_eq!(card.get_rank(), Rank::Two);
	}

	#[test]
	fn test_card_display() {
		let card = Card::new(Suit::Spades, Rank::Two);
		assert_eq!(format!("{}", card), "Two of Spades");
	}

	#[test]
	fn test_card_eq() {
		let card1 = Card::new(Suit::Spades, Rank::Two);
		let card2 = Card::new(Suit::Spades, Rank::Two);
		assert_eq!(card1, card2);
	}

	#[test]
	fn test_card_ord() {
		let card1 = Card::new(Suit::Spades, Rank::Two);
		let card2 = Card::new(Suit::Spades, Rank::Three);
		assert!(card1 < card2);

		let card3 = Card::new(Suit::Spades, Rank::Three);
		let card4 = Card::new(Suit::Spades, Rank::Two);
		assert!(card3 > card4);

		let card5 = Card::new(Suit::Spades, Rank::Three);
		let card6 = Card::new(Suit::Spades, Rank::Three);
		assert_eq!(card5, card6);
	}
}