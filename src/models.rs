use super::schema::rustaceans;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, AsChangeset)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
