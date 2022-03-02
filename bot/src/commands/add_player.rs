use crate::{game::BasicResponse, Context, Error};

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
	let BasicResponse {
		ephemeral: eph,
		message,
	} = ctx.data()
		.games
		.lock()
		.unwrap()
		.entry(ctx.channel_id())
		.or_default()
		.add_player();

	ctx.send(|r| r.content(message).ephemeral(eph)).await?;
	Ok(())
}
