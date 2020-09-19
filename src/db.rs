// File: src/db.rs

use diesel::pg::PgConnection;
use r2d2;
use diesel::r2d2::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
// use std::env;
use std::ops::Deref;
use std::sync::Mutex;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref PGCONN: Mutex<DbConn> = Mutex::new( DbConn(init_pool().get().expect("Could not connect to database.")) );
}

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

fn database_url() -> String {
    String::from("postgres://postgres:postgres@localhost/roll_lang_db")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
	type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}