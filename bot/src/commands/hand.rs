use crate::{utils::no_game_in_this_channel, Context, Error};

#[poise::command(slash_command)]
pub async fn hand(ctx: Context<'_>) -> Result<(), Error> {
	let res = {
		let game = ctx.data().games.get_mut(&ctx.channel_id());

		if let Some(mut game) = game {
			game.get_hand(ctx.author().id)
		} else {
			no_game_in_this_channel()
		}
	};

	ctx.send(|r| r.content(res.message).ephemeral(res.ephemeral))
		.await?;

	Ok(())
}
