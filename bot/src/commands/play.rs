use crate::{Context, Error, utils::no_game_in_this_channel};

#[poise::command(slash_command)]
pub async fn play(
	ctx: Context<'_>,
	#[description = "What card do you want to play?"] card: String,
) -> Result<(), Error> {
	let res = {
		let game = ctx.data().games.get_mut(&ctx.channel_id());

		if let Some(mut game) = game {
			game.play_card(ctx.author().id, card)
		} else {
			no_game_in_this_channel()
		}
	};

	ctx.send(|r| r.content(res.message).ephemeral(res.ephemeral))
		.await?;
	Ok(())
}
