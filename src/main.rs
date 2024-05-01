#[macro_use] extern crate rocket;
extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;



mod auth;
mod models;
mod schema;
mod repositories;

use repositories::RustaceanRepository;
use models::{Rustacean,NewRustacean};
use rocket::http::Status;
use rocket::serde::json::{Value, json,Json};
use rocket::response::status::{self,Custom};
use auth::BasicAuth;



#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db:DbConn) -> Result<Value,Custom<Value>>{
         db.run(|c| {
          RustaceanRepository::find_multiple(c ,100)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError,json!(e.to_string())))
         }).await
}

// for create new 
#[allow(dead_code)]
#[get("/rustaceans/<id>")]
async fn view_rustacens(id:i32, _auth: BasicAuth,db:DbConn)->Result<Value,Custom<Value>>{
     db.run(move |c| {
    RustaceanRepository::find(c , id)
         .map(|rustacean| json!(rustacean))
         .map_err(|e| Custom(Status::InternalServerError,json!(e.to_string())))

  }).await

}

#[post("/rustaceans" , format = "json" , data = "<new_rustacean>")]
async fn crete_rustaceans(_auth: BasicAuth,db:DbConn ,new_rustacean: Json<NewRustacean> )-> Result<Value,Custom<Value>> {
       db.run(|c| {  
          RustaceanRepository::create(c, new_rustacean.into_inner())
        .map(|rustacean| json!(rustacean))
        .map_err(|e| Custom(Status::InternalServerError ,json!(e.to_string())))
       }).await
       
}

// update for old resource
#[put("/rustaceans/<id>" , format = "json", data="<rustacean>")]
async fn update_rustacens(id:i32, _auth: BasicAuth, db:DbConn ,rustacean: Json<Rustacean>) -> Result<Value,Custom<Value>> {

   db.run(move |c| {
      RustaceanRepository::save(c, id, rustacean.into_inner())
        .map(|rustacean| json!(rustacean))
        .map_err(|e| Custom(Status::InternalServerError , json!(e.to_string())))
}).await
}


// delete exsisting data
#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id:i32, _auth: BasicAuth, db:DbConn )-> Result<status::NoContent , Custom<Value>>{
   db.run(move |c| {
    RustaceanRepository::delete(c , id)
      .map(|_| status::NoContent )
      .map_err(|e| Custom(Status::InternalServerError , json!(e.to_string())))
   }).await
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