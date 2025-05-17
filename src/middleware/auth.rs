use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn auth(req: Request<Body>, next: Next) -> Response {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token) = auth_header.to_str() {
            if token.starts_with("Bearer ") {
                let token = &token[7..];
                if decode::<Claims>(
                    token,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::new(Algorithm::HS256),
                ).is_ok() {
                    return next.run(req).await;
                }
            }
        }
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}
