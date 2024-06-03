use actix_web::{ web, App, HttpRequest, HttpResponse, HttpServer, Responder };

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

async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    println!("recieved from req uri: {}, method {}", req.path(), req.method());
    format!("Hello, {}!", &name)
}

async fn health_check () {
    todo!()
}