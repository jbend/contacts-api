use axum::{extract, http};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Contact {
    id: uuid::Uuid,
    name: String,
    email: String,
    phone: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Contact {
    fn new(name: String, email: String, phone: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            email,
            phone,
            inserted_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateContact {
    name: String,
    email: String,
    phone: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_contact(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateContact>,
) -> Result<(http::StatusCode, axum::Json<Contact>), http::StatusCode> {
    let contact = Contact::new(payload.name, payload.email, payload.phone);

    let res = sqlx::query(
        r#"
        INSERT INTO contacts (id, name, email, phone, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(&contact.id)
    .bind(&contact.name)
    .bind(&contact.email)
    .bind(&contact.phone)
    .bind(&contact.inserted_at)
    .bind(&contact.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(contact))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }

}