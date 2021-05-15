use bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serialize};

pub mod conversation;
pub mod message;
pub mod user;

#[derive(Serialize, Deserialize)]
pub struct Claims {
	pub id: String,
	pub name: String,
	pub iat: usize,
	pub exp: usize,
}

pub struct Oid(pub ObjectId);

/// OID 转字符串
pub fn deserialize_oid<'de, D>(d: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	let id: ObjectId = Deserialize::deserialize(d).unwrap();
	Ok(id.to_hex())
}

/// 生成默认 OID 字符串
pub fn default_object_id() -> String {
	ObjectId::new().to_hex()
}
