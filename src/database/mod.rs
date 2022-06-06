use crate::models::tokens::Token;

pub mod connect_to_db;
pub mod impl_mondo_db;

pub enum LoginError {
    Ok(Token),
    WrongLogin,
    WrongPassword,
    Unknown,
}

pub enum RegistrationError {
    Ok(Token),
    AlreadyRegistered,
    WrongPassword,
    Unknown,
}
