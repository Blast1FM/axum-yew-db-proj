use axum::{response::IntoResponse, Extension};
use sqlx::{PgPool, Postgres};
use crate::structs::{self, Book};
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

pub async fn query_book_name(Extension(db) : &Extension<PgPool>, book_name: String)->impl IntoResponse{
    let q = "SELECT * from mini1.\"Books\"";
    let query = sqlx::query_as::<Postgres, Book>(q);
    let books = query.fetch_all(db).await.unwrap();
    Json(books[0].to_owned())
    //println!("{:?}",books[0]);
    //Json(books)
    //TODO figure this shit out, it bork
    //let query = sqlx::query_as::<_, structs::Book>(q);
    //let query = sqlx::query(&q).fetch_all(db);
}