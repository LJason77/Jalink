use content::Json;
use diesel::result::Error;
use diesel::CombineDsl;
use diesel::{
	BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
	RunQueryDsl,
};
use rocket::http::Status;
use rocket::response::content;
use rocket::{get, post};

use crate::models::conversation::{Conversation, NewConversation, Post, PostConversation};
use crate::models::database::DbConn;
use crate::models::participant::NewParticipant;
use crate::models::user::{Claims, User};
use crate::schema::{conversations, messages, participants, users};

/// 获取用户的所有对话
#[get("/")]
pub fn get_conversations(claims: Claims, connection: DbConn) {
	// TODO 待完善：获取用户的所有对话
	let results: Vec<(i32, Option<i32>, i32)> = conversations::table
		.inner_join(messages::table.on(messages::id.nullable().eq(conversations::last_message_id)))
		.inner_join(
			participants::table.on(participants::conversation_id
				.eq(conversations::id)
				.and(participants::user_id.eq(claims.id))),
		)
		.inner_join(users::table.on(users::id.eq(participants::user_id)))
		// .inner_join(
		// 	participants::table.on(participants::conversation_id
		// 		.eq(conversations::id)
		// 		.and(participants::user_id.eq(claims.id))
		// 	))
		.select((
			conversations::id,
			conversations::last_message_id.nullable(),
			messages::id,
		))
		.load(&*connection)
		.unwrap();
	println!("{:?}", results);

	let mut _conversations_: Vec<Conversation> = Vec::new();
	for _result in results {}
}

/// 获取单个对话
#[get("/<id>")]
pub fn get_conversation(claims: Claims, id: i32, connection: DbConn) {
	println!("authUserID：{}", claims.login);
	println!("{}", id);

	// TODO 待完善：获取单个对话
	let results: Vec<(i32, String, Option<String>)> = conversations::table
		.left_join(messages::table.on(messages::id.nullable().eq(conversations::last_message_id)))
		.inner_join(
			participants::table.on(participants::conversation_id
				.eq(conversations::id)
				.and(participants::user_id.eq(claims.id))),
		)
		.inner_join(users::table.on(users::id.eq(participants::user_id)))
		.select((users::id, users::username, users::avatar_url))
		.load(&*connection)
		.unwrap();
	println!("{:?}", results);
}

/// 创建对话
#[post("/", data = "<post>")]
pub fn create_conversation(
	claims: Claims,
	post: rocket_contrib::json::Json<Post>,
	connection: DbConn,
) -> Result<Json<String>, Status> {
	// 查询另一个参与者
	let user: Result<User, Error> = users::table
		.filter(users::username.eq(&post.username))
		.first::<User>(&*connection);
	let user = if let Ok(user) = user {
		user
	} else {
		return Err(Status::NotFound);
	};
	// 无法自言自语
	if &claims.id == &user.id {
		return Err(Status::Forbidden);
	}

	// 寻找已有会话
	let id: Result<i32, Error> = participants::table
		.select(participants::conversation_id)
		.filter(participants::user_id.eq(&claims.id))
		.intersect(
			participants::table
				.select(participants::conversation_id)
				.filter(participants::user_id.eq(&user.id)),
		)
		.get_result(&*connection);

	let mut post_conversation = PostConversation::default();
	match id {
		Ok(id) => {
			post_conversation.id = id;
			// return Redirect::to(format!("/conversations/{}", id))
		}
		Err(_) => {
			// 未找到，创建会话
			let conversation: Conversation = diesel::insert_into(conversations::table)
				.values(NewConversation {
					last_message_id: None,
				})
				.get_result(&*connection)
				.expect("无法添加对话");
			post_conversation.id = conversation.id;
			let new_participant = (
				NewParticipant {
					user_id: &claims.id,
					conversation_id: &conversation.id,
				},
				NewParticipant {
					user_id: &user.id,
					conversation_id: &conversation.id,
				},
			);
			diesel::insert_into(participants::table)
				.values(&new_participant)
				.execute(&*connection)
				.expect("保存新参加者时出错！");
		}
	}
	post_conversation.other_participant = user;
	Ok(Json(serde_json::to_string(&post_conversation).unwrap()))
}
