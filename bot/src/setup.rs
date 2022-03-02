use std::collections::HashSet;

use poise::{serenity_prelude::UserId, FrameworkOptions, PrefixFrameworkOptions};

use crate::{
	commands::{hand, join, ping, play, register_commands, start},
	Data, Error,
};

pub fn get_options() -> FrameworkOptions<Data, Error> {
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
		commands: vec![ping(), register_commands(), join(), start(), hand(), play()],
		prefix_options,
		owners: {
			let mut owners = HashSet::new();
			owners.insert(UserId(339731096793251854_u64));
			owners
		},

		..Default::default()
	};

	framework_options
}

pub fn get_token() -> String {
	dotenv::dotenv().ok();
	std::env::var("DISCORD_TOKEN").expect("Token not provided.")
}
