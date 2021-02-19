#[rocket::main]
async fn main() -> Result<(), String> {
	let rocket = jalink::rocket_factory().unwrap();
	if let Err(err) = rocket.launch().await {
		println!("{}", err);
	}
	Ok(())
}
