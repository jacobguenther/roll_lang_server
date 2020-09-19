// File: src/forms/mod.rs

use rocket::{
	Request,
	Outcome,
	request::{FromRequest},
};
use serde::{Deserialize, Serialize};
use auth::authorization::*;
use std::collections::HashMap;

use crate::models::player::*;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
pub struct CreateAccount {
	pub player_name: String,
	pub email: String,
    pub password: String,
    pub confirm_password: String,
}


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
pub struct Login {
	pub name: String,
	pub password: String,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
pub struct Logout {
    pub _dummy: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCookie {
	userid: i32,
	pub name: String,
}




impl CookieId for Login {
	fn cookie_id<'a>() -> &'a str {
		"acid_text"
	}
}

impl CookieId for LoginCookie {
	fn cookie_id<'a>() -> &'a str {
		"acid_text"
	}
}
impl AuthorizeCookie for LoginCookie {
    fn store_cookie(&self) -> String {
        ::serde_json::to_string(self).expect("Could not serialize")
    }

    #[allow(unused_variables)]
    fn retrieve_cookie(string: String) -> Option<Self> {
        let mut des_buf = string.clone();
        let des: Result<LoginCookie, _> = ::serde_json::from_str(&mut des_buf);
        if let Ok(cooky) = des {
            Some(cooky)
        } else {
            None
        }
    }
}

impl AuthorizeForm for Login {
    type CookieType = LoginCookie;

    fn authenticate(&self) -> Result<Self::CookieType, AuthFail> {
        let conn = match crate::db::PGCONN.lock() {
            Ok(c) => c,
            Err(e) => return Err(AuthFail::new(self.name.clone(), format!("{:?}", e))),
        };
        let player_id = match Player::id_from_name(&self.name, &conn) {
            Ok(id) => id,
            Err(e) => return Err(AuthFail::new(self.name.clone(), format!("{:?}", e))),
        };
        let player = match Player::get(player_id, &conn) {
            Ok(p) => p,
            Err(e) => return Err(AuthFail::new(self.name.clone(), format!("{:?}", e))),
        };

        if player.name == self.name && player.password == self.password {
            return Ok(LoginCookie {
                userid: player.id,
                name: player.name,
            })
        }
        Err(AuthFail::new(self.name.clone(), "Unknown error..".to_string()))
    }

    /// Create a new login form instance
    fn new_form(user: &str, pass: &str, _extras: Option<HashMap<String, String>>) -> Self {
        Login {
			name: user.to_string(),
            password: pass.to_string(),
        }
    }

}

impl<'a, 'r> FromRequest<'a, 'r> for LoginCookie {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<LoginCookie,Self::Error>{
        let cid = LoginCookie::cookie_id();
        let mut cookies = request.cookies();

        match cookies.get_private(cid) {
            Some(cookie) => {
                if let Some(cookie_deserialized) = LoginCookie::retrieve_cookie(cookie.value().to_string()) {
                    Outcome::Success(
                        cookie_deserialized
                    )
                } else {
                    Outcome::Forward(())
                }
            },
            None => Outcome::Forward(())
        }
    }
}

