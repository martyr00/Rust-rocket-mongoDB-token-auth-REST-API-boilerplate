#[macro_use]
extern crate rocket;

use crate::database::connect_to_db::init;
use crate::helper::get_valid_text;
use crate::r#const::{
    ErrorResponse, ERROR_UNAUTHORIZED_STATUS, ERROR_UNKNOWN_STATUS, UNAUTHORIZED_JSON, UNKNOWN_JSON,
};
use crate::routes::authorization::login::login;
use crate::routes::authorization::registration::registration;
use rocket::http::Status;
use rocket::serde::json::Json;

pub mod r#const;
pub mod database;
pub mod helper;
pub mod models;
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
    (ERROR_UNAUTHORIZED_STATUS, Json(UNAUTHORIZED_JSON))
}

#[catch(401)]
pub fn internal_sever_error() -> (Status, Json<ErrorResponse>) {
    (ERROR_UNKNOWN_STATUS, Json(UNKNOWN_JSON))
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
