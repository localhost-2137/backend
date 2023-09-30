use self::universities::{get_all_cities, search};
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;
use universities::get_all_universities;

mod distance;
mod universities;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_all_universities))
        .route("/cities", get(get_all_cities))
        .route("/search", get(search))
        .with_state(pool)
}
