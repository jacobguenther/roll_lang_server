// File: src/models/player/mod.rs

pub mod insertable_player;
use insertable_player::*;

use serde::{Deserialize, Serialize};
use crate::schema::players;
use diesel::sql_types::*;

use diesel;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Queryable, QueryableByName, AsChangeset, Serialize, Deserialize)]
#[table_name = "players"]
pub struct Player {
	#[sql_type="Integer"]
	pub id: i32,
	#[sql_type="Text"]
	pub name: String,
	#[sql_type="Text"]
	pub email: String,
	#[sql_type="Text"]
	pub password: String,
	#[sql_type="Text"]
	pub hash_salt: String,
	#[sql_type="Bool"]
	pub is_admin: bool,
}

impl Player {
	pub fn new(id: i32) -> Player {
		Player {
			id,
			name: "".to_owned(),
			email: "".to_owned(),
			password: "".to_owned(),
			hash_salt: "".to_owned(),
			is_admin: false,
		}
	}

	pub fn all(connection: &PgConnection) -> QueryResult<Vec<Player>> {
		players::table.load::<Player>(&*connection)
	}

    pub fn id_from_name(name: &str, connection: &PgConnection) -> QueryResult<i32> {
        players::table
            .filter(players::name.eq(name))
            .select(players::id)
            .get_result(connection)
	}
    pub fn id_from_email(email: &str, connection: &PgConnection) -> QueryResult<i32> {
        players::table
            .filter(players::email.eq(email))
            .select(players::id)
            .get_result(connection)
	}

	pub fn name_from_id(id: i32, connection: &PgConnection) -> QueryResult<String> {
		players::table.find(id)
			.select(players::name)
			.get_result(connection)
	}
	pub fn email_from_id(id: i32, connection: &PgConnection) -> QueryResult<String> {
		players::table.find(id)
			.select(players::email)
			.get_result(connection)
	}
	pub fn password_from_id(id: i32, connection: &PgConnection) -> QueryResult<String> {
		players::table.find(id)
			.select(players::password)
			.get_result(connection)
	}

	pub fn update_name(id: i32, new_name: &str, connection: &PgConnection) -> QueryResult<Player> {
		diesel::update(players::table.find(id))
			.set(players::name.eq(new_name))
			.get_result(connection)
	}
	pub fn update_email(id: i32, new_email: &str, connection: &PgConnection) -> QueryResult<Player> {
		diesel::update(players::table.find(id))
			.set(players::email.eq(new_email))
			.get_result(connection)
	}
	pub fn update_password(id: i32, new_password: &str, connection: &PgConnection) -> QueryResult<Player> {
		diesel::update(players::table.find(id))
			.set(players::password.eq(new_password))
			.get_result(connection)
	}

	pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Player> {
		players::table
			.find(id)
			.get_result::<Player>(connection)
	}
	pub fn insert(insertable_player: &InsertablePlayer, connection: &PgConnection) -> QueryResult<Player> {
		diesel::insert_into(players::table)
			.values(insertable_player)
			.get_result(connection)
	}
	pub fn update(id: i32, player: Player, connection: &PgConnection) -> QueryResult<Player> {
		diesel::update(players::table.find(id))
			.set(&player)
			.get_result(connection)
	}
	pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
		diesel::delete(players::table.find(id))
			.execute(connection)
    }
}