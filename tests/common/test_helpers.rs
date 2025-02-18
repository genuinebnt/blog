use blog::{settings, startup::run};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct TestApp {
    pub address: String
}

pub async fn spawn_app() -> TestApp {
    let url = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:0", url))
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let settings = settings::Settings::new().expect("Failed to load settings");
    let connection_string = settings.database_settings.connection_string();
    println!("Connection string: {}", connection_string);
    let pool = PgPool::connect(&connection_string).await.unwrap();

    tokio::spawn(async move {
        run(listener, pool).await;
    });

    TestApp {
        address: format!("{}:{}", url, port)
    }
}
