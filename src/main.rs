#[macro_use] extern crate rocket;

use rocket::serde::json::{json, Value};
use rocket::response::status;



#[get("/rustaceans")]
fn get_rustaceans()->Value{
  json!([{"id":1 , "name":"amir khan"},{"id":2,"name":"zuber sir"}])
}

// for create new 
#[get("/rustaceans/<id>")]
fn view_rustacens(id:i32)->Value{
  json!([{"id":id , "name":"ak khan","email":"antor@gmial.com"}])

}

#[post("/rustaceans" , format = "json")]
fn  crete_rustaceans()->Value{
        json!([{"id":3,"name": "sahil","email": "sahil@gmail.com"}])
}

// update for old resource
#[put("/rustaceans/<id>" , format = "json")]
fn update_rustacens(id:i32)->Value{
  json!([{"id":id , "name":"ak khan","email":"antor@gmial.com"}])
}


// delete exsisting data
#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id:i32)->status::NoContent{
  status::NoContent
}

#[catch(404)]
fn dont_get()->Value{
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
                  dont_get
                ])
                .launch()
                .await;
}