use rocket::get;
use rocket::http::{Cookie, CookieJar};

#[get("/")]
pub async fn index() -> &'static str {
	"Hi"
}

#[get("/callback?<expires_at>&<token>")]
pub async fn index_callback(
	expires_at: String,
	token: String,
	cookies: &CookieJar<'_>,
) -> &'static str {
	cookies.add_private(Cookie::new("expires_at", expires_at));
	cookies.add_private(Cookie::new("token", token));
	"Hi"
}
