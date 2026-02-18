use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Account {
	pub account_id: i64,
	pub account_name: String,
}