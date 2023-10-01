use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq)]
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
    pub subjects: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub cities: String,
    pub subjects: String,
}

pub async fn get_all_cities(State(pool): State<PgPool>) -> Json<Value> {
    let all_cities: Vec<String> =
        sqlx::query_scalar!("SELECT city FROM universities GROUP BY city")
            .fetch_all(&pool)
            .await
            .unwrap();

    Json(json!(all_cities))
}

pub async fn get_all_subjects(State(pool): State<PgPool>) -> Json<Value> {
    let all_subjects: Vec<String> =
        sqlx::query_scalar!("SELECT subject FROM universities_subjects GROUP BY subject")
            .fetch_all(&pool)
            .await
            .unwrap();

    Json(json!(all_subjects))
}

pub async fn search(query: Query<SearchQuery>, State(pool): State<PgPool>) -> Json<Value> {
    if query.cities.is_empty() && query.subjects.is_empty() {
        let mut res: Vec<University> = sqlx::query_as("SELECT *, array((SELECT subject FROM universities_subjects WHERE u_id = universities.id)) as subjects FROM universities;")
            .fetch_all(&pool)
            .await
            .unwrap();
        res.dedup();

        return Json(json!(res));
    } else if query.cities.is_empty() {
        let mut res: Vec<University> = sqlx::query_as("SELECT DISTINCT *, array((SELECT subject FROM universities_subjects WHERE u_id = universities.id)) as subjects FROM universities 
            INNER JOIN universities_subjects ON universities.id=universities_subjects.u_id WHERE universities_subjects.subject = ANY($1);")
            .bind(query.subjects.split(',').collect::<Vec<&str>>())
            .fetch_all(&pool)
            .await
            .unwrap();
        res.dedup();

        return Json(json!(res));
    } else if query.subjects.is_empty() {
        let mut res: Vec<University> = sqlx::query_as("SELECT DISTINCT *, array((SELECT subject FROM universities_subjects WHERE u_id = universities.id)) as subjects FROM universities 
            WHERE universities.city = ANY($1);")
            .bind(query.cities.split(',').collect::<Vec<&str>>())
            .fetch_all(&pool)
            .await
            .unwrap();
        res.dedup();

        return Json(json!(res));
    }

    let mut res: Vec<University> = sqlx::query_as("SELECT DISTINCT *, array((SELECT subject FROM universities_subjects WHERE u_id = universities.id)) as subjects FROM universities 
            INNER JOIN universities_subjects ON universities.id=universities_subjects.u_id WHERE universities_subjects.subject = ANY($1) AND universities.city = ANY($2);")
            .bind(query.subjects.split(',').collect::<Vec<&str>>())
            .bind(query.cities.split(',').collect::<Vec<&str>>())
            .fetch_all(&pool)
            .await
            .unwrap();
    res.dedup();

    Json(json!(res))
}
