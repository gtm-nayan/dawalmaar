use dawalmaar::cards::{Card, Rank, Suit};
use lazy_static::lazy_static;
use regex::{Match, Regex};

lazy_static! {
	static ref CARD_REGEX: Regex = Regex::new(r#"(?P<long>^(?P<rank>ace|king|queen|jack|(?:[akqj2-9]|10))\s+of\s+(?P<suit>spade|club|diamond|heart)s?)$|(?P<short>^(?P<suit2>s|c|d|h)(?P<rank2>[akqj2-9]|10))$"#).unwrap();
}

#[derive(Debug)]
pub struct ParseCardError;

pub fn parse_card(mut card: String) -> Result<Card, ParseCardError> {
	card.make_ascii_lowercase();
	let matches = CARD_REGEX.captures(&card).ok_or(ParseCardError)?;

	let (rank, suit) = if matches.name("long").is_some() {
		(matches.name("rank"), matches.name("suit"))
	} else {
		(matches.name("rank2"), matches.name("suit2"))
	};

	Ok(Card::new(parse_suit(suit), parse_rank(rank)))
}

fn parse_suit(suit: Option<Match>) -> Suit {
	let id = &suit.unwrap().as_str()[..1];

	match id {
		"h" => Suit::Hearts,
		"d" => Suit::Diamonds,
		"c" => Suit::Clubs,
		"s" => Suit::Spades,
		_ => panic!("Bad suit string."),
	}
}

fn parse_rank(rank: Option<Match>) -> Rank {
	let id = &rank.unwrap().as_str()[..1];

	match id {
		"2" => Rank::Two,
		"3" => Rank::Three,
		"4" => Rank::Four,
		"5" => Rank::Five,
		"6" => Rank::Six,
		"7" => Rank::Seven,
		"8" => Rank::Eight,
		"9" => Rank::Nine,
		"1" => Rank::Ten,
		"a" => Rank::Ace,
		"k" => Rank::King,
		"q" => Rank::Queen,
		"j" => Rank::Jack,
		_ => panic!("Bad rank string"),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn long() {
		assert_eq!(
			parse_card("Ace of Hearts".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Ace)
		);
		assert_eq!(
			parse_card("Ace of Heart".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Ace)
		);
		assert_eq!(
			parse_card("ace of Hearts".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Ace)
		);
		assert_eq!(
			parse_card("10 of Hearts".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Ten)
		);
		assert_eq!(
			parse_card("5 of Hearts".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Five)
		);
	}

	#[test]
	fn short() {
		assert_eq!(
			parse_card("ha".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Ace)
		);
		assert_eq!(
			parse_card("Ha".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Ace)
		);
		assert_eq!(
			parse_card("hA".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Ace)
		);
		assert_eq!(
			parse_card("h10".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Ten)
		);
		assert_eq!(
			parse_card("h5".into()).unwrap(),
			Card::new(Suit::Hearts, Rank::Five)
		);
	}
}
