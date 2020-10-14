// File: src/models/macros/insertable_macro.rs

use super::Macro;
use crate::schema::macros;

#[derive(Insertable)]
#[table_name = "macros"]
pub struct InsertableMacro {
	pub player_id: i32,
	pub name: String,
	pub source: String,
	pub has_shortcut: bool,
}
impl InsertableMacro {
	pub fn new(player_id: i32, name: &str, source: &str, has_shortcut: bool) -> InsertableMacro {
		InsertableMacro {
			player_id,
			name: name.to_owned(),
			source: source.to_owned(),
			has_shortcut
		}
	}
}
impl From<Macro> for InsertableMacro {
	fn from(r#macro: Macro) -> InsertableMacro {
		InsertableMacro {
			player_id: r#macro.player_id,
			name: r#macro.name,
			source: r#macro.source,
			has_shortcut: r#macro.has_shortcut,
		}
	}
}