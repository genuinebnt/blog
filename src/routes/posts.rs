use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub async fn get_posts() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn create_post() -> impl IntoResponse {
    StatusCode::OK
}

pub fn post_routes() -> Router {
    Router::new().route("/posts", get(get_posts).post(create_post))
}
