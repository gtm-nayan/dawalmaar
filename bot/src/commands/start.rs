use crate::{utils::no_game_in_this_channel, Context, Error};

#[poise::command(slash_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
	//! Start a game

	ctx.send(|r| {
		if let Some(mut game) = ctx.data().games.get_mut(&ctx.channel_id()) {
			game.start(r)
		} else {
			no_game_in_this_channel(r)
		}
		r
	})
	.await?;
	Ok(())
}
