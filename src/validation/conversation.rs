use rocket::data::{ByteUnit, FromData, Outcome};
use rocket::http::{ContentType, Status};
use rocket::outcome::Outcome::{Failure, Forward, Success};
use rocket::{Data, Request};

use crate::models::conversation::Post;

// 防止DoS攻击
const LIMIT: ByteUnit = ByteUnit::Byte(256);

#[rocket::async_trait]
impl FromData for Post {
	type Error = String;

	async fn from_data(request: &Request<'_>, data: Data) -> Outcome<Self, Self::Error> {
		// 打开数据之前，确保内容类型正确。
		let post_ct = ContentType::new("application", "json");
		if request.content_type() != Some(&post_ct) {
			return Forward(data);
		}

		// 将数据读入字符串。
		let limit = request.limits().get("post").unwrap_or(LIMIT);
		let json = match data.open(limit).stream_to_string().await {
			Ok(s) => s,
			Err(err) => return Failure((Status::InternalServerError, format!("{}", err))),
		};
		let post: Post = serde_json::from_str(&json).unwrap();
		Success(post)
	}
}
