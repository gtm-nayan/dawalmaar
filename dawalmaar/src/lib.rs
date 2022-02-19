pub mod cards;
mod deck;
pub mod game;
pub mod enums;
mod player;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
