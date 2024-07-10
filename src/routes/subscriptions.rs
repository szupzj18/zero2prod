use actix_web::{ web::{self}, HttpResponse };
use sqlx::{PgPool};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}


pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let _ = sqlx::query!(
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
    .await;
    HttpResponse::Ok().finish()
}