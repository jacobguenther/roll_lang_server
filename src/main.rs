
// File src/main.rs

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io; // Result
use std::collections::HashMap;

// use rocket::State;
use rocket::response::{
	NamedFile,
	// Redirect,
};
// use rocket::request::{Form};

use rocket_contrib::{
	templates::Template,
	serve::StaticFiles,
};

// use serde::{Deserialize, Serialize};

// use tera::Context;

#[get("/")]
fn index() -> Template {
	let context: HashMap<String, String> = HashMap::new();
	Template::render("index", context)
}
#[get("/help")]
fn help() ->  io::Result<NamedFile> {
	NamedFile::open("www/static/html/help.html")
}
#[get("/about")]
fn about() -> io::Result<NamedFile> {
	NamedFile::open("www/static/html/about.html")
}
#[get("/about/javascript")]
fn licenses() -> io::Result<NamedFile> {
	NamedFile::open("www/static/html/javascript.html")
}

fn main() {
	rocket::ignite()
		.attach(Template::fairing())
		.mount("/",
			routes![
				index,
				help,
				about,
				licenses,
			])
		.mount("/", StaticFiles::from("www/static/"))
		// .manage(state)
		// .register(catchers![not_found])
		.launch();
}
