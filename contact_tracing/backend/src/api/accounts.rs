use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use futures_util::StreamExt;
use lapin::BasicProperties;
use lapin::options::{BasicConsumeOptions, BasicPublishOptions, QueueBindOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use common::broker::{POSITION_EXCHANGE, QUERY_REQUEST_EXCHANGE, QUERY_RESPONSE_EXCHANGE};
use common::query::{QueryRequest, QueryResponse};
use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new()
        .route("/", get(get_accounts))
        .route("/{account_id}/contacts", get(get_account_contacts))
}

async fn get_accounts(
    State(state): State<Context>
) -> Response {
    let accounts = state.0.database.get_accounts().await;
    Json(accounts).into_response()
}

async fn get_account_contacts(
    State(state): State<Context>,
    Path(account_id): Path<i64>
) -> Response {
    let queue = state.0.broker.channel.queue_declare(
        "".into(),
        QueueDeclareOptions {
            exclusive: true,
            auto_delete: true,
            durable: false,
            ..Default::default()
        },
        FieldTable::default()
    ).await.unwrap();

    state.0.broker.channel.queue_bind(
        queue.name().clone(),
        QUERY_RESPONSE_EXCHANGE.into(),
        account_id.to_string().into(),
        QueueBindOptions::default(),
        FieldTable::default(),
    ).await.unwrap();

    let mut consumer = state.0.broker.channel.basic_consume(
        queue.name().clone(),
        "".into(),
        BasicConsumeOptions::default(),
        FieldTable::default()
    ).await.unwrap();
    
    let request = QueryRequest{
        account_id
    };
    const MESSAGE_BUFFER_SIZE: usize = 2usize.pow(16);
    let message = postcard::to_vec::<_, MESSAGE_BUFFER_SIZE>(&request).unwrap();

    state.0.broker.channel.basic_publish(
        QUERY_REQUEST_EXCHANGE.into(),
        account_id.to_string().into(),
        BasicPublishOptions::default(),
        &message,
        BasicProperties::default().with_delivery_mode(1),
    ).await.unwrap().await.unwrap();
    
    let delivery = consumer.next().await.unwrap().unwrap();
    let response: QueryResponse = postcard::from_bytes(&delivery.data).unwrap();
    
    Json(response.collided_accounts).into_response()
}