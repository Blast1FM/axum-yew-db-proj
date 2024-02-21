use serde::{Deserialize, Serialize};
use derive_getters::Getters;
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Debug, Getters)]
pub struct Tenant
{
    id: i32,
    first_name: String
}

#[derive (Serialize, Deserialize, Debug, Getters, FromRow)]
pub struct Book
{
    id: i32,
    author: String,
    name: String,
    publish_date: String,
    publisher: String,
    synopsis: String
}