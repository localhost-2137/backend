mod routes;

use crate::routes::router;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router().into_make_service())
        .await
        .unwrap();
}
