use crate::constants::{
    MAX_LEN_LOGIN, MAX_LEN_PASSWORD, MIN_LEN_LOGIN, MIN_LEN_PASSWORD, WRONG_REQUEST,
};
use crate::database::connect_to_db::MongoDB;
use crate::error_response::error_responses::ErrorResponse;
use crate::routes::routes_models::login_request::LoginRequest;
use crate::routes::{valid_two_str, GetIsValidTwoStr};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[post("/login", format = "json", data = "<maybe_login_request>")]
pub async fn login(
    database: &State<MongoDB>,
    maybe_login_request: Option<Json<LoginRequest>>,
) -> Result<Status, (Status, Json<ErrorResponse>)> {
    match maybe_login_request {
        None => Err(WRONG_REQUEST),
        Some(login_request) => {
            match valid_two_str(
                &login_request.login,
                &login_request.password,
                MAX_LEN_LOGIN,
                MIN_LEN_LOGIN,
                MAX_LEN_PASSWORD,
                MIN_LEN_PASSWORD,
            ) {
                GetIsValidTwoStr::Ok => {
                    match database.login(login_request).await {
                        Ok(true) => Ok(Status::Ok), //todo response TOKEN
                        Ok(false) => Err(WRONG_REQUEST),
                        Err(_) => Err(WRONG_REQUEST),
                    }
                }
                GetIsValidTwoStr::BadFirst => Err(WRONG_REQUEST),
                GetIsValidTwoStr::BadSecond => Err(WRONG_REQUEST),
            }
        }
    }
}
