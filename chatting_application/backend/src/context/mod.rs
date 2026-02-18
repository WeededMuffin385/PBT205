mod database;
mod broker;
mod google_oauth;

use crate::context::broker::Broker;
use crate::context::database::Database;
use crate::context::google_oauth::{GoogleOauth, GOOGLE_CLIENT_ID};
use crate::types::message::Message;
use dashmap::DashMap;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, Validation};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone)]
pub struct Context(pub Arc<InnerContext>);

impl Context {
	pub async fn new() -> Self {
		Self(Arc::new(InnerContext::new().await))
	}
}

pub struct InnerContext {
	pub messages_broadcast: DashMap<Uuid, broadcast::Sender<Message>>,

	pub jwk_set: JwkSet,
	pub validation: Validation,

	pub google_oauth: GoogleOauth,
	pub database: Database,
	pub broker: Broker,
}

impl InnerContext {
	pub async fn new() -> Self {
		// developers.google.com/identity/gsi/web/guides/verify-google-id-token
		let jwk_set: JwkSet = reqwest::get("https://www.googleapis.com/oauth2/v3/certs").await.unwrap().json().await.unwrap();
		let mut validation = Validation::new(Algorithm::RS256);
		validation.set_issuer(&["https://accounts.google.com"]);
		validation.set_audience(&[GOOGLE_CLIENT_ID]);
		validation.validate_exp = true;
		validation.validate_nbf = true;
		validation.validate_aud = true;

		let google_oauth = GoogleOauth::new();
		let database = Database::new().await;
		let broker = Broker::new().await;

		Self {
			messages_broadcast: Default::default(),

			jwk_set,
			validation,

			broker,
			database,
			google_oauth,
		}
	}
}