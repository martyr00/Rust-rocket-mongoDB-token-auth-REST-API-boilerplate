use rocket::http::Status;
use serde::Serialize;

//min && max len login
pub const MAX_LEN_LOGIN: usize = 200;
pub const MIN_LEN_LOGIN: usize = 2;

//min && max len password
pub const MAX_LEN_PASSWORD: usize = 200;
pub const MIN_LEN_PASSWORD: usize = 8;

//min && max len first name
pub const MAX_LEN_FIRST_NAME: usize = 200;
pub const MIN_LEN_FIRST_NAME: usize = 2;

//min && max len last name
pub const MAX_LEN_LAST_NAME: usize = 200;
pub const MIN_LEN_LAST_NAME: usize = 8;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub(crate) cause: &'static str,
}

// common errors
pub const ERROR_UNKNOWN_STATUS: Status = Status::InternalServerError;
pub const UNKNOWN_JSON: ErrorResponse = ErrorResponse {
    cause: "Internal Server Error",
};

pub const ERROR_WRONG_REQUEST_STATUS: Status = Status::BadRequest;
pub const WRONG_REQUEST_JSON: ErrorResponse = ErrorResponse {
    cause: "Wrong request",
};

pub const ERROR_UNAUTHORIZED_STATUS: Status = Status::Unauthorized;
pub const UNAUTHORIZED_JSON: ErrorResponse = ErrorResponse {
    cause: "Unauthorized",
};

// login error
pub const ERROR_USER_NOT_FOUND_STATUS: Status = Status::BadRequest;
pub const USER_NOT_FOUND_JSON: ErrorResponse = ErrorResponse {
    cause: "User not found",
};

// registration error
pub const ERROR_WEAK_PASSWORD_STATUS: Status = Status::BadRequest;
pub const WEAK_PASSWORD_JSON: ErrorResponse = ErrorResponse {
    cause: "Week password",
};

pub const ERROR_WEAK_LOGIN_STATUS: Status = Status::BadRequest;
pub const WEAK_LOGIN_JSON: ErrorResponse = ErrorResponse {
    cause: "Weak login",
};

pub const ERROR_ALREADY_REGISTERED_STATUS: Status = Status::BadRequest;
pub const ALREADY_REGISTERED_JSON: ErrorResponse = ErrorResponse {
    cause: "Already registered",
};
