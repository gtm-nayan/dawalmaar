use poise::CreateReply;

pub fn no_game_in_this_channel(res: &mut CreateReply) {
	res.content("There is no game in this channel")
		.ephemeral(true);
}
