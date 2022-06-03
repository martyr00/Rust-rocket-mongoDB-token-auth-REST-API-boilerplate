use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::database::connect_to_db::MongoDB;
use crate::get_valid_text;
use crate::helper::hash_text;
use crate::r#const::{MAX_LEN_LOGIN, MAX_LEN_PASSWORD, MIN_LEN_LOGIN, MIN_LEN_PASSWORD};
use crate::routes::routes_models::registration_request::RegistrationRequest;

enum GetIsValidLoginAndPassword {
    Ok,
    BadLogin,
    BadPassword,
}

#[post(
    "/registration",
    format = "json",
    data = "<maybe_registration_request>"
)]
pub async fn registration(
    database: &State<MongoDB>,
    maybe_registration_request: Option<Json<RegistrationRequest>>,
) -> Result<Status, Status> {
    match maybe_registration_request {
        None => Err(Status::BadRequest),
        Some(registration_request) => {
            match valid_password_and_login(
                &registration_request.login,
                &registration_request.password,
            ) {
                GetIsValidLoginAndPassword::Ok => {
                    match database
                        .check_login_in_db(registration_request.login.clone())
                        .await
                    {
                        Ok(Some(_)) => Err(Status::BadRequest), //todo login busy
                        Ok(None) => {
                            match database.registration(registration_request).await {
                                Ok(true) => Ok(Status::Ok),           //todo response TOKEN
                                Ok(false) => Err(Status::BadRequest), //todo bad password
                                Err(_) => Err(Status::BadRequest),    // todo bad password
                            }
                        }
                        Err(_) => Err(Status::InternalServerError), //todo other
                    }
                }
                GetIsValidLoginAndPassword::BadLogin => {
                    Err(Status::BadRequest) //todo bad login
                }
                GetIsValidLoginAndPassword::BadPassword => {
                    Err(Status::BadRequest) //todo bad password
                }
            }
        }
    }
}

fn valid_password_and_login(login: &str, password: &str) -> GetIsValidLoginAndPassword {
    if get_valid_text(login, MAX_LEN_LOGIN, MIN_LEN_LOGIN) {
        if get_valid_text(password, MAX_LEN_PASSWORD, MIN_LEN_PASSWORD) {
            GetIsValidLoginAndPassword::Ok
        } else {
            GetIsValidLoginAndPassword::BadPassword
        }
    } else {
        GetIsValidLoginAndPassword::BadLogin
    }
}
