// File: src/routes/mod.rs

use rocket::{
	// State,
	http::{
		Cookie,
		Cookies,
		RawStr,
	},
	response::{
		Redirect,
		Flash,
	},
	request::{
		FlashMessage,
		Form,
	},
};

use rocket_contrib::{
	templates::Template,
	// serve::StaticFiles,
};

use auth::authorization::*;

use crate::forms;
use crate::forms::LoginCookie;

use crate::db::DbConn;

use crate::contexts;

use crate::models::player::*;

#[get("/")]
pub fn index(logged_in_opt: Option<AuthCont<LoginCookie>>, flash: Option<FlashMessage>) -> Template {
	let mut context = contexts::Index::default();
	if let Some(flash_message) = flash {
		if flash_message.name().contains("login_success") {
			context.login_success = true;
		} else if flash_message.name().contains("logout_success") {
			context.base.logout_success = true;
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
		if flash_message.name().contains("password_too_short") {
			context.password_too_short = true;
		} else if flash_message.name().contains("passwords_dont_match") {
			context.passwords_dont_match = true;
		} else if flash_message.name().contains("name_or_email_taken") {
			context.name_or_email_taken = true;
		} else if flash_message.name().contains("unknown_error") {
			context.unknown_error = true;
		}
	};

	Ok(Template::render("create_account", context))
}
#[post("/create_account", data = "<create_form>")]
pub fn create_account_form(create_form: Form<forms::CreateAccount>, connection: DbConn) -> Flash<Redirect> {
	let form = create_form.clone();
	if Player::id_from_name(&form.player_name, &connection).is_ok() ||
		Player::id_from_email(&form.email, &connection).is_ok()
	{
		return Flash::new(Redirect::to("/create_account"), "name_or_email_taken", "");
	}

	let password_problem_redirect_url = format!("/create_account?name={}&email={}", form.player_name, form.email);
	if form.password.len() < 8 {
		return Flash::new(Redirect::to(password_problem_redirect_url), "password_too_short", "");
	} else if form.password != form.confirm_password {
		return Flash::new(Redirect::to(password_problem_redirect_url), "passwords_dont_match", "");
	}
	let player = Player::insert(form.into(), &connection);
	match player {
		Ok(_) => Flash::new(Redirect::to("/login"), "create_account_success", ""),
		Err(_) => Flash::new(Redirect::to("/create_account"), "unkown_error", "")
	}
}

#[get("/login", rank=1)]
pub fn logged_in(_player: AuthCont<LoginCookie>) -> Redirect {
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
		if flash_message.name().contains("login_fail") {
			context.login_fail = true;
		}
	};
	Template::render("login", context)
}
#[post("/login", data = "<login_form>")]
pub fn process_login(login_form: Form<forms::Login>, mut cookies: Cookies) -> Flash<Redirect> {
	let login = login_form.clone();
	match login.authenticate() {
		Ok(cookie) => {
			let cookie_id = forms::Login::cookie_id();
			let contents = cookie.store_cookie();
			cookies.add_private(Cookie::new(cookie_id, contents));
			Flash::new(Redirect::to("/"), "login_success", "")
		},
		Err(_) => {
			let err_redirect_url = format!("/login?name={}", login.name);
			Flash::new(Redirect::to(err_redirect_url), "login_fail", "")
		}
	}
}
#[post("/logout")]
pub fn logout(player: Option<LoginCookie>, mut cookies: Cookies) -> Flash<Redirect> {
	if let Some(_) = player {
		cookies.remove_private(Cookie::named(forms::LoginCookie::cookie_id()));
	}
	Flash::new(Redirect::to("/"), "logout_success", "")
}
