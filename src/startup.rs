use std::net::TcpListener;
use actix_web::{dev::Server, web::{self}, HttpServer, App};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::routes::{health_check, subscribe, index};

#[tracing::instrument(
    name = "Starting server",
    fields(
        addr = %listener.local_addr().unwrap()
    )
)]
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new( move || 
        App::new()
            .wrap(TracingLogger::default())
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