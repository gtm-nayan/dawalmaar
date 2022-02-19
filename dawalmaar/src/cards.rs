use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter)]
pub enum Suit {
	Spades = 100,
	Clubs = 200,
	Diamonds = 300,
	Hearts = 400,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter)]
pub enum Rank {
	Two = 2,
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

	pub fn get_value(&self, suit_in_play: Suit, trump_suit: Suit) -> i32 {
		let temp = (self.rank as i32) + (self.suit as i32);

		if !(self.suit == suit_in_play) {
			if self.suit == trump_suit {
				temp + 1000
			} else {
				temp - 1000
			}
		} else {
			temp
		}
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

	#[test]
	fn test_card_get_value() {
		assert_eq!(
			Card::new(Suit::Spades, Rank::Two).get_value(Suit::Spades, Suit::Spades),
			102
		);
		assert_eq!(
			Card::new(Suit::Spades, Rank::Two).get_value(Suit::Spades, Suit::Hearts),
			102
		);
		assert_eq!(
			Card::new(Suit::Spades, Rank::Two).get_value(Suit::Hearts, Suit::Spades),
			1102
		);
		assert_eq!(
			Card::new(Suit::Spades, Rank::Two).get_value(Suit::Hearts, Suit::Hearts),
			102 - 1000
		);
	}
}
