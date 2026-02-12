use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use crate::context::Context;

pub struct Authentication {

}

impl FromRequestParts<Context> for Authentication {
	type Rejection = StatusCode;

	async fn from_request_parts(parts: &mut Parts, state: &Context) -> Result<Self, Self::Rejection> {
		let jwt = parts.headers.get("x-auth-jwt").ok_or(StatusCode::BAD_REQUEST)?;

		todo!("check if jwt is valid, and if so, let the connection in")
	}
}