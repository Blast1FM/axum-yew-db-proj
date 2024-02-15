use serde::{Deserialize, Serialize};
use derive_getters::Getters;

#[derive(Serialize, Deserialize, Debug, Getters)]
pub struct Tenant
{
    id: i32,
    first_name: String
}
