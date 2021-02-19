use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub id: i32,
	pub login: String,
	pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct Github {
	#[serde(rename = "id", default)]
	pub github_id: i32,
	#[serde(rename = "login", default)]
	pub username: String,
	pub avatar_url: String,
}

#[derive(Debug, Default, Queryable, Serialize, Deserialize)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub avatar_url: Option<String>,
	pub github_id: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
	pub username: &'a str,
	pub avatar_url: Option<&'a str>,
	pub github_id: &'a i32,
}
