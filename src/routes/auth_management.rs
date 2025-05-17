use axum::{
    routing::post,
    Json, Router,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
}

async fn login_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    if payload.username == "admin" && payload.password == "admin123" {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .unwrap()
            .timestamp() as usize;

        let claims = Claims {
            sub: payload.username,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        ).unwrap();

        // Wrap into a full response
        (StatusCode::OK, Json(LoginResponse { token })).into_response()
    } else {
        // Ensure this also becomes a full response
        (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
    }
}


pub fn router() -> Router {
    Router::new().route("/login", post(login_handler))
}
