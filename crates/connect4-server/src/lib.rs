use axum::{Router, routing::get};

pub fn app() -> Router {
     Router::new().route("/health", get(health))
}

async fn health() -> &'static str {
    "ok"
}