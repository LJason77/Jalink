use serde::{Deserialize, Serialize};

use crate::models::message::Message;
use crate::models::user::User;
use crate::schema::conversations;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
	pub username: String,
}

#[derive(Debug, Default, Serialize)]
pub struct PostConversation {
	pub id: i32,
	pub other_participant: User,
	pub last_message: Option<Message>,
	pub has_unread_messages: bool,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Conversation {
	pub id: i32,
	pub last_message: Option<i32>,
}

#[derive(Default, Insertable)]
#[table_name = "conversations"]
pub struct NewConversation<'a> {
	pub last_message_id: Option<&'a i32>,
}
