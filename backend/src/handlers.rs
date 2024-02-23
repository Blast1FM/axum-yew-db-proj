use axum::{response::IntoResponse, Extension};
use sqlx::{PgPool, Postgres};
use crate::structs::{self, Book, BookQueryRequest};
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

pub async fn query_book(Extension(db) : Extension<PgPool>, book_request: Json<BookQueryRequest>)-> Json<Vec<Book>>{
    let q = format!("SELECT * from mini1.\"Books\" WHERE '{}' ~ '{}';", book_request.row_name, book_request.regexp);
    let query = sqlx::query_as::<Postgres, Book>(&q);
    // TODO ERROR HANDLING
    let books = query.fetch_all(&db).await.unwrap();
    Json(books.to_owned())
}