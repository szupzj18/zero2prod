
#[tokio::test]
async fn dummy_test() {
    assert_eq!(2 + 2, 4);
}

async fn spawn_app() -> Result<(), std::io::Error> {
    zero2prod::run().await
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn our app.");
    let client = reqwest::Client::new();

    let resp = client.get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}