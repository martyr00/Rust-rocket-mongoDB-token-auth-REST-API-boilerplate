pub mod connect_to_db;
pub mod impl_mondo_db;

pub enum LoginError {
    Ok,
    WrongLogin,
    WrongPassword,
}

pub enum RegistrationError {
    Ok,
    AlreadyRegistered,
    WrongPassword,
    Unknown,
}
