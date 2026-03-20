pub mod google;

use crate::context::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use std::str::FromStr;
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Deserialize;
use uuid::Uuid;

pub fn router() -> Router<Context> {
	Router::new()
		.route("/", post(post_auth))
	 .route("/check", get(get_auth_check))
	 .nest("/google", google::router())
}

#[derive(Deserialize)]
struct PostAuthRequest {
	account_name: String,
}

async fn post_auth(
	State(state): State<Context>,
	jar: CookieJar,
	Json(request): Json<PostAuthRequest>,
) -> Response {
	let account_name = request.account_name;
	
	let account_id = state.0.database.add_account(&account_name).await;
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
	
	jar.into_response()
}

async fn get_auth_check(
	State(state): State<Context>,
	jar: CookieJar,
) -> Response {
	if let Some(session_id) = jar.get("session_id") {
		let session_id = Uuid::from_str(session_id.value()).unwrap();

		if let Some(account) = state.0.database.get_account_by_session_id(session_id).await {
			Json(account).into_response()
		} else {
			StatusCode::UNAUTHORIZED.into_response()
		}
	} else {
		StatusCode::UNAUTHORIZED.into_response()
	}
}