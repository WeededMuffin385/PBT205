use axum::extract::State;
use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new()
        .route("/", get(get_accounts))
}

async fn get_accounts(
    State(state): State<Context>
) -> Response {
    let accounts = state.0.database.get_accounts().await;
    Json(accounts).into_response()
}