use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::constants::{
    ALREADY_REGISTERED, MAX_LEN_FIRST_NAME, MAX_LEN_LAST_NAME, MAX_LEN_LOGIN, MAX_LEN_PASSWORD,
    MIN_LEN_FIRST_NAME, MIN_LEN_LAST_NAME, MIN_LEN_LOGIN, MIN_LEN_PASSWORD, UNKNOWN, WEAK_LOGIN,
    WEAK_PASSWORD, WRONG_FIRST_NAME, WRONG_LAST_NAME, WRONG_REQUEST,
};
use crate::database::connect_to_db::MongoDB;
use crate::error_response::error_responses::ErrorResponse;
use crate::routes::routes_models::registration_request::RegistrationRequest;
use crate::routes::{valid_two_names, valid_two_str, GetIsValidTwoStr};

#[post(
    "/registration",
    format = "json",
    data = "<maybe_registration_request>"
)]
pub async fn registration(
    database: &State<MongoDB>,
    maybe_registration_request: Option<Json<RegistrationRequest>>,
) -> Result<Status, (Status, Json<ErrorResponse>)> {
    match maybe_registration_request {
        None => Err(WRONG_REQUEST),
        Some(registration_request) => {
            match valid_two_str(
                &registration_request.login,
                &registration_request.password,
                MAX_LEN_LOGIN,
                MIN_LEN_LOGIN,
                MAX_LEN_PASSWORD,
                MIN_LEN_PASSWORD,
            ) {
                GetIsValidTwoStr::Ok => {
                    match valid_two_names(
                        &registration_request.first_name,
                        &registration_request.last_name,
                        MAX_LEN_FIRST_NAME,
                        MIN_LEN_FIRST_NAME,
                        MAX_LEN_LAST_NAME,
                        MIN_LEN_LAST_NAME,
                    ) {
                        GetIsValidTwoStr::Ok => {
                            match database
                                .find_user_by_login(registration_request.login.clone())
                                .await
                            {
                                Ok(Some(_)) => Err(ALREADY_REGISTERED),
                                Ok(None) => {
                                    match database.registration(registration_request).await {
                                        Ok(true) => Ok(Status::Ok), //todo response TOKEN
                                        Ok(false) => Err(WEAK_PASSWORD),
                                        Err(_) => Err(WEAK_PASSWORD),
                                    }
                                }
                                Err(_) => Err(UNKNOWN),
                            }
                        }
                        GetIsValidTwoStr::BadFirst => Err(WRONG_FIRST_NAME),
                        GetIsValidTwoStr::BadSecond => Err(WRONG_LAST_NAME),
                    }
                }
                GetIsValidTwoStr::BadFirst => Err(WEAK_LOGIN),
                GetIsValidTwoStr::BadSecond => Err(WEAK_PASSWORD),
            }
        }
    }
}
