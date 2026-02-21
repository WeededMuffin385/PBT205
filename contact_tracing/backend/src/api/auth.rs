use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use uuid::Uuid;
use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new()
        .route("/", post(post_auth))
        .route("/check", get(get_auth_check))
}

#[derive(Deserialize)]
struct AuthRequest {
    account_name: String,
}

async fn get_auth_check(
    State(state): State<Context>,
    jar: CookieJar,
) -> Response {
    if let Some(session_id) = jar.get("session_id") {
        StatusCode::OK.into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

async fn post_auth(
    State(state): State<Context>,
    jar: CookieJar,
    Json(request): Json<AuthRequest>,
) -> Result<Response, StatusCode> {
    let account_id = state.0.database.add_account(request.account_name).await;
    let session_id = state.0.database.add_account_session_id(account_id).await;

    let jar = jar.add(Cookie::build(("session_id", session_id.to_string()))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
    );

    let jar = jar.add(Cookie::build(("account_id", account_id.to_string()))
        .path("/")
        .secure(false)
        .http_only(false)
        .same_site(SameSite::Lax)
    );

    Ok((jar).into_response())
}