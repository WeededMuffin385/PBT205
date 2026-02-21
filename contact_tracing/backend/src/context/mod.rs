pub mod broker;
mod database;

use std::sync::Arc;
use crate::context::broker::Broker;
use crate::context::database::Database;

#[derive(Clone)]
pub struct Context(pub Arc<InnerContext>);

impl Context {
    pub async fn new() -> Self {
        Self(Arc::new(InnerContext::new().await))
    }
}

pub struct InnerContext {
    pub broker: Broker,
    pub database: Database,
}

impl InnerContext {
    pub async fn new() -> Self {
        let broker = Broker::new().await;
        let database = Database::new().await;

        Self {
            broker,
            database,
        }
    }
}