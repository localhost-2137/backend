use axum::Router;
use axum::routing::get;

mod all_universities;
use all_universities::get_all_universities;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_universities))
}
