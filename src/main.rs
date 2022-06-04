#[macro_use]
extern crate rocket;

use crate::database::connect_to_db::init;
use crate::helper::get_valid_text;
use crate::r#const::{ERROR_UNAUTHORIZED, ERROR_UNKNOWN};
use crate::routes::authorization::login::login;
use crate::routes::authorization::registration::registration;
use rocket::http::Status;

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
pub fn unauthorized() -> (Status, &'static str) {
    ERROR_UNAUTHORIZED
}

#[catch(401)]
pub fn internal_sever_error() -> (Status, &'static str) {
    ERROR_UNKNOWN
}
