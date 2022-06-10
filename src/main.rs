#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::constants::{NOT_FOUND, UNAUTHORIZED, UNKNOWN};
use crate::database::connect_to_db::init;
use crate::error_response::error_responses::ErrorResponse;
use crate::helper::check_valid_text;
use crate::routes::authorization::login::login;
use crate::routes::authorization::registration::registration;
use crate::routes::test_routes::hello_name::hello_name_user;

pub mod constants;
mod database;
pub mod error_response;
mod helper;
mod models;
mod private;
mod routes;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(init().await)
        .mount("/api/v1", routes![registration, login, hello_name_user])
        .register("/", catchers![])
}

#[catch(401)]
pub fn unauthorized() -> (Status, Json<ErrorResponse>) {
    UNAUTHORIZED
}

#[catch(401)]
pub fn not_found() -> (Status, Json<ErrorResponse>) {
    NOT_FOUND
}

#[catch(401)]
pub fn internal_sever_error() -> (Status, Json<ErrorResponse>) {
    UNKNOWN
}
