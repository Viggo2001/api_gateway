use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::collections::HashSet;

pub async fn allow_deny(req: Request<Body>, next: Next) -> Response {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("127.0.0.1");

    // Create a deny list
    let deny_list: HashSet<&str> = HashSet::from(["192.168.1.5", "10.0.0.2"]);

    if deny_list.contains(ip) {
        return (StatusCode::FORBIDDEN, "Access denied").into_response();
    }

    next.run(req).await
}
