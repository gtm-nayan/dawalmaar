use crate::game::BasicResponse;

pub fn no_game_in_this_channel() -> BasicResponse {
	BasicResponse {
		message: "There's no game in this channel. Do `/join` to create one.".into(),
		ephemeral: true,
	}
}
