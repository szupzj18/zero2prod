use std::net::TcpListener;
use actix_web::{dev::Server, web::{self}, HttpServer, App};

use crate::{configuration::Settings, health_check, index, subscribe};

pub fn run(
    listener: TcpListener,
    configuration: Settings
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || 
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/", web::get().to(index)) // default route endpoint
            .route("/{name}", web::get().to(index))
    )
    .listen(listener)?
    .run();
    Ok(server)
}