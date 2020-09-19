// File: src/models/player/insertable_player.rs

use super::Player;
use crate::schema::players;
use crate::forms;

#[derive(Insertable)]
#[table_name = "players"]
pub struct InsertablePlayer {
	pub name: String,
	pub email: String,
	pub password: String,
	pub hash_salt: String,
	pub is_admin: bool,
}
impl InsertablePlayer {
    pub fn from_player(player: Player) -> InsertablePlayer {
        InsertablePlayer {
			name: player.name,
			email: player.email,
			password: player.password,
			hash_salt: player.hash_salt,
			is_admin: player.is_admin,
        }
    }
}
impl From<forms::CreateAccount> for InsertablePlayer {
	fn from(form: forms::CreateAccount) -> InsertablePlayer {
		InsertablePlayer {
			name: form.player_name,
			email: form.email,
			password: form.password,
			hash_salt: "".to_owned(),
			is_admin: false,
		}
	}
}