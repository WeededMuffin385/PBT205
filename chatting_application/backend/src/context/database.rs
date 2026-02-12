use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

pub struct Database {
	pub pool: PgPool,
}

impl Database {
	pub async fn new() -> Self {
		todo!("add password and host");

		let username = "postgres";
		let password = r#"TODO_PASSWORD"#;

		let options = PgConnectOptions::new()
		 .username(username)
		 .password(password)
		 .database("postgres")
		 .host("TODO_HOST");

		let pool = PgPoolOptions::new()
		 .max_connections(2)
		 .connect_with(options).await.unwrap();
		
		Self { pool }
	}
}