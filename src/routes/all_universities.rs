use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct University {
    pub id: i32,
    pub rank: i32,
    pub name: String,
    pub academic: bool,
    pub url: String,
    pub lng: f64,
    pub lat: f64,
    pub address: String,
    pub city: String,
    pub number_students: i32,
    pub subjects: String,
}

pub async fn get_all_universities(State(pool): State<PgPool>) -> Json<Value> {
    let all_universities = sqlx::query_as!(University, "SELECT * FROM universities;")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(json!(all_universities))
}
