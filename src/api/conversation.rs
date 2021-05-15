use bson::{doc, from_document, to_document, Document};
use mongodb::Database;
use rocket::{futures::StreamExt, get, http::Status, post, response::status::Custom, State};
use rocket_contrib::{
	json,
	json::{Json, JsonValue},
};

use crate::models::{
	conversation::{Conversation, Participant},
	user::AuthUser,
	Claims,
};

/// 表名
const COLLECTION_NAME: &str = "conversations";

/// 创建对话
#[post("/", data = "<participant>", format = "application/json")]
pub async fn create_conversation(
	claims: Claims,
	db: &State<Database>,
	participant: Json<Participant>,
) -> Custom<JsonValue> {
	// 查询另一个参与者
	let collection = db.collection("users");
	let option = collection
		.find_one(doc! { "name": &participant.name }, None)
		.await
		.unwrap();
	let user: AuthUser = match option {
		Some(doc) => from_document(doc).unwrap(),
		None => return Custom(Status::Ok, json!({ "error" : "用户不存在" })),
	};
	// 无法自言自语
	if &claims.name == &user.name {
		return Custom(Status::Forbidden, json!({ "error" : "无法自言自语" }));
	}

	// 寻找已有会话
	let collection = db.collection(COLLECTION_NAME);
	let option_doc: Option<Document> = collection
		.find_one(
			doc! { "participants": { "$all": [&claims.name, &user.name]} },
			None,
		)
		.await
		.unwrap();

	let doc = match option_doc {
		Some(doc) => doc,
		None => {
			// 未找到，创建会话
			let result_insert = collection
				.insert_one(
					to_document(&json!({ "participants": [ &claims.name, &participant.name ]}))
						.unwrap(),
					None,
				)
				.await;
			collection
				.find_one(doc! { "_id": result_insert.unwrap().inserted_id }, None)
				.await
				.unwrap()
				.unwrap()
		}
	};

	let conversation: Conversation = from_document(doc).unwrap();
	Custom(Status::Ok, json!(conversation))
}

/// 获取用户的所有对话
#[get("/")]
pub async fn get_conversations(claims: Claims, db: &State<Database>) -> Custom<JsonValue> {
	let collection = db.collection(COLLECTION_NAME);
	let mut cursor = collection
		.find(doc! { "participants": claims.name }, None)
		.await
		.unwrap();
	let mut conversations = Vec::new();
	while let Some(result) = cursor.next().await {
		if let Ok(document) = result {
			conversations.push(from_document::<Conversation>(document).unwrap());
		}
	}
	Custom(Status::Ok, json!(conversations))
}
