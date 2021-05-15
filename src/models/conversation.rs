use serde::{Deserialize, Serialize};

use super::{default_object_id, deserialize_oid};

#[derive(Serialize, Deserialize)]
pub struct Participant {
	/// 参与者
	pub name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Conversation {
	#[serde(default = "default_object_id", deserialize_with = "deserialize_oid")]
	pub _id: String,
	/// 参与者
	pub participants: Vec<String>,
}
