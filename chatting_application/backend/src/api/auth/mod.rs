mod google;

use axum::Router;
use crate::context::Context;

pub fn router() -> Router<Context> {
	Router::new()
	 .nest("/google", google::router())
}