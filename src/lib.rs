use actix_web::{HttpResponse, HttpServer, App, web, Responder, HttpRequest};

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new( || 
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(index)) // default route endpoint
            .route("/{name}", web::get().to(index))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn health_check () -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    println!("recieved from req uri: {}, method {}", req.path(), req.method());
    format!("Hello, {}!", &name)
}