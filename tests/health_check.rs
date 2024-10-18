use std::net::TcpListener;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{configuration::{get_configuration, Settings}, startup};

struct TestApp {
    address: String,
    pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address.");
    let port = listener.local_addr().unwrap().port();
    let mut configuration = get_configuration().expect("failed to load config.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_database(configuration).await;
    let server: actix_web::dev::Server = startup::run(listener, pool.clone()).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        pool: pool
    }
}

async fn configure_database(configuration: Settings) -> PgPool {
    // create a new db for testing.
    let connection = PgPool::connect(&configuration.database.connection_string_without_db())
        .await
        .expect("failed to connect to Postgres.");

    connection.execute(format!(r#"CREATE DATABASE "{}";"#, configuration.database.database_name).as_str())
        .await
        .expect("failed to create db.");
    // connect old db. migration.
    let conncection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&conncection_pool)
        .await
        .expect("failed to migrate db.");

    conncection_pool
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let resp = client.get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

// subscribe service test.
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_from_data() {
    let app = spawn_app().await;
    let configuration = get_configuration().expect("failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string) // connect testable db.
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to post request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to post request.");

        assert_eq!(
            400, 
            response.status().as_u16(),
            "The api didn't failed with the 400 bad request when the payload was {}.", 
            error_message
        );
    }
    
}