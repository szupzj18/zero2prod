use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;
use zero2prod::telementary::{get_subscriber, init_subscriber};
use Result;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "zero2prod".into(), 
        "info".into(), 
        std::io::stdout
    );
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");
    let address = format!("{}:{}",configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await
}