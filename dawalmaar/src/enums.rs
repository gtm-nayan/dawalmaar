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
pub enum TurnEndResult {
	NextTurn(usize),
	GameOver([Team; 2]),
}
