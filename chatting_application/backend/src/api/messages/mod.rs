use crate::context::Context;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Response, Sse};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::{stream, Stream};
use std::convert::Infallible;
use std::time::Duration;
use axum::http::StatusCode;
use tokio_stream::StreamExt as _;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;
use tracing::error;
use crate::message::Message;

pub fn router() -> Router<Context> {
	Router::new()
	 .route("/", get(get_messages))
	 .route("/", post(post_message))
	 .route("/callback", get(sse_handler))
}

async fn get_messages(
	State(state): State<Context>,
) -> Response {
	todo!()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMessageRequest {
	content: String,
}

async fn post_message(
	State(state): State<Context>,
	Json(request): Json<PostMessageRequest>,
) -> Response {
	state.0.messages_broadcaster.send(Message{
		content: request.content,
		sender: "anonymous".to_string(),
		time: "todo!".to_string(),
		date: "todo!".to_string(),
	}).unwrap();

	StatusCode::CREATED.into_response()
}

async fn sse_handler(
	State(state): State<Context>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
	let receiver = state.0.messages_broadcaster.subscribe();

	let stream = BroadcastStream::new(receiver).filter_map(|result|  {
		match result {
			Ok(message) => {
				let json = serde_json::to_string(&message).unwrap();
				Some(Ok(Event::default().data(json)))
			},
			Err(error) => {
				error!("{:?}", error);
				None
			},
		}
	});

	Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}