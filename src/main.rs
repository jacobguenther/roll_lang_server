
// File src/main.rs

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket::State;
use rocket::response::{Redirect};
use rocket::request::{Form};

use rocket_contrib::{
	templates::Template,
	serve::StaticFiles,
};

use serde::{Deserialize, Serialize};

use tera::Context;

#[get("/")]
fn index() -> Template {
	let context: HashMap<String, String> = HashMap::new();
	Template::render("index", context)
}

fn main() {
	rocket::ignite()
		.attach(Template::fairing())
		.mount("/",
			routes![
				index,
			])
		.mount("/", StaticFiles::from("www/static/"))
		// .manage(incidents_json)
		// .register(catchers![not_found])
		.launch();
}
