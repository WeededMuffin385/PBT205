// https://github.com/tokio-rs/axum/blob/main/examples/tls-rustls/src/main.rs

use crate::context::Context;
use axum::ServiceExt;
use axum_server::tls_rustls::RustlsConfig;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::time::sleep;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod context;
mod app;
mod api;
mod authentication_extractor;
mod types;

#[tokio::main]
async fn main() {
    jsonwebtoken::crypto::aws_lc::DEFAULT_PROVIDER.install_default().unwrap();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    sleep(Duration::from_secs(10)).await;
    info!("Hello, World!");

    let config = RustlsConfig::from_pem_file("assets/localhost.pem", "assets/localhost-key.pem").await.unwrap();
    let addr = "0.0.0.0:8080".to_socket_addrs().unwrap().next().unwrap();
    let context = Context::new().await;

    let app = app::router(context.clone())
     .with_state(context)
     .layer(TraceLayer::new_for_http());

    axum_server::bind_rustls(addr, config).serve(app.into_make_service()).await.unwrap()
}
