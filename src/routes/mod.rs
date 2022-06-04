use crate::get_valid_text;
use crate::r#const::{MAX_LEN_LOGIN, MAX_LEN_PASSWORD, MIN_LEN_LOGIN, MIN_LEN_PASSWORD};

pub mod login;
pub mod registration;
pub mod routes_models;

pub enum GetIsValidLoginAndPassword {
    Ok,
    BadLogin,
    BadPassword,
}

pub fn valid_password_and_login(login: &str, password: &str) -> GetIsValidLoginAndPassword {
    if get_valid_text(login, MAX_LEN_LOGIN, MIN_LEN_LOGIN) {
        if get_valid_text(password, MAX_LEN_PASSWORD, MIN_LEN_PASSWORD) {
            GetIsValidLoginAndPassword::Ok
        } else {
            GetIsValidLoginAndPassword::BadPassword
        }
    } else {
        GetIsValidLoginAndPassword::BadLogin
    }
}
