use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegistrationRequest {
    pub login: String,
    pub password: String,

    //#[validate(length(min = 200))]
    pub first_name: Option<String>,

    //#[validate(length(min = 200))]
    pub last_name: Option<String>,
}
