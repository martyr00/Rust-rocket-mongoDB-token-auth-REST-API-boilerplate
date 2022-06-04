use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegistrationRequest {
    pub login: String,
    pub password: String,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
