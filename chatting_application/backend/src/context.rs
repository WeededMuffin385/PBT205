use std::sync::Arc;
use tokio::sync::broadcast;
use crate::message::Message;

#[derive(Clone)]
pub struct Context(pub Arc<InnerContext>);

impl Context {
	pub fn new() -> Self {
		Self(Arc::new(InnerContext::new()))
	}
}

pub struct InnerContext {
	pub messages_broadcaster: broadcast::Sender<Message>,
}

impl InnerContext {
	pub fn new() -> Self {
		Self {
			messages_broadcaster: broadcast::Sender::new(1024),
		}
	}
}