use axum::{Router, middleware as mid, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::routes;
use crate::middleware;

pub async fn run() {
    let protected_routes = Router::new()
        .nest("/bookings", routes::bookings::router())
        .nest("/feedback", routes::feedback::router())
        .nest("/profile_management", routes::profile_management::router())
        .nest("/session_management", routes::session_management::router())
        .nest("/social", routes::social::router())
        .layer(mid::from_fn(middleware::rate_limiter::rate_limit))
        .layer(mid::from_fn(middleware::auth::auth))
        .layer(mid::from_fn(middleware::allow_deny::allow_deny));

    let app = Router::new()
        .nest("/auth", routes::auth_management::router()) // ✅ Unprotected login route
        .merge(protected_routes);              // ✅ All others protected

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Gateway running at http://{}", addr);

    serve(listener, app).await.unwrap();
}