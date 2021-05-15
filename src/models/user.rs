use serde::{Deserialize, Serialize};

use super::{default_object_id, deserialize_oid};

#[derive(Serialize, Deserialize)]
pub struct AuthUser {
	#[serde(deserialize_with = "deserialize_oid")]
	pub _id: String,
	pub name: String,
	pub password: String,
}

/// 新账户
#[derive(Serialize, Deserialize)]
pub struct NewUser {
	/// 用户名
	pub name: String,
	/// 密码
	pub password: String,
}

/// 账户
#[derive(Serialize, Deserialize)]
pub struct User {
	/// id
	#[serde(default = "default_object_id", deserialize_with = "deserialize_oid")]
	pub _id: String,
	/// 账户名
	pub name: String,
}
