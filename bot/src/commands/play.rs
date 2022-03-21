use crate::{utils::no_game_in_this_channel, Context, Error};

#[poise::command(slash_command)]
pub async fn play(
	ctx: Context<'_>,
	#[description = "What card do you want to play?"] card: String,
) -> Result<(), Error> {
	//! Play a card

	ctx.send(|reply| {
		let games = &ctx.data().games;
		let game = games.get_mut(&ctx.channel_id());

		if let Some(mut game) = game {
			let should_end = game.play_card(ctx.author().id, card, reply);
			drop(game);
			if should_end {
				games.remove(&ctx.channel_id());
			};
		} else {
			no_game_in_this_channel(reply)
		};
		reply
	})
	.await?;
	Ok(())
}
