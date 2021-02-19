use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::messages;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePost {
	pub content: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Message {
	pub id: i32,
	pub content: String,
	pub user_id: i32,
	pub conversation_id: i32,
	pub created_at: NaiveDateTime,
	// pub mine: bool,
	// pub receiver_id: String,
}

#[derive(Default, Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
	pub content: &'a str,
	pub user_id: i32,
	pub conversation_id: i32,
}
