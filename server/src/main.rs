use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
	let app = static_frontend();
	let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
	let socket_addr = listener.local_addr().unwrap();

	println!("listening to {}:{}", socket_addr.ip().to_string(), socket_addr.port().to_string());
	axum::serve(listener, app)
		.await
		.unwrap();
}

fn static_frontend() -> Router {
	// Give ServeDir the location to be served
	let static_frontend_dir = ServeDir::new("../client/build");

	// Generate the router to use the root path
	Router::new().fallback_service(static_frontend_dir)
}