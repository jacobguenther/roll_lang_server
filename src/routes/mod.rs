// File: src/routes/mod.rs

pub mod api;

use std::fs;

use rocket::{
	http::RawStr,
	response::{
		Redirect,
		Flash,
	},
	request::FlashMessage,
};

use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;

use auth::authorization::*;

use crate::forms::LoginCookie;
use crate::contexts;
use crate::db::DbConn;
use crate::models::player::*;

#[get("/")]
pub fn index(logged_in_opt: Option<AuthCont<LoginCookie>>, flash: Option<FlashMessage>) -> Template {
	let mut context = contexts::Index::default();
	if let Some(flash_message) = flash {
		if flash_message.name() == "login_success" {
			context.login_success = true;
		} else if flash_message.name() == "logout_success" {
			context.base.logout_success = true;
		} else if flash_message.name() == "deleted_account" {
			context.deleted_account = true;
		}
	};
	if let Some(logged_in) = logged_in_opt {
		context.base.logged_in = true;
		context.base.display_name = logged_in.cookie.name;
	}

	Template::render("index", context)
}
#[get("/help")]
pub fn help(logged_in_opt: Option<AuthCont<LoginCookie>>) -> Template {
	let mut context = contexts::Base::default();
	if let Some(logged_in) = logged_in_opt {
		context.logged_in = true;
		context.display_name = logged_in.cookie.name;
	}
	Template::render("help", context)
}
#[get("/about")]
pub fn about(logged_in_opt: Option<AuthCont<LoginCookie>>) -> Template {
	let mut context = contexts::Base::default();
	if let Some(logged_in) = logged_in_opt {
		context.logged_in = true;
		context.display_name = logged_in.cookie.name;
	}
	Template::render("about", context)
}
#[get("/about/javascript")]
pub fn licenses(logged_in_opt: Option<AuthCont<LoginCookie>>) -> Template {
	let mut context = contexts::Base::default();
	if let Some(logged_in) = logged_in_opt {
		context.logged_in = true;
		context.display_name = logged_in.cookie.name;
	}
	Template::render("javascript", context)
}
#[get("/assets/shaders/<name>")]
pub fn get_shader(name: String) -> Result<Json<String>, String> {
	match fs::read_to_string(&format!("/assets/shaders/{}", name)) {
		Ok(source) => Ok(Json(source)),
		Err(_e) => Err(format!("Shader {} does not exist!", name)),
	}
}

#[get("/create_account")]
pub fn create_account(logged_in_opt: Option<AuthCont<LoginCookie>>, flash: Option<FlashMessage>) -> Result<Template, Flash<Redirect>> {
	get_create_account("", "", logged_in_opt, flash)
}
#[get("/create_account?<name>&<email>")]
pub fn retry_create_account(name: &RawStr, email: &RawStr, logged_in_opt: Option<AuthCont<LoginCookie>>, flash: Option<FlashMessage>) -> Result<Template, Flash<Redirect>> {
	get_create_account(name.as_str(), email.as_str(), logged_in_opt, flash)
}
pub fn get_create_account(name: &str, email: &str, logged_in_opt: Option<AuthCont<LoginCookie>>, flash: Option<FlashMessage>) -> Result<Template, Flash<Redirect>> {
	if let Some(_) = logged_in_opt {
		return Err(Flash::new(Redirect::to("/"), "cant_creat_account_while_logged_in", ""));
	}
	let mut context = contexts::CreateAccount::new(name, email);
	if let Some(flash_message) = flash {
		if flash_message.name() == "password_too_short" {
			context.password_too_short = true;
		} else if flash_message.name() == "passwords_dont_match" {
			context.passwords_dont_match = true;
		} else if flash_message.name() == "name_or_email_taken" {
			context.name_or_email_taken = true;
		} else if flash_message.name() == "unknown_error" {
			context.unknown_error = true;
		}
	};

	Ok(Template::render("create_account", context))
}


#[get("/login", rank=1)]
pub fn logged_in(_logged_in: AuthCont<LoginCookie>) -> Redirect {
	Redirect::to("/")
}
#[get("/login", rank=3)]
pub fn login(flash: Option<FlashMessage>) -> Template {
	get_login("", flash)
}
#[get("/login?<name>", rank=2)]
pub fn retry_login(name: &RawStr, flash: Option<FlashMessage>) -> Template {
	get_login(name.as_str(), flash)
}
fn get_login(name: &str, flash: Option<FlashMessage>) -> Template {
	let mut context = contexts::Login::new(name);
	if let Some(flash_message) = flash {
		if flash_message.name() == "login_fail" {
			context.login_fail = true;
		}
	};
	Template::render("login", context)
}

#[get("/account", rank=1)]
pub fn account(logged_in: AuthCont<LoginCookie>, flash: Option<FlashMessage>, connection: DbConn) -> Template {
	let email = match Player::email_from_id(logged_in.cookie.player_id, &connection) {
		Ok(email) => email,
		Err(_query_error) => String::new(),
	};
	let mut context = contexts::Account::new(&logged_in.cookie.name, &email);
	context.base.logged_in = true;
	context.base.display_name = logged_in.cookie.name;

	if let Some(flash_message) = flash {
		if flash_message.name() == "update_name_wrong_password" {
			context.update_name_wrong_password = true;
		} else if flash_message.name() == "update_name_taken" {
			context.update_name_taken = true;
		} else if flash_message.name() == "update_email_wrong_password" {
			context.update_email_wrong_password = true;
		} else if flash_message.name() == "update_email_taken" {
			context.update_email_taken = true;
		} else if flash_message.name() == "update_password_wrong_password" {
			context.update_password_wrong_password = true;
		} else if flash_message.name() == "update_password_dont_match" {
			context.update_password_dont_match = true;
		} else if flash_message.name() == "update_password_too_short" {
			context.update_password_too_short = true;
		} else if flash_message.name() == "update_password_too_long" {
			context.update_password_too_long = true;
		} else if flash_message.name() == "update_password_missing_characters" {
			context.update_password_missing_characters = true;
		} else if flash_message.name() == "delete_account_wrong_password" {
			context.delete_account_wrong_password = true;
		} else if flash_message.name() == "changed_name" {
			context.changed_name = true;
		} else if flash_message.name() == "changed_email" {
			context.changed_email = true;
		} else if flash_message.name() == "changed_password" {
			context.changed_password = true;
		} else if flash_message.name() == "unknown_error" {
			context.unknown_error = true;
		}
	};

	Template::render("account", context)
}

#[get("/account", rank=2)]
pub fn account_redirect() -> Redirect {
	Redirect::to("/login")
}