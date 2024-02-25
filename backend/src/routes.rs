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
    .allow_origin(Any);
    Router::new()
        .route("/", get(handlers::hello))
        .route("/hello", get(handlers::hello_2))
        .route("/tenant", post(handlers::tenant_test))
        .route("/api/v1/books", get(handlers::query_book))
        .layer(cors)
        .layer(Extension(db_pool))

}