use sqlx::{FromRow, PgPool};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tracing::info;
use uuid::Uuid;
use crate::account::Account;

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

	pub async fn add_session_id(
		&self,
		session_id: Uuid,
		account_id: i64,
	) {
		let mut conn = self.pool.acquire().await.unwrap();
		let session_id = session_id.to_string();

		sqlx::query!("INSERT INTO sessions (session_id, account_id) VALUES ($1, $2)", &session_id, account_id).execute(&mut *conn).await.unwrap();
	}

	pub async fn get_or_init_account_id_with_google_account_id(&self, google_account_id: String, google_account_name: String) -> i64 {
		let mut conn = self.pool.acquire().await.unwrap();


		sqlx::query_scalar!(r#"
			WITH existing AS (
			    SELECT account_id
			    FROM google_accounts
			    WHERE google_account_id = $1
			),
			insert_account AS (
			    INSERT INTO accounts (account_name)
				SELECT $2
				WHERE NOT EXISTS (SELECT 1 FROM existing)
				RETURNING account_id
			),
			insert_google_account AS (
				INSERT INTO google_accounts (account_id, google_account_id)
				SELECT account_id, $1
				FROM insert_account
				RETURNING account_id
			)
			SELECT account_id as "account_id!"
			FROM (
				SELECT account_id FROM existing
				UNION ALL
				SELECT account_id FROM insert_google_account
			) s
		"#, google_account_id, google_account_name).fetch_one(&mut *conn).await.unwrap()
	}

	pub async fn get_account_by_session(
		&self,
		session: Uuid,
	) -> Option<Account> {
		let mut conn = self.pool.acquire().await.unwrap();

		let session_id = session.to_string();

		sqlx::query_as!(Account, r#"
			SELECT accounts.account_id, account_name
			FROM sessions
			JOIN accounts ON accounts.account_id = sessions.account_id
			WHERE session_id = $1
			AND expires_at > now()
		"#, &session_id).fetch_optional(&mut *conn).await.unwrap()
	}
}

