mod routes;
mod pg_pool;
mod g_maps;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::routes::router;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:test@localhost/school_finder")
        .await
        .unwrap();
    
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router(pool).into_make_service())
        .await
        .unwrap();
}
