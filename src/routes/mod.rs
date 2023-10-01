use self::universities::{get_all_cities, get_all_subjects, search, get_university};
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

mod distance;
mod universities;

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
        .layer(cors)
        .with_state(pool)
}
