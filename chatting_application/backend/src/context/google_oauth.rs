use serde::Deserialize;
use tracing::info;
use url::Url;

pub const GOOGLE_CLIENT_ID: &str = "375985968237-rfs9subku539q8l40kuvc7v2g526bfuc.apps.googleusercontent.com";
const GOOGLE_CLIENT_SECRET: &str = "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";
const GOOGLE_REDIRECT_URL: &str = "https://localhost:8080/api/auth/google/callback";

pub struct GoogleOauth {
	client: reqwest::Client,
}

impl GoogleOauth {
	pub fn new() -> Self {
		Self {
			client: reqwest::Client::new(),
		}
	}

	pub async fn get_auth_url(
		&self,
		state: &str,
	) -> Url {
		let mut auth_url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
		auth_url.query_pairs_mut()
		 .append_pair("client_id", GOOGLE_CLIENT_ID)
		 .append_pair("redirect_uri", GOOGLE_REDIRECT_URL)
		 .append_pair("response_type", "code")
		 .append_pair("scope", "openid profile")
		 .append_pair("state", state)
		 .append_pair("access_type", "offline")
		 .append_pair("prompt", "consent");
		auth_url
	}

	pub async fn exchange_code(
		&self,
		code: String,
	) -> GoogleOauthResponse {
		let response = self.client
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
		serde_json::from_str(&payload).expect(format!("failed to deserialize: {payload}").as_str())
	}
}

#[derive(Deserialize)]
pub struct GoogleOauthResponse {
	pub access_token: String,            // A token that can be sent to a Google API.
	pub expires_in: i64,                 // The remaining lifetime of the access token in seconds.
	pub id_token: String,                // A JWT that contains identity information about the user that is digitally signed by Google.
	pub scope: String,                   // The scopes of access granted by the access_token expressed as a list of space-delimited, case-sensitive strings.
	pub token_type: String,              // Identifies the type of token returned. At this time, this field always has the value Bearer.
	pub refresh_token: Option<String>,   // (optional)
}