use axum::Router;
use axum_server::service::SendService;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use crate::api;
use crate::context::Context;

pub fn router() -> Router<Context> {
	let spa = ServeDir::new("./frontend/dist").fallback(ServeDir::new("./frontend/dist/index.html"));

	Router::new()
	 .nest("/api", api::router())
	 .fallback_service(spa)
}