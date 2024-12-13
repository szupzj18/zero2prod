use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;
use serde_json::json;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub(crate) name: String,
    pub(crate) email: String,
}

#[tracing::instrument(
    name = "Adding a new subcriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_span = tracing::info_span!("adding a new subscriber",);

    // enter the span
    // bad practice, it's a temporay solution, we will fix later.
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving subscriber details to the database");
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
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "ok" })),
        Err(e) => {
            tracing::error!("failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
