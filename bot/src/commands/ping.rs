use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error>{
	ctx.say("pong").await?;
	Ok(())
}