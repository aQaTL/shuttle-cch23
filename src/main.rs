use axum::{http::StatusCode, routing::get, Router};
use shuttle_axum::AxumService;

#[shuttle_runtime::main]
async fn shuttle_main() -> shuttle_axum::ShuttleAxum {
	let router = Router::new()
		.route("/", get(nothing))
		.route("/hello", get(hello_world))
		.route("/-1/error", get(error_page));
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
