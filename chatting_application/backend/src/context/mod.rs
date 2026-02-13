pub mod database;
mod broker;

use std::sync::Arc;
use dashmap::DashMap;
use jsonwebtoken::{Algorithm, Validation};
use jsonwebtoken::jwk::JwkSet;
use tokio::sync::broadcast;
use uuid::Uuid;
use crate::api::auth::google::GOOGLE_CLIENT_ID;
use crate::context::broker::Broker;
use crate::context::database::Database;
use crate::message::Message;

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

		let database = Database::new().await;
		let broker = Broker::new().await;

		Self {
			messages_broadcast: Default::default(),

			jwk_set,
			database,
			validation,
			broker,
		}
	}
}