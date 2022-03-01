mod commands;
mod game;

use commands::{join, ping, register};
use dotenv::dotenv;
use game::Game;
use poise::{
	serenity_prelude::{ChannelId, UserId},
	Framework, FrameworkOptions, PrefixFrameworkOptions,
};
use std::{
	collections::{HashMap, HashSet},
	env,
	sync::Mutex,
};
pub struct Data {
	games: Mutex<HashMap<ChannelId, Game>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
	dotenv().ok();

	let prefix_options = PrefixFrameworkOptions {
		prefix: Some("cap".into()),
		additional_prefixes: vec![],
		dynamic_prefix: None,
		stripped_dynamic_prefix: None,
		mention_as_prefix: true,
		case_insensitive_commands: false,
		edit_tracker: None,
		execute_self_messages: false,
		execute_untracked_edits: false,
		ignore_edits_if_not_yet_responded: true,
	};

	let framework_options = FrameworkOptions {
		commands: vec![ping(), register(), join()],
		prefix_options,
		owners: {
			let mut owners = HashSet::new();
			owners.insert(UserId(339731096793251854_u64));
			owners
		},

		..Default::default()
	};

	Framework::<Data, Error>::build()
		.token(get_token())
		.user_data_setup(move |_, _, _| {
			Box::pin(async move {
				Ok(Data {
					games: Mutex::new(HashMap::new()),
				})
			})
		})
		.options(framework_options)
		.run()
		.await
		.unwrap();
}

fn get_token() -> String {
	env::var("DISCORD_TOKEN").expect("Token not provided.")
}


