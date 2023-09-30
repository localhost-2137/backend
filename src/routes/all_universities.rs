use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use serde_json::{json, Value};

#[derive(FromRow)]
struct University {
    id: u32,
    rank: Option<u32>,
    name: String,
    url: String,
    lng: f32,
    lat: f32,
    address: String,
    number_students: u32,
    subjects: String,
}

pub async fn get_all_universities(
    State(pool): State<PgPool>
) -> Json<Value> {
    let all_universities = sqlx::query!(
        "SELECT name FROM universities;"
    )
        .fetch_all(&pool)
        .await.unwrap();
    println!("{:?}", all_universities);
    Json(
        json!(vec![""])
    )
}
