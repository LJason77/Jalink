use bson::{doc, from_document, to_document};
use mongodb::Database;
use rocket::{futures::StreamExt, get, http::Status, post, response::status::Custom, State};
use rocket_contrib::{
	json,
	json::{Json, JsonValue},
};

use crate::models::{
	message::{Message, NewMessage},
	Claims, Oid,
};

/// 表名
const COLLECTION_NAME: &str = "messages";

/// 创建消息
#[post("/<oid>", data = "<message>", format = "application/json")]
pub async fn create_message(
	claims: Claims,
	db: &State<Database>,
	oid: Oid,
	message: Json<NewMessage>,
) -> Custom<JsonValue> {
	let collection = db.collection(COLLECTION_NAME);
	let content = remove_spaces(&message.content);
	let result_insert = collection
		.insert_one(
			to_document(&json!({ "cid": &oid.0, "user": &claims.name, "content": content }))
				.unwrap(),
			None,
		)
		.await;

	let id = result_insert
		.unwrap()
		.inserted_id
		.as_object_id()
		.unwrap()
		.to_hex();

	Custom(Status::Ok, json!({ "_id": id }))
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
#[get("/<oid>")]
pub async fn get_messages(_claims: Claims, db: &State<Database>, oid: Oid) -> Custom<JsonValue> {
	let collection = db.collection(COLLECTION_NAME);
	let mut cursor = collection.find(doc! { "cid": &oid.0 }, None).await.unwrap();
	let mut messages = Vec::new();
	while let Some(result) = cursor.next().await {
		if let Ok(document) = result {
			messages.push(from_document::<Message>(document).unwrap());
		}
	}

	Custom(Status::Ok, json!(messages))
}
