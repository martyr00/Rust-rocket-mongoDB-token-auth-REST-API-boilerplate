use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::constants::{LEN_LOGIN, LEN_PASSWORD, WRONG_REQUEST};
use crate::database::connect_to_db::MongoDB;
use crate::database::LoginError;
use crate::error_response::error_responses::ErrorResponse;
use crate::routes::routes_models::login_request::LoginRequest;
use crate::routes::validator_authorization::get_valid_login_and_password;
use crate::routes::TypeValidTwoStr;

#[post("/login", format = "json", data = "<maybe_login_request>")]
pub async fn login(
    database: &State<MongoDB>,
    maybe_login_request: Option<Json<LoginRequest>>,
) -> Result<Status, (Status, Json<ErrorResponse>)> {
    match maybe_login_request {
        None => Err(WRONG_REQUEST),
        Some(login_request) => {
            match get_valid_login_and_password(
                &login_request.login,
                &login_request.password,
                LEN_LOGIN,
                LEN_PASSWORD,
            ) {
                TypeValidTwoStr::Ok => {
                    match database.login(login_request).await {
                        Ok(LoginError::Ok) => Ok(Status::Ok), //todo response TOKEN
                        Ok(LoginError::WrongPassword) => Err(WRONG_REQUEST),
                        Ok(LoginError::WrongLogin) => Err(WRONG_REQUEST),
                        Err(_) => Err(WRONG_REQUEST),
                    }
                }
                TypeValidTwoStr::BadFirst => Err(WRONG_REQUEST),
                TypeValidTwoStr::BadSecond => Err(WRONG_REQUEST),
            }
        }
    }
}
