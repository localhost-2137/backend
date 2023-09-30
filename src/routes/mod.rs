use axum::Router;
use axum::routing::get;
use sqlx::PgPool;

mod all_universities;
mod distance;

use all_universities::get_all_universities;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_all_universities))
        .with_state(pool)
}
