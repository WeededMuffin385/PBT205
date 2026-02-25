use std::collections::HashMap;
use std::sync::Arc;
use lapin::{Channel, Connection, ConnectionProperties, Consumer};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tracing::{error, info};
use backend::common::account::Account;
use backend::common::{POSITION_EXCHANGE, QUERY_REQUEST_EXCHANGE};
use futures_util::StreamExt;
use tokio::sync::{mpsc, Mutex};
use tracing_subscriber::EnvFilter;
use backend::common::query::{QueryRequest, QueryResponse};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Hello, World!");

    let rabbitmq = Connection::connect(
        "amqp://admin:secret@rabbitmq:5672",
        ConnectionProperties::default(),
    ).await.unwrap();
    let channel = rabbitmq.create_channel().await.unwrap();

    let username = "admin";
    let password = "secret";
    let options = PgConnectOptions::new()
        .username(username)
        .password(password)
        .database("postgres")
        .host("postgres");

    let postgres = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options).await.unwrap();

    let mut conn = postgres.acquire().await.unwrap();
    let accounts = sqlx::query_as!(Account, "SELECT * FROM accounts").fetch_all(&mut *conn).await.unwrap();
    let mut accounts: HashMap<i64, Account> = accounts.into_iter().map(|account|(account.account_id, account)).collect();

    let queue = channel.queue_declare(
        "".into(),
        QueueDeclareOptions {
            exclusive: true,
            auto_delete: true,
            durable: false,
            ..Default::default()
        },
        FieldTable::default()
    ).await.unwrap();

    channel.queue_bind(
        queue.name().clone(),
        POSITION_EXCHANGE.into(),
        "#".into(),
        QueueBindOptions::default(),
        FieldTable::default(),
    ).await.unwrap();

    let mut consumer = channel.basic_consume(
        queue.name().clone(),
        "".into(),
        BasicConsumeOptions::default(),
        FieldTable::default()
    ).await.unwrap();

    let collisions = Arc::new(Mutex::new(Vec::new()));

    run_responder(&channel, collisions.clone()).await;

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.unwrap();
        delivery.ack(BasicAckOptions::default()).await.unwrap();
        let message: Account = postcard::from_bytes(&delivery.data).unwrap();

        info!("Message received: {:?}", message);

        for (account_id, account) in accounts.iter().filter(|(id, _)| **id != message.account_id) {
            if (account.x == message.x) && (account.y == message.y) {
                collisions.lock().await.push((account_id.clone(), message.account_id));
            }
        }

        accounts.insert(message.account_id, message);
    }
}

async fn run_responder(channel: &Channel, collisions: Arc<Mutex<Vec<(i64, i64)>>>) {
    let mut query_request_consumer = create_query_request_consumer(channel).await;

    tokio::spawn(async move {
        while let Some(delivery) = query_request_consumer.next().await {
            let delivery = delivery.unwrap();
            delivery.ack(BasicAckOptions::default()).await.unwrap();
            let message: QueryRequest = postcard::from_bytes(&delivery.data).unwrap();

            let collisions: Vec<i64> = collisions.lock().await.iter().filter_map(|&(a, b)|{
                if a == message.account_id {
                    Some(b)
                } else if b == message.account_id {
                    Some(a)
                } else {
                    None
                }
            }).collect();
        }
    });
}

async fn create_query_request_consumer(channel: &Channel) -> Consumer {
    let queue = channel.queue_declare(
        "".into(),
        QueueDeclareOptions {
            exclusive: true,
            auto_delete: true,
            durable: false,
            ..Default::default()
        },
        FieldTable::default()
    ).await.unwrap();

    channel.queue_bind(
        queue.name().clone(),
        QUERY_REQUEST_EXCHANGE.into(),
        "#".into(),
        QueueBindOptions::default(),
        FieldTable::default(),
    ).await.unwrap();

    let consumer = channel.basic_consume(
        queue.name().clone(),
        "".into(),
        BasicConsumeOptions::default(),
        FieldTable::default()
    ).await.unwrap();

    consumer
}
