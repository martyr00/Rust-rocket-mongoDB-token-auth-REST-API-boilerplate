use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegistrationRequest {
    pub login: String,
    pub password: String,
    //pub full_name: Option<String>,
}
