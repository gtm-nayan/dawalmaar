mod game;

use dotenv::dotenv;
use poise::{
	serenity_prelude::{CreateAllowedMentions, UserId},
	FrameworkOptions, PrefixFrameworkOptions,
};
use std::{
	collections::{HashMap, HashSet},
	env,
	sync::Mutex,
};

pub struct Data {
	games: Mutex<HashMap<String, u32>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
	dotenv().ok();

	let prefix_options: PrefixFrameworkOptions<Data, Error> = PrefixFrameworkOptions {
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

	let framework_options: FrameworkOptions<Data, Error> = FrameworkOptions {
		
		pre_command: |ctx| {
			Box::pin(async move {
				println!("{:?}", ctx.data().games.lock());
			})
		},
		..Default::default()
	};
}

fn get_token() -> String {
	env::var("DISCORD_TOKEN").expect("Token not provided.")
}
