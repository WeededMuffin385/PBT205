use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
	pub content: String,
	pub sender: String,
	pub time: String,
	pub date: String,
}