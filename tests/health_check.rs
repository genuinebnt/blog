mod common;

#[tokio::test]
async fn health_check_works() {
    let app = common::test_helpers::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client.get(format!("http://{}/health_check", &app.address))
        .send()
        .await.unwrap();

    assert_eq!(response.status(), 200);
}