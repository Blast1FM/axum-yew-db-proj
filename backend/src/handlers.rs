use axum::{response::IntoResponse, Extension};
use sqlx::{query, PgPool, Pool, Postgres};
use crate::structs;
use axum::Json;

pub async fn hello() -> impl IntoResponse {
    "hello from server!"
}

pub async fn hello_2() -> impl IntoResponse{
    "HELLO 2"
}

pub async fn tenant_test(Json(body):Json<structs::Tenant>) -> impl IntoResponse{
    format!("Tenant's name is {}", body.first_name())
}

pub async fn query_book_name(Extension(db) : &Extension<PgPool>, book_name: String) -> impl IntoResponse{
    let q = "SELECT * from Books";
    //TODO figure this shit out, it bork
    //let query = sqlx::query_as::<_, structs::Book>(q);
    
}