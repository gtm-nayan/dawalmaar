use crate::{Context, Error, utils::no_game_in_this_channel};

#[poise::command(slash_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
	let res = {
		let game = ctx.data().games.get_mut(&ctx.channel_id());

		if let Some(mut game) = game {
			game.start()
		} else {
			no_game_in_this_channel()
		}
	};
	ctx.send(|r| r.content(res.message).ephemeral(res.ephemeral))
		.await?;
	Ok(())
}
