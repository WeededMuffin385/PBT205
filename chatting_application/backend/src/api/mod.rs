pub mod auth;
mod channels;

use axum::Router;
use crate::context::Context;

pub fn router(context: Context) -> Router<Context> {
	Router::new()
	 .nest("/auth", auth::router())
	 .nest("/channels", channels::router(context))
}