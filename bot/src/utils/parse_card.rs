use dawalmaar::cards::{Card, Rank, Suit};
use lazy_static::lazy_static;
use regex::{Match, Regex};

lazy_static! {
	static ref CARD_REGEX: Regex = Regex::new(r#"(?P<long>^(?P<rank>Ace|King|Queen|Jack|(?:[AKQJ2-9]|10))\s+of\s+(?P<suit>Spade|Club|Diamond|Heart)s?)$|(?P<short>^(?P<suit2>S|C|D|H)(?P<rank2>[AKQJ2-9]|10))$"#).unwrap();
}

pub struct ParseCardError;

pub fn parse_card(card: &str) -> Result<Card, ParseCardError> {
	let matches = CARD_REGEX.captures(card).ok_or(ParseCardError)?;

	let (rank, suit) = if matches.name("long").is_some() {
		(matches.name("rank"), matches.name("suit"))
	} else {
		(matches.name("rank2"), matches.name("suit2"))
	};

	Ok(Card::new(parse_suit(suit), parse_rank(rank)))
}

fn parse_suit(suit: Option<Match>) -> Suit {
	let id = &suit.unwrap().as_str().to_ascii_lowercase()[..1];

	match id {
		"h" => Suit::Hearts,
		"d" => Suit::Diamonds,
		"c" => Suit::Clubs,
		"s" => Suit::Spades,
		_ => panic!("Bad suit string."),
	}
}

fn parse_rank(rank: Option<Match>) -> Rank {
	let id = &rank.unwrap().as_str().to_ascii_lowercase()[..1];

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
