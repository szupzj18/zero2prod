use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;
use Result;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("failed to read configuration.");
    startup::run(
        TcpListener::bind("127.0.0.1:8080")?, 
        configuration)?.await
}