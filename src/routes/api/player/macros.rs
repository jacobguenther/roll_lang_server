// File: src/routes/macros.rs

use rocket_contrib::json::Json;


use auth::authorization::*;

use crate::forms;
use crate::forms::LoginCookie;

use crate::db::DbConn;

use crate::models::{
	r#macro::*,
	r#macro::insertable_macro::*,
};

// pub struct MacrosWrapper {
// 	macros: Vec<r#Macro>,
// }

#[get("/api/player/macros/all")]
pub fn all(logged_in: AuthCont<LoginCookie>, connection: DbConn) -> Result<Json<Vec<Macro>>, String> {
	match Macro::get_all_for_player(logged_in.cookie.player_id, &connection) {
		Ok(player_macros) => Ok(Json(player_macros)),
		Err(_query_error) => return Err("Read Failed: Unkown Error".to_owned()),
	}
}
// #[get("/api/player/macros/get", data = "<get_macro>")]
// pub fn get(get_macro: Form<forms::MacroName>, logged_in: AuthCont<LoginCookie>, connection: DbConn) -> Result<Json<Macro>, String> {
// 	match Macro::get_from_player_id_and_name(logged_in.cookie.player_id, &get_macro.name, &connection) {
// 		Ok(r#macro) => Ok(Json(r#macro)),
// 		Err(_query_error) => return Err("Read Failed: Unkown Error".to_owned()),
// 	}
// }
#[post("/api/player/macros/update_create", data = "<new_macro>")]
pub fn update_create(new_macro: Json<forms::Macro>, logged_in: AuthCont<LoginCookie>, connection: DbConn) -> Result<String, String> {
	match Macro::id_from_player_id_and_name(logged_in.cookie.player_id, &new_macro.name, &connection) {
		Ok(_id) => update(new_macro, logged_in, connection),
		Err(_query_error) => new(new_macro, logged_in, connection),
	}
}
#[post("/api/player/macros/new", data = "<new_macro>")]
pub fn new(new_macro: Json<forms::Macro>, logged_in: AuthCont<LoginCookie>, connection: DbConn) -> Result<String, String> {
	let insertable_macro = InsertableMacro {
		player_id: logged_in.cookie.player_id,
		name: new_macro.name.clone(),
		source: new_macro.source.clone(),
		has_shortcut: new_macro.has_shortcut,
	};
	match Macro::insert(&insertable_macro, &connection) {
		Ok(_macro) => Ok("Create Success".to_owned()),
		Err(_query_error) => Err("Create Failed: Unknown Error".to_owned()),
	}
}
#[post("/api/player/macros/update", data = "<update_macro>")]
pub fn update(update_macro: Json<forms::Macro>, logged_in: AuthCont<LoginCookie>, connection: DbConn) -> Result<String, String> {
	let insertable_macro = InsertableMacro {
		player_id: logged_in.cookie.player_id,
		name: update_macro.name.clone(),
		source: update_macro.source.clone(),
		has_shortcut: update_macro.has_shortcut,
	};
	match Macro::update(&insertable_macro, &connection) {
		Ok(_macro) => Ok("Update Success".to_owned()),
		Err(_query_error) => Err("Update Failed: Unknown Error".to_owned()),
	}
}
#[post("/api/player/macros/update_source", data = "<update_macro>")]
pub fn update_source(update_macro: Json<forms::Macro>, logged_in: AuthCont<LoginCookie>, connection: DbConn) -> Result<String, String> {
	let insertable_macro = InsertableMacro::new(
		logged_in.cookie.player_id,
		&update_macro.name,
		&update_macro.source,
		update_macro.has_shortcut,
	);
	match Macro::update_source(&insertable_macro, &connection) {
		Ok(_macro) => Ok("Update Success".to_owned()),
		Err(_query_error) => Err(format!("Update Failed: {}", _query_error))// Err("Update Failed: Unknown Error".to_owned()),
	}
}
#[post("/api/player/macros/update_shortcut", data = "<update_macro>")]
pub fn update_shortcut(update_macro: Json<forms::Macro>, logged_in: AuthCont<LoginCookie>, connection: DbConn) -> Result<String, String> {
	let insertable_macro = InsertableMacro::new(
		logged_in.cookie.player_id,
		&update_macro.name,
		&update_macro.source,
		update_macro.has_shortcut,
	);
	match Macro::update_has_shortcut(&insertable_macro, &connection) {
		Ok(_macro) => Ok("Update Success".to_owned()),
		Err(_query_error) => Err(format!(
			"Update Failed: player_id: {} name: {}, error: {}",
			logged_in.cookie.player_id,
			update_macro.name,
			_query_error
		))
		// Err("Update Failed: Unknown Error".to_owned()),
	}
}
#[delete("/api/player/macros/delete", data = "<delete_macro>")]
pub fn delete(delete_macro: Json<forms::MacroName>, logged_in: AuthCont<LoginCookie>, connection: DbConn) -> Result<String, String> {
	let id = match Macro::id_from_player_id_and_name(logged_in.cookie.player_id, &delete_macro.name, &connection) {
		Ok(id) => id,
		Err(_query_error) => return Err(format!("Delete Failed: No macro named {} for player {}.",
			delete_macro.name,
			logged_in.cookie.name,
		)),
	};
	match Macro::delete(id, &connection) {
		Ok(_macro) => Ok(format!("Delete Success: Deleted macro {}", delete_macro.name)),
		Err(_query_error) => Err("Delete Failed: Unknown Error".to_owned()),
	}
}
