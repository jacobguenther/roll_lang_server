
// File src/main.rs

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
extern crate rocket_auth_login as auth;

use rocket_contrib::{
	templates::Template,
	serve::StaticFiles,
};

mod db;
mod models;
mod schema;
mod forms;
mod routes;
mod contexts;

use db::init_pool;
use routes::{
	*,
};

fn main() {
	rocket::ignite()
		.attach(Template::fairing())
		.mount("/",
			routes![
				index,
				help,
				about,
				licenses,

				logged_in,
				login,
				retry_login,

				create_account,
				retry_create_account,

				account,
				account_redirect,

				// get_shader,

				api::player::is_logged_in,
				api::player::create,
				api::player::delete,
				api::player::login,
				api::player::logout,

				api::player::settings::update_name,
				api::player::settings::update_email,
				api::player::settings::update_password,

				api::player::macros::all,
				// api::player::macros::get,
				api::player::macros::update_create,
				api::player::macros::new,
				api::player::macros::update,
				api::player::macros::update_source,
				api::player::macros::update_shortcut,
				api::player::macros::delete,
			])
		.mount("/", StaticFiles::from("www/static/"))
		.manage(init_pool())
		// .register(catchers![not_found])
		.launch();
}
