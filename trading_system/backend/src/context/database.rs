use uuid::Uuid;
use common::account::Account;
use common::database::Database;

pub trait DatabaseBackendExt {
    async fn add_account(&self, account_name: &str) -> i64;
    async fn add_account_session_id(&self, account_id: i64) -> Uuid;
    async fn get_account_by_session_id(&self, session_id: Uuid) -> Option<Account>;
}

impl DatabaseBackendExt for Database {
    async fn get_account_by_session_id(&self, session_id: Uuid) -> Option<Account> {
        let mut conn = self.pool.acquire().await.unwrap();

        sqlx::query_as!(Account, "
            SELECT accounts.account_id, account_name, balance
            FROM sessions
            JOIN accounts ON accounts.account_id = sessions.account_id
            WHERE sessions.session_id = $1
        ", &session_id).fetch_optional(&mut *conn).await.unwrap()
    }

    async fn add_account(&self, account_name: &str) -> i64 {
        let mut conn = self.pool.acquire().await.unwrap();

        sqlx::query_scalar!("INSERT INTO accounts (account_name) VALUES ($1) RETURNING account_id", account_name).fetch_one(&mut *conn).await.unwrap()
    }

    async fn add_account_session_id(&self, account_id: i64) -> Uuid {
        let mut conn = self.pool.acquire().await.unwrap();

        sqlx::query_scalar!("INSERT INTO sessions (account_id) VALUES ($1) RETURNING session_id", account_id).fetch_one(&mut *conn).await.unwrap()
    }
}