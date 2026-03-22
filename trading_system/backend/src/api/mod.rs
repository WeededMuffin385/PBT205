use axum::Router;
use crate::context::Context;

mod auth;

pub fn router(context: Context) -> Router<Context> {
    Router::new()
        .nest("/auth", auth::router())
}