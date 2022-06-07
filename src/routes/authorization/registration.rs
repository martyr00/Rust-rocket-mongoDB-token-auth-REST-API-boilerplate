use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::constants::{
    ALREADY_REGISTERED_LOGIN, ALREADY_REGISTERED_MAIL, LEN_FIRST_NAME, LEN_LAST_NAME, LEN_LOGIN,
    LEN_PASSWORD, UNKNOWN, WEAK_LOGIN, WEAK_PASSWORD, WRONG_FIRST_NAME, WRONG_LAST_NAME,
    WRONG_MAIL, WRONG_REQUEST,
};
use crate::database::connect_to_db::MongoDB;
use crate::database::RegistrationError;
use crate::error_response::error_responses::ErrorResponse;
use crate::models::tokens::Token;
use crate::routes::routes_models::registration_request::RegistrationRequest;
use crate::routes::validator_authorization::valid_registration_data_user;
use crate::routes::TypeValidDataFromRegistration;

#[post(
    "/registration",
    format = "json",
    data = "<maybe_registration_request>"
)]
pub async fn registration(
    database: &State<MongoDB>,
    maybe_registration_request: Option<Json<RegistrationRequest>>,
) -> Result<Json<Token>, (Status, Json<ErrorResponse>)> {
    match maybe_registration_request {
        None => Err(WRONG_REQUEST),
        Some(registration_request) => {
            match valid_registration_data_user(
                &registration_request,
                LEN_FIRST_NAME,
                LEN_LAST_NAME,
                LEN_LOGIN,
                LEN_PASSWORD,
            ) {
                TypeValidDataFromRegistration::Ok => {
                    match database.registration(registration_request).await {
                        Ok(RegistrationError::Ok(token)) => Ok(Json(token)),
                        Ok(RegistrationError::AlreadyRegisteredByEmail) => {
                            Err(ALREADY_REGISTERED_MAIL)
                        }
                        Ok(RegistrationError::AlreadyRegisteredByLogin) => {
                            Err(ALREADY_REGISTERED_LOGIN)
                        }
                        Ok(RegistrationError::WrongPassword) => Err(WEAK_PASSWORD),
                        Ok(RegistrationError::Unknown) => Err(UNKNOWN),
                        Err(_) => Err(UNKNOWN),
                    }
                }
                TypeValidDataFromRegistration::BadMail => Err(WRONG_MAIL),
                TypeValidDataFromRegistration::BadFirstName => Err(WRONG_FIRST_NAME),
                TypeValidDataFromRegistration::BadLastName => Err(WRONG_LAST_NAME),
                TypeValidDataFromRegistration::BadLogin => Err(WEAK_LOGIN),
                TypeValidDataFromRegistration::BadPassword => Err(WEAK_PASSWORD),
            }
        }
    }
}
