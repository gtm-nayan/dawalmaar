mod commands;
mod game;
mod setup;

use setup::{get_options, get_token};

use game::Game;
use poise::{serenity_prelude::ChannelId, Framework};
use std::{collections::HashMap, sync::Mutex};
pub struct Data {
	games: Mutex<HashMap<ChannelId, Game>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
	Framework::<Data, Error>::build()
		.token(get_token())
		.user_data_setup(move |_, _, _| {
			Box::pin(async move {
				Ok(Data {
					games: Mutex::new(HashMap::new()),
				})
			})
		})
		.options(get_options())
		.run()
		.await
		.unwrap();
}
