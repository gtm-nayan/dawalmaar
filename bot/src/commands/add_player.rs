use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
	let (message, eph) = ctx
		.data()
		.games // Mutex<HashMap<ChannelId, Game>>
		.lock()
		.unwrap()
		.entry(ctx.channel_id())
		.or_default()
		.add_player();

	ctx.send(|r| r.content(message).ephemeral(eph)).await?;
	Ok(())
}
