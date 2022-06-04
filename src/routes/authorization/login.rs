use crate::database::connect_to_db::MongoDB;
use crate::r#const::{
    ERROR_WRONG_REQUEST_STATUS, MAX_LEN_LOGIN, MAX_LEN_PASSWORD, MIN_LEN_LOGIN, MIN_LEN_PASSWORD,
    WRONG_REQUEST_JSON,
};
use crate::routes::routes_models::login_request::LoginRequest;
use crate::routes::{valid_two_str, GetIsValidTwoStr};
use crate::ErrorResponse;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[post("/login", format = "json", data = "<maybe_login_request>")]
pub async fn login(
    database: &State<MongoDB>,
    maybe_login_request: Option<Json<LoginRequest>>,
) -> Result<Status, (Status, Json<ErrorResponse>)> {
    match maybe_login_request {
        None => Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON))),
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
                        Ok(false) => Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON))),
                        Err(_) => Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON))),
                    }
                }
                GetIsValidTwoStr::BadFirst => {
                    Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON)))
                }
                GetIsValidTwoStr::BadSecond => {
                    Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON)))
                }
            }
        }
    }
}
