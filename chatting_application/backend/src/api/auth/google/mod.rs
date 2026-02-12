use axum::extract::Query;
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Router;
use axum::routing::{get, post};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use rand::distr::Alphanumeric;
use rand::RngExt;
use serde::Deserialize;
use crate::context::Context;

const GOOGLE_CLIENT_ID: &str = "375985968237-rfs9subku539q8l40kuvc7v2g526bfuc.apps.googleusercontent.com";
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

	let redirect_url = utf8_percent_encode(&GOOGLE_REDIRECT_URL, NON_ALPHANUMERIC);
	let scope = utf8_percent_encode("openid", NON_ALPHANUMERIC);

	let auth_url = format!(
		"https://accounts.google.com/o/oauth2/v2/auth?client_id={GOOGLE_CLIENT_ID}&redirect_uri={redirect_url}&response_type=code&scope={scope}&state={state}&access_type=offline&prompt=consent"
	);

	let cookie = format!(
		"oauth_state={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=300",
		state
	);

	let mut response = Redirect::temporary(&auth_url).into_response();
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
	Query(params): Query<AuthQuery>,
	headers: HeaderMap,
) -> Response {
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

	let access_token = exchange_code(params.code).await;

	let cookie = format!(
		"access_token={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=900",
		access_token
	);

	let mut response = Redirect::temporary("/").into_response();
	response.headers_mut().append(
		header::SET_COOKIE,
		HeaderValue::from_str(&cookie).unwrap(),
	);

	response
}

async fn exchange_code(code: String) -> String {
	let client = reqwest::Client::new();

	let res = client
	 .post("https://oauth2.googleapis.com/token")
	 .form(&[
		 ("client_id", GOOGLE_CLIENT_ID),
		 ("client_secret", "YOUR_SECRET"),
		 ("code", &code),
		 ("grant_type", "authorization_code"),
		 ("redirect_uri", GOOGLE_REDIRECT_URL),
	 ])
	 .send()
	 .await
	 .unwrap();

	let json: serde_json::Value = res.json().await.unwrap();

	json["access_token"]
	 .as_str()
	 .unwrap()
	 .to_string()
}