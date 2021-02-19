#[macro_use]
extern crate diesel;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use rocket::routes;

pub mod api;
pub mod models;
pub mod schema;
pub mod validation;

/// 初始化 数据库连接池
fn init_pool() -> Pool<ConnectionManager<PgConnection>> {
	let database_url = std::env::var("DATABASE_URL")
		.unwrap_or_else(|_| "postgres://root:a@127.0.0.1:5432/messenger".to_string());
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	Pool::builder().max_size(32).build(manager).unwrap()
}

pub fn rocket_factory() -> Result<rocket::Rocket, String> {
	dotenv::dotenv().ok();

	let pool = init_pool();

	let rocket = rocket::ignite()
		.manage(pool)
		.mount(
			"/oauth",
			routes![
				api::auth::github_oauth_start,
				api::auth::github_oauth_callback,
				api::auth::get_auth_user
			],
		)
		.mount(
			"/conversations",
			routes![
				api::conversation::get_conversation,
				api::conversation::get_conversations,
				api::conversation::create_conversation
			],
		)
		.mount(
			"/conversations",
			routes![
				api::message::create_message,
				api::message::get_message,
				api::message::read_message
			],
		)
		// .mount("/messages", routes![api::message::subscribe_to_messages])
		.mount("/", routes![api::index::index, api::index::index_callback]);
	Ok(rocket)
}
