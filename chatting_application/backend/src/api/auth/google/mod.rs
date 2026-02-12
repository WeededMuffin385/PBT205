use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Router;
use axum::routing::{get, post};
use jsonwebtoken::{DecodingKey, TokenData};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use rand::distr::Alphanumeric;
use rand::RngExt;
use serde::Deserialize;
use serde_json::Value;
use tracing::info;
use url::Url;
use crate::context::Context;

/// https://developers.google.com/identity/protocols/oauth2/javascript-implicit-flow

pub const GOOGLE_CLIENT_ID: &str = "375985968237-rfs9subku539q8l40kuvc7v2g526bfuc.apps.googleusercontent.com";
const GOOGLE_CLIENT_SECRET: &str = "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";
const GOOGLE_REDIRECT_URL: &str = "https://localhost:8080/api/auth/google/callback";

pub fn router() -> Router<Context> {
	Router::new()
	 .route("/", get(google_auth))
	 .route("/callback", get(google_auth_callback))
}

async fn google_auth() -> Response {
	let state: String = rand::rng()
	 .sample_iter(Alphanumeric)
	 .take(32)
	 .map(char::from)
	 .collect();

	let mut auth_url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
	auth_url.query_pairs_mut()
	 .append_pair("client_id", GOOGLE_CLIENT_ID)
	 .append_pair("redirect_uri", GOOGLE_REDIRECT_URL)
	 .append_pair("response_type", "code")
	 .append_pair("scope", "openid")
	 .append_pair("state", &state)
	 .append_pair("access_type", "offline")
	 .append_pair("prompt", "consent");

	let cookie = format!(
		"oauth_state={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=300",
		state
	);

	let mut response = Redirect::temporary(auth_url.as_str()).into_response();
	response.headers_mut().append(
		header::SET_COOKIE,
		cookie.parse().unwrap()
	);

	response
}

#[derive(Deserialize)]
struct AuthQuery {
	code: String,
	state: String,
}

async fn google_auth_callback(
	State(state): State<Context>,
	Query(params): Query<AuthQuery>,
	headers: HeaderMap,
) -> Result<Response, StatusCode> {
	let cookie_header = headers
	 .get("cookie")
	 .and_then(|v| v.to_str().ok())
	 .unwrap_or("");

	let stored_state = cookie_header
	 .split("; ")
	 .find_map(|c| {
		 let mut parts = c.split('=');
		 if parts.next()? == "oauth_state" {
			 parts.next()
		 } else {
			 None
		 }
	 })
	 .unwrap_or("");

	if stored_state != params.state {
		panic!("OAuth state mismatch");
	}

	let response = get_google_token(params.code).await;
	let jwt = response.id_token;

	let header = jsonwebtoken::decode_header(&jwt).map_err(|_| StatusCode::UNAUTHORIZED)?;
	let kid = header.kid.ok_or(StatusCode::UNAUTHORIZED)?;
	let jwk = state.0.jwk_set.find(&kid).ok_or(StatusCode::UNAUTHORIZED)?;

	let claims: TokenData<GoogleClaims> = jsonwebtoken::decode(&jwt, &DecodingKey::from_jwk(jwk).unwrap(), &state.0.validation).map_err(|_| StatusCode::UNAUTHORIZED)?;
	info!("headers: {:#?}", claims.header);
	info!("claims: {:#?}", claims.claims);

	Ok(StatusCode::OK.into_response())

/*	let session_id = todo!("get session id by generating UUIDV4 and placing it in a Database with sub; sub is taken from id_token that is JWT");

	let cookie = format!(
		"session={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=900",
		session_id
	);

	let mut response = Redirect::temporary("/").into_response();
	response.headers_mut().append(
		header::SET_COOKIE,
		HeaderValue::from_str(&cookie).unwrap(),
	);

	Ok(response)*/
}

/// https://developers.google.com/identity/gsi/web/guides/verify-google-id-token
#[derive(Deserialize, Debug)]
struct GoogleClaims {
	pub email: Option<String>,
	pub email_verified: Option<bool>,

	pub sub: String,
	pub iss: String,
	pub iat: usize,
	pub exp: usize,
}

#[derive(Deserialize, Debug)]
struct GoogleTokenResponse {
	id_token: String,
}

async fn get_google_token(code: String) -> GoogleTokenResponse {
	let client = reqwest::Client::new();

	let response = client
	 .post("https://oauth2.googleapis.com/token")
	 .form(&[
		 ("client_id", GOOGLE_CLIENT_ID),
		 ("client_secret", GOOGLE_CLIENT_SECRET),
		 ("code", &code),
		 ("grant_type", "authorization_code"),
		 ("redirect_uri", GOOGLE_REDIRECT_URL),
	 ])
	 .send()
	 .await
	 .unwrap();

	let payload = response.text().await.unwrap();

	info!("payload: {payload}");

	serde_json::from_str(&payload).unwrap()
}