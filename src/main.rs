use axum::extract::Path;
use axum::routing::post;
use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Deserialize;
use shuttle_axum::AxumService;

#[shuttle_runtime::main]
async fn shuttle_main() -> shuttle_axum::ShuttleAxum {
	let router = Router::new()
		.route("/", get(nothing))
		.route("/hello", get(hello_world))
		.route("/-1/error", get(error_page))
		.route("/1/*nums", get(day_1))
		.route("/4/strength", post(day_4));
	Ok(AxumService(router))
}

async fn nothing() -> &'static str {
	""
}

async fn hello_world() -> &'static str {
	"Hello, Shuttle!"
}

async fn error_page() -> (StatusCode, &'static str) {
	(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
}

async fn day_1(Path(nums): Path<String>) -> String {
	nums.split('/')
		.map(|x| x.parse::<i64>().unwrap())
		.fold(0, |acc, x| acc ^ x)
		.pow(3)
		.to_string()
}

#[derive(Deserialize)]
struct Reindeer {
	#[allow(dead_code)]
	name: String,
	strength: i32,
}

async fn day_4(Json(reindeer): Json<Vec<Reindeer>>) -> String {
	reindeer
		.into_iter()
		.map(|r| r.strength)
		.sum::<i32>()
		.to_string()
}
