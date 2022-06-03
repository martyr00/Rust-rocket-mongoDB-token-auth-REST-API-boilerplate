#[macro_use]
extern crate rocket;

use crate::database::connect_to_db::init;
use crate::helper::get_valid_text;
use crate::routes::authorization::registration;

pub mod r#const;
pub mod database;
pub mod helper;
pub mod model_user;
pub mod routes;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(init().await)
        .mount("/api/v1", routes![registration])
}
