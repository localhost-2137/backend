use std::env;

use self::universities::{get_all_cities, get_all_subjects, get_university, search};
use axum::routing::get;
use axum::Router;
use lazy_static::lazy_static;
use sqlx::PgPool;
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
        .route("/university/:u_id", get(get_university))
        .route("/cities", get(get_all_cities))
        .layer(cors)
        .with_state(pool)
}
