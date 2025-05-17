use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::{Duration, Instant};
use dashmap::DashMap;
use once_cell::sync::Lazy;

static LIMITS: Lazy<DashMap<String, Instant>> = Lazy::new(DashMap::new);

pub async fn rate_limit(req: Request<Body>, next: Next) -> Response {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("127.0.0.1")
        .to_string();

    let now = Instant::now();
    let mut entry = LIMITS.entry(ip.clone()).or_insert(now);

    if now.duration_since(*entry) < Duration::from_secs(1) {
        return (StatusCode::TOO_MANY_REQUESTS, "Too Many Requests").into_response();
    }

    *entry = now;

    next.run(req).await
}
