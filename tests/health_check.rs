use std::net::TcpListener;

fn spawn_app() -> String{
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let server = web_microservice::startup::run(listener).expect("Unable to start server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}