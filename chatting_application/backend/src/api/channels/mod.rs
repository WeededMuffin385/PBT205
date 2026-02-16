use crate::authentication_extractor::Authentication;
use crate::context::Context;
use crate::message::Message;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Response, Sse};
use axum::routing::{get, post};
use axum::{middleware, Json, Router};
use axum_extra::extract::CookieJar;
use axum_extra::TypedHeader;
use futures_util::{stream, Stream, StreamExt as _};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueBindOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::BasicProperties;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::time::Duration;
use chrono::{DateTime, Utc};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tracing::error;
use uuid::Uuid;
use crate::account::Account;

pub fn router(context: Context) -> Router<Context> {
	Router::new()
	 .route("/", get(get_channels))
	 .route("/{id}", get(get_messages))
	 .route("/{id}", post(post_message))
	 .route("/{id}/callback", get(sse_handler))
/*	 .layer(middleware::from_fn_with_state(context, async |
		 State(state): State<Context>,
		 Path(id): Path<Uuid>,
		 request: Request,
		 next: Next,
	 | -> Response {
		 next.run(request).await
	 }))*/
}

async fn get_channels(
	State(state): State<Context>,
) -> Response {
	todo!()
}

async fn get_messages(
	State(state): State<Context>,
	Path(id): Path<Uuid>,
) -> Response {
	todo!()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMessageRequest {
	content: String,
}

async fn post_message(
	State(state): State<Context>,
	authentication: Authentication,
	Path(id): Path<Uuid>,
	Json(request): Json<PostMessageRequest>,
) -> Response {
	let Account{account_id, account_name} = authentication.account.clone();

	let message = Message{
		account_name,
		account_id,

		content: request.content,
		created_at: Utc::now(),
	};

	const MESSAGE_BUFFER_SIZE: usize = 2usize.pow(13);
	let message = postcard::to_vec::<_, MESSAGE_BUFFER_SIZE>(&message).unwrap();

	state.0.broker.channel.basic_publish(
		"events".into(),
		id.to_string().into(),
		BasicPublishOptions::default(),
		&message,
		BasicProperties::default().with_delivery_mode(1),
	).await.unwrap().await.unwrap();

	StatusCode::CREATED.into_response()
}

async fn sse_handler(
	State(state): State<Context>,
	Path(id): Path<Uuid>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
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
		"events".into(),
		id.to_string().into(),
		QueueBindOptions::default(),
		FieldTable::default(),
	).await.unwrap();

	let consumer = state.0.broker.channel.basic_consume(
		queue.name().clone(),
		"".into(),
		BasicConsumeOptions::default(),
		FieldTable::default()
	).await.unwrap();

	let stream = consumer.filter_map(async |delivery| {
		match delivery {
			Ok(delivery) => {
				delivery.ack(BasicAckOptions::default()).await.unwrap();
				let message: Message = postcard::from_bytes(&delivery.data).unwrap();
				let json = serde_json::to_string(&message).unwrap();

				Some(Ok(Event::default().data(json)))
			}
			Err(err) => {
				error!("error: {err:?}");
				None
			}
		}
	});

	Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}