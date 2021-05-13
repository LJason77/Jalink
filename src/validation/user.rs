use crate::models::user::Claims;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::request::FromRequest;
use rocket::{request, Request};

pub const SECRET_KEY: &[u8; 6] = b"secret";

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
	type Error = ();

	async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		// 从 Authorization 获取加密的 token
		let encode_token = if let Some(bearer) = request.headers().get_one("Authorization") {
			&bearer[7..]
		} else {
			return Failure((Status::Unauthorized, ()));
		};

		// 解码 token
		let decode_token = decode::<Claims>(
			encode_token,
			&DecodingKey::from_secret(SECRET_KEY),
			&Validation::new(Algorithm::HS256),
		);

		match decode_token {
			Ok(token) => Success(token.claims),
			// 解密 token 失败
			Err(_) => return Failure((Status::Unauthorized, ())),
		}
	}
}
