mod commands;
mod game;
mod setup;
mod utils;

use dashmap::DashMap;
use setup::{get_options, get_token};

use game::Game;
use poise::{serenity_prelude::ChannelId, Framework};
pub struct Data {
	games: DashMap<ChannelId, Game>,
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
					games: DashMap::new(),
				})
			})
		})
		.options(get_options())
		.run()
		.await
		.unwrap();
}
