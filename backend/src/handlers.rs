use axum::response::IntoResponse;

pub async fn hello() -> impl IntoResponse {
    "hello from server!"
}

pub async fn hello_2() -> impl IntoResponse{
    "HELLO 2"
}