use actix_web::{ HttpServer, web, App };

#[tokio::main]
async fn main() -> Result<(), Error> {
    HttpServer::new( || 
        App::new()
            .route("/", web::get().to(index)) // default route endpoint
    ).bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() {
    // WIP: indexing
    "Hello, world!"
}