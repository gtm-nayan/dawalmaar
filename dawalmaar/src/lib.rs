pub mod cards;
mod deck;
mod player;
pub mod game;
pub mod game_errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
