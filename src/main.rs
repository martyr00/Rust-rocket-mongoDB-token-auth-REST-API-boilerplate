#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::constants::{NOT_FOUND, UNAUTHORIZED, UNKNOWN};
use crate::database::connect_to_db::init;
use crate::error_response::error_responses::{
    ErrorResponse, NOT_FOUND_JSON, UNAUTHORIZED_JSON, UNKNOWN_JSON,
};
use crate::helper::check_valid_text;
use crate::routes::authorization::login::login;
use crate::routes::authorization::registration::registration;
use crate::routes::routes::delete_user::delete_user;
use crate::routes::routes::hello_name::{hello_name_user, hello_world};
use crate::routes::routes::refresh_tokens::refresh_tokens;

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
        .mount(
            "/api/v1",
            routes![
                registration,
                login,
                hello_name_user,
                hello_world,
                refresh_tokens,
                delete_user
            ],
        )
        .register(
            "/",
            catchers![unauthorized, not_found, internal_sever_error,],
        )
}

#[catch(401)]
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(UNAUTHORIZED_JSON)
}

#[catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(NOT_FOUND_JSON)
}

#[catch(500)]
pub fn internal_sever_error() -> Json<ErrorResponse> {
    Json(UNKNOWN_JSON)
}
