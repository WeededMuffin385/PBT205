// https://github.com/tokio-rs/axum/blob/main/examples/tls-rustls/src/main.rs

use std::net::{SocketAddr, ToSocketAddrs};
use axum::ServiceExt;
use axum_server::tls_rustls::RustlsConfig;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::context::Context;

mod context;
mod app;
mod api;
mod authentication_extractor;
mod message;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
     .with_env_filter(EnvFilter::from_default_env())
     .init();

    info!("Hello, World!");

    let config = RustlsConfig::from_pem_file("assets/localhost.pem", "assets/localhost-key.pem").await.unwrap();
    let addr = "localhost:8080".to_socket_addrs().unwrap().next().unwrap();

    let app = app::router()
     .with_state(Context::new())
     .layer(TraceLayer::new_for_http());

    axum_server::bind_rustls(addr, config).serve(app.into_make_service()).await.unwrap()
}
