use std::sync::{Arc, Mutex};

use axum::{serve, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::routes::{health_check::health_check_routes, posts::post_routes};

#[derive(Debug, Clone)]
pub struct AppState {
    pool: PgPool,
}

pub async fn run(listener: TcpListener, pool: PgPool) {
    let state = Arc::new(Mutex::new(AppState { pool }));

    let app = Router::new()
        .with_state(state)
        .merge(post_routes())
        .merge(health_check_routes());

    serve(listener, app).await.unwrap();
}
