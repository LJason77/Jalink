use diesel::dsl::now;
use diesel::{
	BoolExpressionMethods, ExpressionMethods, PgTimestampExpressionMethods, QueryDsl, RunQueryDsl,
};
use rocket::http::Status;
use rocket::response::content;
use rocket::{get, post};
use rocket_contrib::json::Json;

use crate::models::conversation::Conversation;
use crate::models::database::DbConn;
use crate::models::message::{Message, MessagePost, NewMessage};
use crate::models::user::Claims;
use crate::schema::{conversations, messages, participants};

// TODO 订阅消息：EventSource
// #[get("/")]
// pub fn subscribe_to_messages() {}

/// 创建消息
#[post("/<conversation_id>/messages", data = "<message_post>")]
pub fn create_message(
	claims: Claims,
	conversation_id: i32,
	message_post: Json<MessagePost>,
	connection: DbConn,
) -> Result<content::Json<String>, Status> {
	// 查询对话中的参与者是否存在
	let conversation = conversations::table
		.find(conversation_id)
		.first::<Conversation>(&*connection);
	if let Err(_) = conversation {
		return Err(Status::NotFound);
	}
	let content = remove_spaces(&message_post.content);
	if content.len() > 500 {
		return Err(Status::Forbidden);
	}

	let messages: Message = diesel::insert_into(messages::table)
		.values(NewMessage {
			content: &content,
			user_id: claims.id,
			conversation_id,
		})
		.get_result(&*connection)
		.expect("无法添加消息");
	// 插入消息并更新对话 last_message_id
	let _ = diesel::update(conversations::table.find(conversation_id))
		.set(conversations::last_message_id.eq(messages.id))
		.execute(&*connection)
		.unwrap();
	// TODO 通知消息
	Ok(content::Json(serde_json::to_string(&messages).unwrap()))
}

/// 删除两边和两个以上的的连续空格
fn remove_spaces(s: &str) -> String {
	let s: Vec<&str> = s.trim().split("  ").collect();
	let mut a = String::new();
	for x in s.into_iter() {
		if x != "" {
			a.push_str(format!("{} ", x).as_str())
		}
	}
	a.trim().replace("  ", " ")
}

/// 获取消息
#[get("/<conversation_id>/messages")]
pub fn get_message(
	claims: Claims,
	conversation_id: i32,
	connection: DbConn,
) -> Result<content::Json<String>, Status> {
	println!("{}", claims.login);
	println!("get_message: {}", conversation_id);

	// 查询对话中的参与者是否存在
	let conversation = conversations::table
		.find(conversation_id)
		.first::<Conversation>(&*connection);
	if let Err(_) = conversation {
		return Err(Status::NotFound);
	}

	let messages: Vec<Message> = messages::table
		.filter(messages::conversation_id.eq(conversation_id))
		.order(messages::created_at.desc())
		.load::<Message>(&*connection)
		.expect("加载对话错误！");

	Ok(content::Json(serde_json::to_string(&messages).unwrap()))
}

/// 读取消息
#[post("/<conversation_id>/read_messages")]
pub fn read_message(claims: Claims, conversation_id: i32, connection: DbConn) -> Status {
	println!("{}", claims.login);
	println!("read_message: {}", conversation_id);

	diesel::update(
		participants::table.filter(
			participants::user_id
				.eq(&claims.id)
				.and(participants::conversation_id.eq(conversation_id)),
		),
	)
	.set(participants::messages_read_at.eq(now.at_time_zone("Asia/Shanghai")))
	.execute(&*connection)
	.unwrap();

	Status::NoContent
}
