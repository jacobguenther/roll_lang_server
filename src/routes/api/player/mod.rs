// File: src/routes/api/player/mod.rs

pub mod macros;
pub mod settings;


use rocket::{
	http::{
		Cookie,
		Cookies,
	},
	response::{
		Redirect,
		Flash,
	},
	request::{
		Form,
	},
};

use auth::authorization::*;

use crate::forms;
use crate::forms::LoginCookie;
use crate::db::DbConn;
use crate::models::player::*;

#[post("/api/player/create", data = "<form>")]
pub fn create(form: Form<forms::CreateAccount>, connection: DbConn) -> Flash<Redirect> {
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
	let player = Player::insert(&form.clone().into(), &connection);
	match player {
		Ok(_) => Flash::new(Redirect::to("/login"), "create_account_success", ""),
		Err(_) => Flash::new(Redirect::to("/create_account"), "unkown_error", "")
	}
}
#[post("/api/player/delete", data = "<delete_form>")]
pub fn delete(delete_form: Form<forms::DeletePlayer>, logged_in: AuthCont<LoginCookie>, mut cookies: Cookies, connection: DbConn) -> Flash<Redirect> {
	if let Ok(player) = Player::get(logged_in.cookie.player_id, &connection) {
		if player.password == delete_form.auth_password_for_delete {
			if Player::delete(logged_in.cookie.player_id, &connection).is_ok() {
				cookies.remove_private(Cookie::named(forms::LoginCookie::cookie_id()));
				return Flash::new(Redirect::to("/"), "deleted_account", "");
			}
		} else {
			return Flash::new(Redirect::to("/account"), "delete_account_wrong_password", "");
		}
	}
	Flash::new(Redirect::to("/account"), "unknown_error", "")
}
#[post("/api/player/login", data = "<login_form>")]
pub fn login(login_form: Form<forms::Login>, mut cookies: Cookies) -> Flash<Redirect> {
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
#[post("/api/player/logout")]
pub fn logout(player: Option<LoginCookie>, mut cookies: Cookies) -> Flash<Redirect> {
	if let Some(_) = player {
		cookies.remove_private(Cookie::named(forms::LoginCookie::cookie_id()));
	}
	Flash::new(Redirect::to("/"), "logout_success", "")
}