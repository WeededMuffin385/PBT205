use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tracing::info;
use uuid::Uuid;
use crate::types::account::Account;

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

    pub async fn add_account(&self, account_name: String) -> i64 {
        let mut conn = self.pool.acquire().await.unwrap();

        sqlx::query_scalar!("INSERT INTO accounts (account_name) VALUES ($1) RETURNING account_id", &account_name).fetch_one(&mut *conn).await.unwrap()
    }

    pub async fn add_account_session_id(&self, account_id: i64) -> Uuid {
        let mut conn = self.pool.acquire().await.unwrap();

        sqlx::query_scalar!("INSERT INTO sessions (account_id) VALUES ($1) RETURNING session_id", account_id).fetch_one(&mut *conn).await.unwrap()
    }

    pub async fn get_account_by_session_id(&self, session_id: Uuid) -> Option<Account> {
        let mut conn = self.pool.acquire().await.unwrap();

        sqlx::query_as!(Account, "
            SELECT accounts.account_id, account_name
            FROM sessions
            JOIN accounts ON accounts.account_id = sessions.account_id
            WHERE sessions.session_id = $1
        ", &session_id).fetch_optional(&mut *conn).await.unwrap()
    }
}