use crate::{utils::no_game_in_this_channel, Context, Error};

#[poise::command(slash_command)]
pub async fn play(
	ctx: Context<'_>,
	#[description = "What card do you want to play?"] card: String,
) -> Result<(), Error> {
	let res = {
		let games = &ctx.data().games;
		let game = games.get_mut(&ctx.channel_id());

		if let Some(mut game) = game {
			let temp = game.play_card(ctx.author().id, card);
			drop(game);
			if temp.0 { // Get rid of the game if it's over
				games.remove(&ctx.channel_id());
			};
			temp.1
		} else {
			no_game_in_this_channel()
		}
	};

	ctx.send(|r| r.content(res.message).ephemeral(res.ephemeral))
		.await?;
	Ok(())
}
