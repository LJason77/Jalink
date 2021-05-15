use bson::oid::ObjectId;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::{
	http::Status,
	outcome::Outcome,
	request::{self, FromParam, FromRequest},
	response::status::Custom,
	Request,
};
use rocket_contrib::{json, json::JsonValue};

use crate::models::{Claims, Oid};

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Claims {
	type Error = String;

	async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
		let encode_token = if let Some(bearer) = request.headers().get_one("Authorization") {
			bearer[7..].to_owned()
		} else {
			return Outcome::Failure((Status::Unauthorized, "未找到 Authorization".to_owned()));
		};

		let public_key = include_bytes!("public_ecdsa_key.pem");
		let decode_token = decode::<Claims>(
			&encode_token,
			&DecodingKey::from_ec_pem(public_key).unwrap(),
			&Validation::new(Algorithm::ES384),
		);
		match decode_token {
			Ok(token) => Outcome::Success(token.claims),
			Err(_) => Outcome::Failure((Status::Unauthorized, "Authorization 错误".to_owned())),
		}
	}
}

impl<'r> FromParam<'r> for Oid {
	type Error = Custom<JsonValue>;

	fn from_param(id: &'r str) -> Result<Self, Self::Error> {
		match ObjectId::with_string(&id) {
			Ok(oid) => Ok(Oid(oid)),
			Err(_) => Err(Custom(Status::Forbidden, json!({ "error" : "id 错误" }))),
		}
	}
}
