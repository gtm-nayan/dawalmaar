use crate::{game::BasicResponse, Context, Error};

#[poise::command(slash_command)]
pub async fn hand(ctx: Context<'_>) -> Result<(), Error> {
	let res = {
		let game = ctx.data().games.get_mut(&ctx.channel_id());

		if let Some(mut game) = game {
			game.get_hand(ctx.author().id)
		} else {
			BasicResponse {
				message: "There's no game in this channel. Do /join to create one.".into(),
				ephemeral: true,
			}
		}
	};

	ctx.send(|r| r.content(res.message).ephemeral(res.ephemeral))
		.await?;

	Ok(())
}
