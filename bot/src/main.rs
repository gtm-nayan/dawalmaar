use dotenv::dotenv;
use std::env;

fn main() {
	dotenv().ok();
	println!("{}", get_token());
}

fn get_token() -> String {
	env::var("DISCORD_TOKEN").expect("Login token was not provided.")
}
