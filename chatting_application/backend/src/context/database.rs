use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tracing::info;

pub struct Database {
	pub pool: PgPool,
}

impl Database {
	pub async fn new() -> Self {
		let username = "admin";
		let password = "secret";

		let options = PgConnectOptions::new()
		 .username(username)
		 .password(password)
		 .database("postgres")
		 .host("database");

		let pool = PgPoolOptions::new()
		 .max_connections(2)
		 .connect_with(options).await.unwrap();

		info!("Connected to Postgres");
		
		Self { pool }
	}
}