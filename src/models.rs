// use diesel::{deserialize::Queryable};
// use rocket::serde;
// use diesel::query_dsl::RunQueryDsl;
// use diesel::deserialize::Queryable;
use diesel::Queryable;
#[derive(serde::Serialize,Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}