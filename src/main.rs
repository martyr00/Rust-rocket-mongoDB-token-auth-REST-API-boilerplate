#[macro_use]
extern crate rocket;

use crate::constants::{UNAUTHORIZED, UNKNOWN};
use crate::database::connect_to_db::init;
use crate::error_response::error_responses::ErrorResponse;
use crate::helper::get_valid_text;
use crate::routes::authorization::login::login;
use crate::routes::authorization::registration::registration;
use rocket::http::Status;
use rocket::serde::json::Json;

pub mod constants;
pub mod database;
pub mod error_response;
pub mod helper;
pub mod models;
pub mod private;
pub mod routes;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(init().await)
        .mount("/api/v1", routes![registration, login])
        .register("/", catchers![])
}

#[catch(401)]
pub fn unauthorized() -> (Status, Json<ErrorResponse>) {
    UNAUTHORIZED
}

#[catch(401)]
pub fn internal_sever_error() -> (Status, Json<ErrorResponse>) {
    UNKNOWN
}

// Bearer
// match get one header Authorization -> (Bearer 'TOKEN')
//     Some(header) => {
//         let array_header_val = header.split(" ")
//             if array_header_val[1].is_empty {return Err(Status::401)}
//             else {
//                 array_header_val[1].parse_from_JWD() => struct { user_id: 'ObjectId' }
//                 if find user in DB by user_id {
//                     return Ok(Status::Ok)
//                 } else {
//                      return Err(Status::401)
//                 }
//             }
//         },
//     None(_) => return Err(Status::401)
