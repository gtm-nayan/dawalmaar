use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
	//! Join a game or create a new one

	ctx.send(|r| {
		ctx.data()
			.games
			.entry(ctx.channel_id())
			.or_default()
			.add_player(ctx.author().id, r);
		r
	})
	.await?;
	Ok(())
}
