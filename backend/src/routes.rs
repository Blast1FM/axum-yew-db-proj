use axum::{
    routing::{get,post},
    Router,
    Extension
};

use sqlx::{Pool, Postgres};

use crate::handlers;

pub fn create_routes(db_pool: Pool<Postgres>) -> Router
{
    // #TODO add cors
    Router::new()
        .route("/", get(handlers::hello))
        .route("/hello", get(handlers::hello_2))
        .route("/tenant", post(handlers::tenant_test))
        .route("/books", get(handlers::query_book_name))
        .layer(Extension(db_pool))

}