use std::net::TcpListener;
use actix_web::{dev::Server, web::{self}, HttpServer, App};
use sqlx::{PgConnection, PgPool};

use crate::{health_check, index, subscribe};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new( move || 
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/", web::get().to(index)) // default route endpoint
            .route("/{name}", web::get().to(index))
            .app_data(db_pool.clone())
    )
    .listen(listener)?
    .run();
    Ok(server)
}