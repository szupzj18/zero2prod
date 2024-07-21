use actix_web::{ web::{self}, HttpResponse };
use serde::de::value;
use serde_json::value;
use sqlx::{PgPool};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub(crate) name: String,
    pub(crate) email: String,
}


pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Adding a new subscriber. name: {}, email: {}", form.name, form.email);
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
    .await {
        Ok(_) => {
            log::info!("Subscribed {} with email {}", form.name, form.email);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}