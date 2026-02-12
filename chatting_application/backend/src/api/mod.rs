mod auth;
mod messages;

use axum::Router;
use crate::context::Context;

pub fn router() -> Router<Context> {
	Router::new()
	 .nest("/auth", auth::router())
	 .nest("/messages", messages::router())
}