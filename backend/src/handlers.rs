use axum::response::IntoResponse;
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