#[macro_use] extern crate rocket;

use rocket::{serde::json::{ Value, json }, response::status, request::{ FromRequest, Request, Outcome }, http::{ Status } };


pub struct BasicAuth{
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header : &str) -> Option<BasicAuth>{
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }
        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth>{
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        let ( username, password ) = (split[0].to_string(), split[1].to_string());


        Some(BasicAuth { username, password })
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!({"resource":"Not found"})
}

#[catch(401)]
fn not_authorized() -> Value {
    json!({"message":"Ups, you are not allow here!"})
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth{
    type Error = ();

    async fn from_request( request: &'r Request<'_>) -> Outcome<Self, Self::Error>{
        let auth_header = request.headers().get_one("Authorization");

        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header){
                return Outcome::Success(auth);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([{"id": 1, "name": "Jhon Doe"}, {"id": 2, "name": "Jhon Doe Again"}])
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> Value {
    json!({"id": id, "name": "Jhon doe"})
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({"id": 3, "name": "Jhon doe"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Value {
    json!({"id": id, "name": "Jhon doe", "email": "test@gmail.com"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
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