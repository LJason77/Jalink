//! index
//!
//! # 登录
//!
//! api： /login
//! - 方法： post
//! - 数据： [Login](../../models/user/struct.NewUser.html)
//! - 返回：`{ "token": token }`
//!
//! # 注册
//!
//! api： /register
//! - 方法： post
//! - 数据： [Register](../../models/user/struct.NewUser.html)
//! - 返回：`{ "_id": id, "name": name }`

use bson::{doc, from_document};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use mongodb::{options::FindOneOptions, Database};
use rocket::{
	http::Status,
	{post, response::status::Custom, State},
};
use rocket_contrib::{
	json,
	json::{Json, JsonValue},
};

use crate::models::user::{AuthUser, NewUser, User};

/// 表名
const COLLECTION_NAME: &str = "users";

/// 用户注册
#[post("/register", data = "<register>", format = "application/json")]
pub async fn register(db: &State<Database>, register: Json<NewUser>) -> Custom<JsonValue> {
	let find_one_options = FindOneOptions::builder()
		.projection(doc! { "password": 0 })
		.sort(doc! { "_id": -1 })
		.build();
	super::create::<NewUser, User>(db, COLLECTION_NAME, register.0, find_one_options).await
}

/// 用户登录
#[post("/login", data = "<login>", format = "application/json")]
pub async fn login(db: &State<Database>, login: Json<NewUser>) -> Custom<JsonValue> {
	let collection = db.collection(COLLECTION_NAME);
	let option = collection
		.find_one(doc! { "name": &login.name }, None)
		.await
		.unwrap();
	let user: AuthUser = match option {
		Some(doc) => from_document(doc).unwrap(),
		None => return Custom(Status::Ok, json!({ "error" : "用户不存在" })),
	};

	// 用户名或密码错误
	if user.password != login.password {
		return Custom(Status::BadRequest, json!({ "error" : "用户名或密码错误" }));
	}

	let token = format!("Bearer {}", encoding_token(&user));
	Custom(Status::Ok, json!({ "token": token }))
}

/// 9 小时
const JWT_LIFETIME: usize = 60 * 60 * 9;

/// 编码 token
fn encoding_token(user: &AuthUser) -> String {
	let id = (&user._id).to_owned();
	let name = (&user.name).to_owned();
	let iat = chrono::Local::now().timestamp() as usize;
	let exp = iat + JWT_LIFETIME;
	let claims = crate::models::Claims { id, name, iat, exp };

	let private_key = include_bytes!("private_ecdsa_key.pem");
	let encrypted = encode(
		&Header::new(Algorithm::ES384),
		&claims,
		&EncodingKey::from_ec_pem(private_key).unwrap(),
	);
	encrypted.unwrap()
}
