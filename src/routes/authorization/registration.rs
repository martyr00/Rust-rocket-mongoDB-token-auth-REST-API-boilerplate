use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::database::connect_to_db::MongoDB;
use crate::r#const::{
    ALREADY_REGISTERED_JSON, ERROR_ALREADY_REGISTERED_STATUS, ERROR_WEAK_LOGIN_STATUS,
    ERROR_WEAK_PASSWORD_STATUS, ERROR_WRONG_REQUEST_STATUS, WEAK_LOGIN_JSON, WEAK_PASSWORD_JSON,
    WRONG_REQUEST_JSON,
};
use crate::routes::routes_models::registration_request::RegistrationRequest;
use crate::routes::{valid_password_and_login, GetIsValidLoginAndPassword};
use crate::{ErrorResponse, ERROR_UNKNOWN_STATUS, UNKNOWN_JSON};

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
        None => Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON))),
        Some(registration_request) => {
            match valid_password_and_login(
                &registration_request.login,
                &registration_request.password,
            ) {
                GetIsValidLoginAndPassword::Ok => {
                    match database
                        .find_user_by_login(registration_request.login.clone())
                        .await
                    {
                        Ok(Some(_)) => Err((
                            ERROR_ALREADY_REGISTERED_STATUS,
                            Json(ALREADY_REGISTERED_JSON),
                        )),
                        Ok(None) => {
                            match database.registration(registration_request).await {
                                Ok(true) => Ok(Status::Ok), //todo response TOKEN
                                Ok(false) => {
                                    Err((ERROR_WEAK_PASSWORD_STATUS, Json(WEAK_PASSWORD_JSON)))
                                }
                                Err(_) => {
                                    Err((ERROR_WEAK_PASSWORD_STATUS, Json(WEAK_PASSWORD_JSON)))
                                }
                            }
                        }
                        Err(_) => Err((ERROR_UNKNOWN_STATUS, Json(UNKNOWN_JSON))),
                    }
                }
                GetIsValidLoginAndPassword::BadLogin => {
                    Err((ERROR_WEAK_LOGIN_STATUS, Json(WEAK_LOGIN_JSON)))
                }
                GetIsValidLoginAndPassword::BadPassword => {
                    Err((ERROR_WEAK_PASSWORD_STATUS, Json(WEAK_PASSWORD_JSON)))
                }
            }
        }
    }
}
