use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Account {
	pub account_id: i64,
	pub account_name: String,
}