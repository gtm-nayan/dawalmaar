use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
	let res = ctx
		.data()
		.games
		.entry(ctx.channel_id())
		.or_default()
		.add_player(ctx.author().id);

	ctx.send(|r| r.content(res.message).ephemeral(res.ephemeral))
		.await?;
	Ok(())
}
