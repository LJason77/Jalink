use serde::{Deserialize, Serialize};

use super::deserialize_oid;

#[derive(Serialize, Deserialize)]
pub struct Message {
	/// id
	#[serde(deserialize_with = "deserialize_oid")]
	pub _id: String,
	/// 对话 id
	#[serde(deserialize_with = "deserialize_oid")]
	pub cid: String,
	/// 发送者
	pub user: String,
	/// 消息正文
	pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
	/// 消息正文
	pub content: String,
}
