use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub fn health_check_routes() -> Router {
    Router::new().route("/health_check", get(health_check))
}