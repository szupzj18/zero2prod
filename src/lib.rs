pub mod routes;
pub mod configuration;
pub mod startup;
use actix_web::{ web, HttpResponse };
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber",
        request_id,
        form.email,
        form.name
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(pool.get_ref())
    .await 
    {
        Ok(_) => { 
            log::info!("request_id {} - New Subscriber: {} details have been saved", request_id, form.email);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("request_id {} - failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn index(form: web::Form<FormData>) -> String {
    format!("Welcome, {} -- email: {}", form.name,form.email)
}

// async fn index(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     println!("recieved from req uri: {}, method {}", req.path(), req.method());
//     format!("Hello, {}!", &name)