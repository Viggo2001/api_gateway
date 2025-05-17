use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/test", get(test))
}

async fn hello() -> &'static str {
    "Hello from Bookings"
}

async fn test() -> &'static str {
    "Testing 123"
}