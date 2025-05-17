use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct MyQuery {
    #[validate(range(min = 1, max = 100))]
    limit: u32,
}

async fn handler(Query(params): Query<MyQuery>) -> impl IntoResponse {
    if let Err(e) = params.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{:?}", e)));
    }

    Ok("Valid params")
}
