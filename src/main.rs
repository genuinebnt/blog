use blog::settings;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let settings = settings::Settings::new().unwrap();
    let connection_string = settings.database_settings.connection_string();
    let listener = TcpListener::bind(format!(
        "{}:{}",
        settings.application_host, settings.application_port
    ))
    .await
    .unwrap();
    blog::startup::run(listener).await;
}
