pub mod routes;
pub mod configuration;
pub mod startup;
pub mod telementary;
use actix_web::web;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn index(form: web::Form<FormData>) -> String {
    format!("Welcome, {} -- email: {}", form.name,form.email)
}

// async fn index(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     println!("recieved from req uri: {}, method {}", req.path(), req.method());
//     format!("Hello, {}!", &name)