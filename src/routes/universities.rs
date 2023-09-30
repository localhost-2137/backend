use axum::extract::{Query, State};
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

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub cities: Vec<String>,
    pub studies: Vec<String>,
    pub academic: bool,
}

pub async fn get_all_universities(State(pool): State<PgPool>) -> Json<Value> {
    let all_universities = sqlx::query_as!(University, "SELECT * FROM universities;")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(json!(all_universities))
}

pub async fn get_all_cities(State(pool): State<PgPool>) -> Json<Value> {
    let all_cities: Vec<String> =
        sqlx::query_scalar!("SELECT city FROM universities GROUP BY city")
            .fetch_all(&pool)
            .await
            .unwrap();

    Json(json!(all_cities))
}

pub async fn search(query: Query<SearchQuery>) -> Json<Value> {
    println!("{:#?}", query);

    Json(json!(vec![""]))
}
