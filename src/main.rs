use std::net::TcpListener;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;
use Result;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await
}