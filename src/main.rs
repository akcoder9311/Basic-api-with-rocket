#[macro_use] extern crate rocket;
extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;



mod auth;
mod models;
mod schema;
mod repositories;

use repositories::RustaceanRepository;
use models::{Rustacean,NewRustacean};
use rocket::serde::json::{Value, json,Json};
use rocket::response::status;
use auth::BasicAuth;



#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db:DbConn) -> Value{
         db.run(|c| {
            let result = RustaceanRepository::find_multiple(c ,100).expect("failed to read rustaceans entries");
            json!(result)
         }).await
}

// for create new 
#[get("/rustaceans/<id>")]
async fn view_rustacens(id:i32, _auth: BasicAuth,db:DbConn)->Value{
     db.run(move |c| {
   let rustacean = RustaceanRepository::find(c , id).expect("failed retrieving rustacean row");
   json!(rustacean)
  }).await

}

#[post("/rustaceans" , format = "json" , data = "<new_rustacean>")]
async fn crete_rustaceans(_auth: BasicAuth,db:DbConn ,new_rustacean: Json<NewRustacean> )->Value{
       let result = db.run(|c| {  
          RustaceanRepository::create(c, new_rustacean.into_inner())
        .expect("failed insertion new_rustacean entry ");
       }).await;
       json!(result)
}

// update for old resource
#[put("/rustaceans/<id>" , format = "json", data="<rustacean>")]
async fn update_rustacens(id:i32, _auth: BasicAuth, db:DbConn ,rustacean: Json<Rustacean>) -> Value {

   db.run(move |c| {
     let result =  RustaceanRepository::save(c, id, rustacean.into_inner())
   .expect("failed updating rustacean entry ");
   json!(result)
}).await
}


// delete exsisting data
#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id:i32, _auth: BasicAuth, db:DbConn )->status::NoContent{
   db.run(move |c| {
    RustaceanRepository::delete(c,id)
       .expect("failed to delete rustacean entry ");
      status::NoContent
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