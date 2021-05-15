use rocket::routes;

mod api;
mod models;
mod validation;

/// 初始化 数据库连接池
async fn init_pool() -> mongodb::Database {
	let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
		"mongodb://root:root@127.0.0.1:27017/messenger?authSource=admin".to_string()
	});
	let client = mongodb::Client::with_uri_str(&database_url).await.unwrap();
	client.database("messenger")
}

#[rocket::main]
async fn main() -> Result<(), String> {
	dotenv::dotenv().ok();

	let db = init_pool().await;

	let rocket = rocket::build()
		.manage(db)
		.mount(
			"/conversations",
			routes![
				api::conversation::create_conversation,
				api::conversation::get_conversations
			],
		)
		.mount(
			"/messages",
			routes![api::message::create_message, api::message::get_messages],
		)
		.mount("/", routes![api::index::register, api::index::login]);

	if let Err(err) = rocket.launch().await {
		println!("Rocket Err: {}", err);
	}
	Ok(())
}
