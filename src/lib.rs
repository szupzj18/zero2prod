use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || 
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(index)) // default route endpoint
            .route("/{name}", web::get().to(index))
    )
    .listen(listener)?
    .run();
    Ok(server)
}

async fn health_check () -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    println!("recieved from req uri: {}, method {}", req.path(), req.method());
    format!("Hello, {}!", &name)
}