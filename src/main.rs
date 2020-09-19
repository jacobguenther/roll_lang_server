
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
use routes::*;

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
				logout,
				process_login,

				create_account,
				retry_create_account,
				create_account_form,
			])
		.mount("/", StaticFiles::from("www/static/"))
		.manage(init_pool())
		// .register(catchers![not_found])
		.launch();
}
