use bson::{doc, from_document, to_document};
use mongodb::{options::FindOneOptions, Database};
use rocket::{http::Status, response::status::Custom, State};
use rocket_contrib::{json, json::JsonValue};
use serde::{Deserialize, Serialize};

pub mod conversation;
pub mod index;
pub mod message;

/// 创建对象
#[inline(always)]
pub async fn create<N, T>(
	db: &State<Database>,
	collection_name: &str,
	new: N,
	find_one_options: FindOneOptions,
) -> Custom<JsonValue>
where
	N: Serialize,
	T: for<'de> Deserialize<'de> + Serialize,
{
	let collection = db.collection(collection_name);
	let result_insert = collection
		.insert_one(to_document(&new).unwrap(), None)
		.await;
	let oid = if let Ok(result) = result_insert {
		result.inserted_id
	} else {
		return Custom(Status::Conflict, json!({ "error" : "已存在" }));
	};
	let doc = collection
		.find_one(doc! { "_id": oid }, find_one_options)
		.await
		.unwrap()
		.unwrap();
	let t: T = from_document(doc).unwrap();
	Custom(Status::Ok, json!(t))
}
