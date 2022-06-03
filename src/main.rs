#[macro_use]
extern crate rocket;

use crate::helper::get_valid_text;
use crate::database::connect_to_db::init;
use crate::routes::authorization::registration;

pub mod helper;
pub mod routes;
pub mod database;
pub mod model_user;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(init().await)
        .mount(
            "/api/v1",
            routes![
                registration
            ],
        )
}