use std::time::Duration;
use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Router;
use axum::routing::{get, post};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, TokenData};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use rand::distr::Alphanumeric;
use rand::RngExt;
use serde::Deserialize;
use serde_json::Value;
use tracing::info;
use url::Url;
use uuid::Uuid;
use crate::context::Context;

/// https://developers.google.com/identity/protocols/oauth2/javascript-implicit-flow
/// https://developers.google.com/identity/protocols/oauth2/web-server



pub fn router() -> Router<Context> {
	Router::new()
	 .route("/", get(google_auth))
	 .route("/callback", get(google_auth_callback))
}

async fn google_auth(
	State(state): State<Context>,
	jar: CookieJar,
) -> Response {
	let oauth_state: String = rand::rng()
	 .sample_iter(Alphanumeric)
	 .take(32)
	 .map(char::from)
	 .collect();

	info!("auth state: {}", oauth_state);

	let auth_url = state.0.google_oauth.get_auth_url(&oauth_state).await;

	let jar = jar.add(Cookie::build(("oauth_state", oauth_state))
	 .path("/")
	 .http_only(true)
	 .secure(false)
	 .same_site(SameSite::Lax)
	 .max_age(Duration::from_mins(30).try_into().unwrap())
	);

	(jar, Redirect::temporary(auth_url.as_str())).into_response()
}

#[derive(Deserialize)]
struct AuthCallbackQuery {
	code: String,
	state: String,
}

async fn google_auth_callback(
	State(state): State<Context>,
	Query(params): Query<AuthCallbackQuery>,
	headers: HeaderMap,
	jar: CookieJar,
) -> Result<Response, StatusCode> {
	let stored_state = jar.get("oauth_state").unwrap().value().to_string();

	if stored_state != params.state {
		panic!("OAuth state mismatch");
	}

	let response = state.0.google_oauth.exchange_code(params.code).await;
	let jwt = response.id_token;

	let header = jsonwebtoken::decode_header(&jwt).map_err(|_| StatusCode::UNAUTHORIZED)?;
	let kid = header.kid.ok_or(StatusCode::UNAUTHORIZED)?;
	let jwk = state.0.jwk_set.find(&kid).ok_or(StatusCode::UNAUTHORIZED)?;

	let claims: TokenData<GoogleClaims> = jsonwebtoken::decode(&jwt, &DecodingKey::from_jwk(jwk).unwrap(), &state.0.validation).map_err(|_| StatusCode::UNAUTHORIZED)?;
	info!("headers: {:#?}", claims.header);
	info!("claims: {:#?}", claims.claims);

	let google_account_id = claims.claims.sub;
	let google_account_name = claims.claims.name;
	let account_id = state.0.database.get_or_init_account_id_with_google_account_id(google_account_id, google_account_name).await;
	
	let session_id = state.0.database.add_session(account_id).await;

	let jar = jar.add(Cookie::build(("session", session_id.to_string()))
	 .path("/")
	 .http_only(true)
	 .secure(true)
	 .same_site(SameSite::Lax)
	 .max_age(Duration::from_mins(30).try_into().unwrap())
	);

	let jar = jar.add(Cookie::build(("account_id", account_id.to_string()))
	 .path("/")
	 .http_only(false)
	 .secure(false)
	 .same_site(SameSite::Lax)
	 .max_age(Duration::from_mins(30).try_into().unwrap())
	);

	Ok((jar, Redirect::temporary("/")).into_response())
}

/// https://developers.google.com/identity/openid-connect/openid-connect#an-id-tokens-payload
#[derive(Deserialize, Debug)]
struct GoogleClaims {
	pub sub: String,
	pub iss: String,
	pub iat: usize,
	pub exp: usize,

	pub name: String
}