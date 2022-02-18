use crate::{cards::Suit, deck::Deck, game_errors::StartError, player::Player};

pub struct Game {
    players: Vec<Player>,
    started: bool,
    suit_in_play: Option<Suit>,
    trump_suit: Option<Suit>,
    turn: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            started: false,
            suit_in_play: None,
            trump_suit: None,
            turn: 0,
        }
    }

    pub fn add_player(&mut self) -> Result<usize, ()> {
        if self.is_full() {
            return Err(());
        }
        self.players.push(Player::new());
        Ok(self.players.len() - 1)
    }

    pub fn is_full(&self) -> bool {
        self.players.len() == 4
    }

    pub fn start(&mut self) -> Result<(), StartError> {
        if !self.is_full() {
            return Err(StartError::GameNotFull);
        }
        if self.started {
            return Err(StartError::GameAlreadyStarted);
        }
        self.started = true;
        self.deal_cards();
        Ok(())
    }

	pub fn has_started(&self) -> bool {
		self.started
	}

    pub fn deal_cards(&mut self) {
        let mut deck = Deck::new();
        deck.shuffle();

        for player in self.players.iter_mut() {
            for _ in 0..13 {
                player.add_card(deck.deal_card().unwrap());
            }
            player.sort_hand();
        }
    }

    pub fn is_over(&self) -> bool {
        self.players.iter().all(|player| player.is_empty())
    }
}
