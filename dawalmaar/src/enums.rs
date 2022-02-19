use crate::teams::Team;

pub enum StartError {
	GameAlreadyStarted,
	GameNotFull,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayCardError {
	GameNotStarted,
	NotThisPlayersTurn,
	CardNotInHand,
	CantPlaySuit,
}

#[derive(Debug)]
pub enum TrickEndResult {
	NextTrick(usize),
	GameOver([Team; 2]),
}
