use crate::database::connect_to_db::MongoDB;
use crate::r#const::{ERROR_WRONG_REQUEST_STATUS, WRONG_REQUEST_JSON};
use crate::routes::routes_models::login_request::LoginRequest;
use crate::routes::{valid_password_and_login, GetIsValidLoginAndPassword};
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
            match valid_password_and_login(&login_request.login, &login_request.password) {
                GetIsValidLoginAndPassword::Ok => {
                    match database.login(login_request).await {
                        Ok(true) => Ok(Status::Ok), //todo response TOKEN
                        Ok(false) => Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON))),
                        Err(_) => Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON))),
                    }
                }
                GetIsValidLoginAndPassword::BadLogin => {
                    Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON)))
                }
                GetIsValidLoginAndPassword::BadPassword => {
                    Err((ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON)))
                }
            }
        }
    }
}
