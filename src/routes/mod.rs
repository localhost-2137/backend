use self::universities::get_all_cities;
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
        .with_state(pool)
}
