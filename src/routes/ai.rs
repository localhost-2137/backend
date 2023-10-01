use axum::{extract::State, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn ai(State(pool): State<PgPool>) -> Json<Value> {
    Json(json!(vec![""]))
}
