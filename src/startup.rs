use axum::{serve, Router};
use tokio::net::TcpListener;

use crate::routes::posts::post_routes;

pub async fn run(listener: TcpListener) {
    let app = Router::new().merge(post_routes());
    serve(listener, app).await.unwrap();
}
