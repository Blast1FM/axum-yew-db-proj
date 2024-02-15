use axum::routing::get;
use axum::Router;

use crate::handlers;

pub fn create_routes() -> Router
{
    // #TODO add cors
    Router::new()
        .route("/", get(handlers::hello))
        .route("/hello", get(handlers::hello_2))

}