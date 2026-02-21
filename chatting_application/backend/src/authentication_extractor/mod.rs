use std::str::FromStr;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use uuid::Uuid;
use crate::context::Context;
use crate::types::account::Account;

#[derive(Clone, Debug)]
pub struct Authentication {
	pub account: Account
}

impl FromRequestParts<Context> for Authentication {
	type Rejection = StatusCode;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &Context,
	) -> Result<Self, Self::Rejection> {
		let jar = CookieJar::from_request_parts(parts, state).await.unwrap();
		if let Some(session) = jar.get("session") {
			if let Some(account) = state.0.database.get_account_by_session_id(Uuid::from_str(session.value()).unwrap()).await {
				Ok(Authentication {
					account
				})
			} else {
				Err(StatusCode::UNAUTHORIZED)
			}
		} else {
			Ok(Authentication {
				account: Account {
					account_id: 0,
					account_name: "guest".to_string(),
				}
			})
		}
	}
}