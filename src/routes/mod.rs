use self::ai::ai;
use self::universities::{get_all_cities, get_all_subjects, get_university, search};
use crate::routes::universities::all_universities_locations;
use axum::routing::{get, post};
use axum::Router;
use lazy_static::lazy_static;
use sqlx::PgPool;
use std::env;
use tower_http::cors::{Any, CorsLayer};

mod ai;
mod distance;
mod universities;

lazy_static! {
    pub static ref MAPS_API_KEY: String = env::var("GOOGLE_MAPS_API_KEY").unwrap();
    pub static ref OPENAI_KEY: String = env::var("OPENAI_KEY").unwrap();
}

pub fn router(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    Router::new()
        .route("/cities", get(get_all_cities))
        .route("/subjects", get(get_all_subjects))
        .route("/search", get(search))
        .route("/ai", post(ai))
        .route("/university/:u_id", get(get_university))
        .route("/universities/location", get(all_universities_locations))
        .layer(cors)
        .with_state(pool)
}
