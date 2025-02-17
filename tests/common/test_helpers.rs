use blog::{settings, startup::run};
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    let url = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:0", url))
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let settings = settings::Settings::new().expect("Failed to load settings");

    tokio::spawn(async move {
        run(listener).await;
    });

    TestApp {
        address: format!("{}:{}", url, port),
    }
}
