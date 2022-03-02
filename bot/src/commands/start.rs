use crate::{Context, Error, game::BasicResponse};

#[poise::command(slash_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
	let BasicResponse {
		ephemeral: eph,
		message,
	} = ctx.data()
		.games
		.lock()
		.unwrap()
		.entry(ctx.channel_id())
		.or_default()
		.start();

	ctx.send(|r| r.content(message).ephemeral(eph)).await?;
	Ok(())
}
