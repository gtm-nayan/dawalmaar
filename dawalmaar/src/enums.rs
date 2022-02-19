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

#[derive(Debug, PartialEq, Eq)]
pub enum TrickEndResult {
	NextTrick(usize),
	GameOver,
}