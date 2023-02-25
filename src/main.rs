#[macro_use] extern crate rocket;
use rocket::{serde::json::{ Value, json }, response::status };
mod auth;
use auth::BasicAuth;

#[catch(404)]
fn not_found() -> Value {
    json!({"resource":"Not found"})
}

#[catch(401)]
fn not_authorized() -> Value {
    json!({"message":"Ups, you are not allow here!"})
}

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([{"id": 1, "name": "Jhon Doe"}, {"id": 2, "name": "Jhon Doe Again"}])
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Jhon doe"})
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "Jhon doe"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Jhon doe", "email": "test@gmail.com"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[rocket::main] 

async fn main(){
   let _ = rocket::build().mount("/", routes![
    get_rustaceans,
    view_rustacean,
    create_rustacean,
    update_rustacean,
    delete_rustacean,
   ])
   .register("/", catchers![ not_found, not_authorized ])
   .launch().await; 
}
