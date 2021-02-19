use std::env;

use bytes::Buf as _;
use chrono::offset::Local;
use chrono::{SecondsFormat, TimeZone};
use diesel::query_dsl::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};
use hyper::{Body, Method, Request as hy_Request};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicTokenResponse, BasicTokenType};
use oauth2::reqwest::http_client;
use oauth2::{
	AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
	TokenResponse, TokenUrl,
};
use rocket::get;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::{content, Redirect};

use crate::models::database::DbConn;
use crate::models::user::{Claims, Github, NewUser, User};
use crate::schema::users;
use crate::validation::user::SECRET_KEY;

const JWT_LIFETIME: i64 = 60 * 60 * 24 * 14;

fn get_client() -> Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType> {
	let github_client_id =
		ClientId::new(env::var("GITHUB_CLIENT_ID").expect("未设置 GITHUB_CLIENT_ID！"));
	let github_client_secret =
		ClientSecret::new(env::var("GITHUB_CLIENT_SECRET").expect("未设置 GITHUB_CLIENT_SECRET！"));

	let github_auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
		.expect("授权端点URL无效");
	let github_token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
		.expect("无效的令牌端点 URL");

	BasicClient::new(
		github_client_id,
		Some(github_client_secret),
		github_auth_url,
		Some(github_token_url),
	)
	.set_redirect_url(
		RedirectUrl::new("http://127.0.0.1:8000/oauth/github/callback".to_string())
			.expect("无效的重定向网址"),
	)
}

#[get("/github")]
pub fn github_oauth_start(cookies: &CookieJar<'_>) -> Redirect {
	let cookies_token = cookies
		.get_private("token")
		.map(|crumb| crumb.value().to_owned());
	if let Some(token) = cookies_token {
		let result_encoding_token = decode::<Claims>(
			&token,
			&DecodingKey::from_secret(SECRET_KEY),
			&Validation::new(Algorithm::HS256),
		);
		if let Ok(_) = result_encoding_token {
			// 如果 token 存在，并且没有过期，则直接跳转，不需要再次认证
			return Redirect::to("/".to_string());
		}
	}
	// 没有 token 则认证
	let client = get_client();
	let (authorize_url, csrf_state) = client
		.authorize_url(CsrfToken::new_random)
		.add_scope(Scope::new("read:user".to_string()))
		.url();
	cookies.add_private(Cookie::new("csrf_state", csrf_state.secret().to_owned()));
	Redirect::to(authorize_url.to_string())
}

#[get("/github/callback?<code>&<state>")]
pub async fn github_oauth_callback(
	code: String,
	state: String,
	cookies: &CookieJar<'_>,
	connection: DbConn,
) -> Result<Redirect, Status> {
	let csrf_state = cookies
		.get_private("csrf_state")
		.map(|crumb| crumb.value().to_string());
	cookies.remove_private(Cookie::named("csrf_state"));
	if state != csrf_state.unwrap() {
		return Err(Status::ImATeapot);
	}

	let token_response = get_client()
		.exchange_code(AuthorizationCode::new(code))
		.request(http_client)
		.unwrap();
	let auth_token = format!("token {}", token_response.access_token().secret());

	// 手动获取用户信息
	let https = hyper_tls::HttpsConnector::new();
	let client = hyper::Client::builder().build::<_, hyper::Body>(https);
	let request = hy_Request::builder()
		.method(Method::GET)
		.uri("https://api.github.com/user")
		.header("Authorization", auth_token)
		.header("User-Agent", "ja_link/1.0")
		.body(Body::empty())
		.unwrap();
	let response = client.request(request).await.unwrap();

	let body = hyper::body::aggregate(response).await.unwrap();
	let github: Github = serde_json::from_reader(body.reader()).unwrap();

	let user: Result<User, diesel::result::Error> = users::table
		.filter(users::github_id.eq(&github.github_id))
		.first::<User>(&*connection);
	let user: User = match user {
		Ok(user) => user,
		Err(_) => {
			let new_user = NewUser {
				username: &github.username,
				avatar_url: Some(&github.avatar_url),
				github_id: &github.github_id,
			};
			diesel::insert_into(users::table)
				.values(&new_user)
				.get_result(&*connection)
				.expect("保存新用户时出错！")
		}
	};

	let expiration = (Local::now().timestamp() + JWT_LIFETIME) as usize;
	let token = encoding_token(&user.id, &github.username, expiration);
	let expires_at = Local
		.timestamp(expiration as i64, 0)
		.to_rfc3339_opts(SecondsFormat::Millis, false);
	Ok(Redirect::to(format!(
		"/callback?expires_at={}&token={}",
		&expires_at, &token
	)))
}

/// 编码 token
fn encoding_token(id: &i32, login: &str, expiration: usize) -> String {
	let claims = Claims {
		id: id.to_owned(),
		login: login.to_owned(),
		exp: expiration,
	};

	let mut header = Header::default();
	header.kid = Some("signing_key".to_owned());
	header.alg = Algorithm::HS256;

	match encode(&header, &claims, &EncodingKey::from_secret(SECRET_KEY)) {
		Ok(token) => token,
		Err(err) => panic!("{:?}", err),
	}
}

#[get("/auth_user")]
pub fn get_auth_user(claims: Claims, connection: DbConn) -> content::Json<String> {
	let user: User = users::table
		.filter(users::id.eq(&claims.id))
		.first::<User>(&*connection)
		.expect("加载用户错误！");
	content::Json(serde_json::to_string(&user).unwrap())
}
