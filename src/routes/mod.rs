use self::universities::{get_all_cities, get_all_subjects, search};
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use universities::get_all_universities;

mod distance;
mod universities;

pub fn router(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    Router::new()
        .route("/", get(get_all_universities))
        .route("/cities", get(get_all_cities))
        .route("/subjects", get(get_all_subjects))
        .route("/search", get(search))
        .layer(cors)
        .with_state(pool)
}
