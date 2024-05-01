use super::schema::rustaceans;
use diesel::{Queryable,Insertable};
use serde::Deserialize;


#[derive(serde::Serialize,Deserialize,Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}



#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean{
    pub name: String,
    pub email: String,
}