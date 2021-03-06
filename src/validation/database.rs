use std::ops::Deref;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::models::database::DbConn;

type DatabasePool = Pool<ConnectionManager<diesel::PgConnection>>;

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
	type Error = ();

	async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		let pool = request
			.guard::<rocket::State<DatabasePool>>()
			.await
			.unwrap();
		match pool.get() {
			Ok(conn) => Outcome::Success(DbConn(conn)),
			Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
		}
	}
}

impl Deref for DbConn {
	type Target = PgConnection;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
