use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
	pub account_name: String,
	pub account_id: i64,

	pub content: String,
	pub created_at: chrono::DateTime<chrono::Utc>,
}