use axum::{
    http::Method, routing::{get,post}, Extension, Router
};

use sqlx::{Pool, Postgres};
use tower_http::cors::{CorsLayer, Any};

use crate::handlers;

pub fn create_routes(db_pool: Pool<Postgres>) -> Router
{
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any)
    .allow_headers(Any);

    Router::new()
        .route("/", get(handlers::hello))
        .route("/hello", post(handlers::hello_2))
        .route("/tenant", post(handlers::tenant_test))
        .route("/api/v1/books", post(handlers::query_book))
        .layer(Extension(db_pool))
        .layer(cors)
}