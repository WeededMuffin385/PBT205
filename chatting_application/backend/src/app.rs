use axum::Router;
use axum_server::service::SendService;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use crate::api;
use crate::context::Context;

pub fn router(context: Context) -> Router<Context> {
	let spa = ServeDir::new("./frontend/dist").fallback(ServeFile::new("./frontend/dist/index.html"));

	Router::new()
	 .nest("/api", api::router(context))
	 .fallback_service(spa)
}