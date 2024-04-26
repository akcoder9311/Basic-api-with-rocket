#[macro_use] extern crate rocket;
extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;



mod auth;
mod models;
mod schema;

use diesel::prelude::*;
use models::{Rustacean,NewRustacean};
use rocket::serde::json::{Value, json,Json};
use rocket::response::status;
use auth::BasicAuth;
use schema::rustaceans;



#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(auth: BasicAuth, db:DbConn) -> Value{
         db.run(|c| {
            let result = rustaceans::table.limit(100).load::<Rustacean>(c).expect("failed to read rustaceans entries");
            json!(result)
         }).await
}

// for create new 
#[get("/rustaceans/<id>")]
fn view_rustacens(id:i32, _auth: BasicAuth)->Value{
  json!([{"id":id , "name":"ak khan","email":"antor@gmial.com"}])

}

#[post("/rustaceans" , format = "json" , data = "<new_rustacean>")]
async fn crete_rustaceans(_auth: BasicAuth,db:DbConn ,new_rustacean: Json<NewRustacean> )->Value{
       let result = db.run(|c| {  
        diesel::insert_into(rustaceans::table)
        .values(new_rustacean.into_inner())
        .execute(c)
        .expect("failed insertion new_rustacean entry ");
       }).await;
       json!(result)
}

// update for old resource
#[put("/rustaceans/<id>" , format = "json")]
fn update_rustacens(id:i32, _auth: BasicAuth)->Value{
  json!([{"id":id , "name":"ak khan","email":"antor@gmial.com"}])
}


// delete exsisting data
#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id:i32, _auth: BasicAuth)->status::NoContent{
  status::NoContent
}

#[catch(404)]
fn not_found()->Value{
  json!("cant get anything")
}
#[rocket::main]
async fn main(){
  let _ = rocket::build()
                .mount("/", routes![
                        get_rustaceans,
                        view_rustacens,
                        crete_rustaceans,
                        update_rustacens,
                        delete_rustaceans
                 ])
                .register("/", catchers![
                   not_found
                ])
                .attach(DbConn::fairing())
                .launch()
                .await;
}