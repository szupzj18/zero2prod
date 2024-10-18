use actix_web::{ web, HttpResponse };
use uuid::Uuid;
use sqlx::PgPool;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub(crate) name: String,
    pub(crate) email: String,
}


pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );

    // enter the span 
    // bad practice, it's a temporay solution for this.
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
        "Saving subscriber details to the database"
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
    .instrument(query_span)
    .await 
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id {} - failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}