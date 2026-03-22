use std::str::FromStr;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use uuid::Uuid;
use serde::Deserialize;
use common::account::Account;
use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new()
        .route("/", post(post_auth))
        .route("/check", get(get_auth_check))
}

async fn get_auth_check(
    State(state): State<Context>,
    jar: CookieJar,
) -> Response {
    if let Some(session_id) = jar.get("session_id") {
        let session_id = Uuid::from_str(session_id.value()).unwrap();

        if let Some(account) = state.0.database.get_account_by_session_id(session_id).await {
            Json(account).into_response()
        } else {
            StatusCode::UNAUTHORIZED.into_response()
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

#[derive(Deserialize)]
struct PostAuthRequest {
    account_name: String,
}

async fn post_auth(
    State(state): State<Context>,
    jar: CookieJar,
    Json(request): Json<PostAuthRequest>,
) -> Response {
    let account_name = request.account_name;
    let account_id = state.0.database.add_account(&account_name).await;
    let session_id = state.0.database.add_account_session_id(account_id).await;

    let jar = jar.add(Cookie::build(("session_id", session_id.to_string()))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
    );

    let jar = jar.add(Cookie::build(("account_id", account_id.to_string()))
        .path("/")
        .secure(false)
        .http_only(false)
        .same_site(SameSite::Lax)
    );

    let message = Account {
        balance: 0,
        account_id,
        account_name,
    };

    const MESSAGE_BUFFER_SIZE: usize = 2usize.pow(8);
    let message = postcard::to_vec::<_, MESSAGE_BUFFER_SIZE>(&message).unwrap();

    state.0.broker.channel.basic_publish(
        POSITION_EXCHANGE.into(),
        account_id.to_string().into(),
        BasicPublishOptions::default(),
        &message,
        BasicProperties::default().with_delivery_mode(1),
    ).await.unwrap().await.unwrap();

    (jar, Json(PostAuthResponse{ x, y })).into_response()
}