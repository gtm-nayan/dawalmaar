#[derive(PartialEq, Eq, Debug)]
pub enum PlayCardError {
	CardNotInHand,
	CantPlaySuit,
}