use actix_web::{ web, App, HttpResponse, HttpServer, Responder };

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new( || 
        App::new()
            .route("/", web::get().to(index)) // default route endpoint
            .route("/{name}", web::get().to(index))
    ).bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    // WIP: indexing
    HttpResponse::Ok().finish()
}