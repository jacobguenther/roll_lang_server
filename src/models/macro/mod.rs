// File: src/models/macros/mod.rs

// File: src/models/player/mod.rs

pub mod insertable_macro;
use insertable_macro::*;

use serde::{Deserialize, Serialize};
use crate::schema::macros;
use diesel::sql_types::*;

use diesel;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Queryable, QueryableByName, AsChangeset, Serialize, Deserialize)]
#[table_name = "macros"]
pub struct r#Macro {
	#[sql_type="Integer"]
	pub id: i32,
	#[sql_type="Integer"]
	pub player_id: i32,
	#[sql_type="Text"]
	pub name: String,
	#[sql_type="Text"]
	pub source: String,
	#[sql_type="Bool"]
	pub has_shortcut: bool,
}

impl r#Macro {
	pub fn new(id: i32, player_id: i32) -> Self {
		Self {
			id,
			player_id,
			name: "".to_owned(),
			source: "".to_owned(),
			has_shortcut: false,
		}
	}

    pub fn get_all_for_player(player_id: i32, connection: &PgConnection) -> QueryResult<Vec<r#Macro>> {
		macros::table
			.filter(macros::player_id.eq(player_id))
			.load(connection)
	}
	pub fn id_from_player_id_and_name(player_id: i32, name: &str, connection: &PgConnection) -> QueryResult<i32> {
		macros::table
			.filter(macros::player_id.eq(player_id))
			.filter(macros::name.eq(name))
			.select(macros::id)
			.get_result(connection)
	}
	pub fn get_from_player_id_and_name(player_id: i32, name: &str, connection: &PgConnection) -> QueryResult<r#Macro> {
		macros::table
			.filter(macros::player_id.eq(player_id))
			.filter(macros::name.eq(name))
			.get_result(connection)
	}
	pub fn get(id: i32, connection: &PgConnection) -> QueryResult<r#Macro> {
		macros::table
			.find(id)
			.get_result::<r#Macro>(connection)
	}

	pub fn insert(insertable_macro: &InsertableMacro, connection: &PgConnection) -> QueryResult<r#Macro> {
		diesel::insert_into(macros::table)
			.values(insertable_macro)
			.get_result(connection)
	}
	pub fn update(insertable_macro: &InsertableMacro, connection: &PgConnection) -> QueryResult<r#Macro> {
		diesel::update(macros::table)
			.filter(macros::player_id.eq(insertable_macro.player_id))
			.filter(macros::name.eq(&insertable_macro.name))
			.set((
				macros::source.eq(&insertable_macro.source),
				macros::has_shortcut.eq(insertable_macro.has_shortcut),
			))
			.get_result(connection)
	}
	pub fn update_source(insertable_macro: &InsertableMacro, connection: &PgConnection) -> QueryResult<r#Macro> {
		diesel::update(macros::table)
			.filter(macros::player_id.eq(insertable_macro.player_id))
			.filter(macros::name.eq(&insertable_macro.name))
			.set(macros::source.eq(&insertable_macro.source))
			.get_result(connection)
	}
	pub fn update_has_shortcut(insertable_macro: &InsertableMacro, connection: &PgConnection) -> QueryResult<r#Macro> {
		diesel::update(macros::table)
			.filter(macros::player_id.eq(insertable_macro.player_id))
			.filter(macros::name.eq(&insertable_macro.name))
			.set(macros::has_shortcut.eq(insertable_macro.has_shortcut))
			.get_result(connection)
	}
	pub fn delete(id: i32, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
		diesel::delete(macros::table.find(id))
			.execute(connection)
	}
	pub fn delete_with_player_id_and_name(player_id: i32, name: &str, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
		diesel::delete(macros::table)
			.filter(macros::player_id.eq(player_id))
			.filter(macros::name.eq(name))
			.execute(connection)
	}
}