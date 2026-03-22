use tracing::info;
use tracing_subscriber::EnvFilter;

mod app;
mod api;
mod context;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Hello, World!");
}
