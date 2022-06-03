use rocket::http::Status;

//min && max len login
pub const MAX_LEN_LOGIN: usize = 200;
pub const MIN_LEN_LOGIN: usize = 2;

//min && max len password
pub const MAX_LEN_PASSWORD: usize = 200;
pub const MIN_LEN_PASSWORD: usize = 8;

// common errors
pub const ERROR_UNKNOWN: (Status, &'static str) = (Status::InternalServerError, "InternalServerError");

pub const ERROR_WRONG_REQUEST: (Status, &'static str) = (Status::BadRequest, "wrong_request");

pub const ERROR_UNAUTHORIZED: (Status, &'static str) = (Status::Unauthorized, "unauthorized");

// login error
pub const ERROR_USER_NOT_FOUND: (Status, &'static str) = (Status::BadRequest, "user_not_found");

// registration error
pub const ERROR_WEAK_PASSWORD: (Status, &'static str) = (Status::BadRequest, "weak_password");

pub const ERROR_WEAK_LOGIN: (Status, &'static str) = (Status::BadRequest, "weak_login");

pub const ERROR_ALREADY_REGISTERED: (Status, &'static str) = (Status::BadRequest, "already_registered");
