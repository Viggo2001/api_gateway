use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/hello", get(hello))
}

async fn hello() -> &'static str {
    "Hello from Profile Management"
}