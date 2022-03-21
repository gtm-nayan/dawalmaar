use crate::{utils::no_game_in_this_channel, Context, Error};

#[poise::command(slash_command)]
pub async fn hand(ctx: Context<'_>) -> Result<(), Error> {
	//! Get your hand

	ctx.send(|r| {
		let game = ctx.data().games.get_mut(&ctx.channel_id());

		if let Some(mut game) = game {
			game.get_hand(ctx.author().id, r)
		} else {
			no_game_in_this_channel(r)
		};
		r
	})
	.await?;

	Ok(())
}
