use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Form};
use axum_macros::debug_handler;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

use chrono::Utc;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    Extension(pool): Extension<Arc<PgPool>>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email,name,subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query:{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
