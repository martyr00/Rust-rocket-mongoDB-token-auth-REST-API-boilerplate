use crate::error_response::error_responses::{
    ErrorResponse, ALREADY_REGISTERED_JSON, ERROR_ALREADY_REGISTERED_STATUS,
    ERROR_UNAUTHORIZED_STATUS, ERROR_UNKNOWN_STATUS, ERROR_WEAK_LOGIN_STATUS,
    ERROR_WEAK_PASSWORD_STATUS, ERROR_WRONG_FIRST_NAME_STATUS, ERROR_WRONG_LAST_NAME_STATUS,
    ERROR_WRONG_REQUEST_STATUS, UNAUTHORIZED_JSON, UNKNOWN_JSON, WEAK_LOGIN_JSON,
    WEAK_PASSWORD_JSON, WRONG_FIRST_NAME_JSON, WRONG_LAST_NAME_JSON, WRONG_REQUEST_JSON,
};
use rocket::http::Status;
use rocket::serde::json::Json;

//min && max len login
pub const MAX_LEN_LOGIN: usize = 200;
pub const MIN_LEN_LOGIN: usize = 2;

//min && max len password
pub const MAX_LEN_PASSWORD: usize = 200;
pub const MIN_LEN_PASSWORD: usize = 8;

//min && max len first name
pub const MAX_LEN_FIRST_NAME: usize = 100;
pub const MIN_LEN_FIRST_NAME: usize = 2;

//min && max len last name
pub const MAX_LEN_LAST_NAME: usize = 150;
pub const MIN_LEN_LAST_NAME: usize = 2;

pub const WRONG_REQUEST: (Status, Json<ErrorResponse>) =
    (ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON));

pub const ALREADY_REGISTERED: (Status, Json<ErrorResponse>) = (
    ERROR_ALREADY_REGISTERED_STATUS,
    Json(ALREADY_REGISTERED_JSON),
);

pub const WEAK_PASSWORD: (Status, Json<ErrorResponse>) =
    (ERROR_WEAK_PASSWORD_STATUS, Json(WEAK_PASSWORD_JSON));

pub const WEAK_LOGIN: (Status, Json<ErrorResponse>) =
    (ERROR_WEAK_LOGIN_STATUS, Json(WEAK_LOGIN_JSON));

pub const UNKNOWN: (Status, Json<ErrorResponse>) = (ERROR_UNKNOWN_STATUS, Json(UNKNOWN_JSON));

pub const UNAUTHORIZED: (Status, Json<ErrorResponse>) =
    (ERROR_UNAUTHORIZED_STATUS, Json(UNAUTHORIZED_JSON));

pub const WRONG_FIRST_NAME: (Status, Json<ErrorResponse>) =
    (ERROR_WRONG_FIRST_NAME_STATUS, Json(WRONG_FIRST_NAME_JSON));

pub const WRONG_LAST_NAME: (Status, Json<ErrorResponse>) =
    (ERROR_WRONG_LAST_NAME_STATUS, Json(WRONG_LAST_NAME_JSON));
