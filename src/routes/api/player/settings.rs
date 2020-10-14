// File: src/routes/settings.rs

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


#[post("/api/player/settings/name/update", data = "<update_name_form>")]
pub fn update_name(update_name_form: Form<forms::UpdateName>, logged_in: AuthCont<LoginCookie>, mut cookies: Cookies, connection: DbConn) -> Flash<Redirect> {
	if let Ok(password) = Player::password_from_id(logged_in.cookie.player_id, &connection) {
		let inner = update_name_form.into_inner();
		if password == inner.auth_password_for_name {
			if Player::id_from_name(&inner.new_name, &connection).is_ok() {
				return Flash::new(Redirect::to("/account"), "update_name_taken", "");
			}
			if let Ok(_player) = Player::update_name(logged_in.cookie.player_id, &inner.new_name, &connection) {
				let login = forms::Login {
					name: inner.new_name,
					password: password,
				};
				if let Ok(cookie) = login.authenticate() {
					let cookie_id = forms::Login::cookie_id();
					cookies.remove_private(Cookie::named(cookie_id));
					let contents = cookie.store_cookie();
					cookies.add_private(Cookie::new(cookie_id, contents));
					return Flash::new(Redirect::to("/account"), "changed_name", "");
				}
			}
		} else {
			return Flash::new(Redirect::to("/account"), "update_name_wrong_password", "");
		}
	}
	Flash::new(Redirect::to("/account"), "unknown_error", "")
}
#[post("/api/player/settings/email/update", data = "<update_email_form>")]
pub fn update_email(update_email_form: Form<forms::UpdateEmail>, logged_in: AuthCont<LoginCookie>, connection: DbConn) -> Flash<Redirect> {
	if let Ok(password) = Player::password_from_id(logged_in.cookie.player_id, &connection) {
		let inner = update_email_form.into_inner();
		if password == inner.auth_password_for_email {
			if Player::id_from_email(&inner.new_email, &connection).is_ok() {
				return Flash::new(Redirect::to("/account"), "update_email_taken", "");
			}
			if Player::update_email(logged_in.cookie.player_id, &inner.new_email, &connection).is_ok() {
				return Flash::new(Redirect::to("/account"), "changed_email", "");
			}
		} else {
			return Flash::new(Redirect::to("/account"), "update_email_wrong_password", "");
		}
	}
	Flash::new(Redirect::to("/account"), "unknown_error", "")
}
#[post("/api/player/settings/password/update", data = "<update_password_form>")]
pub fn update_password(update_password_form: Form<forms::UpdatePassword>, logged_in: AuthCont<LoginCookie>, mut cookies: Cookies, connection: DbConn) -> Flash<Redirect> {
	if let Ok(password) = Player::password_from_id(logged_in.cookie.player_id, &connection) {
		let inner = update_password_form.into_inner();
		if password == inner.auth_password {
			if inner.new_password != inner.confirm_password {
				return Flash::new(Redirect::to("/account"), "update_password_dont_match", "");
			}
			if inner.new_password.len() >= 8 {
				return Flash::new(Redirect::to("/account"), "update_password_too_short", "");
			}
			if inner.new_password.len() <= 32 {
				return Flash::new(Redirect::to("/account"), "update_password_too_long", "");
			}
			// check characters in password
			if let Ok(_player) = Player::update_password(logged_in.cookie.player_id, &inner.new_password, &connection) {
				let login = forms::Login {
					name: logged_in.cookie.name,
					password: password,
				};
				if let Ok(cookie) = login.authenticate() {
					let cookie_id = forms::Login::cookie_id();
					cookies.remove_private(Cookie::named(cookie_id));
					let contents = cookie.store_cookie();
					cookies.add_private(Cookie::new(cookie_id, contents));
					return Flash::new(Redirect::to("/account"), "changed_password", "");
				}
			}
		} else {
			return Flash::new(Redirect::to("/account"), "update_password_wrong_password", "");
		}
	}
	Flash::new(Redirect::to("/account"), "unknown_error", "")
}