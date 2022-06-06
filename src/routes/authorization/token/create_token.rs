use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use mongodb::bson::oid::ObjectId;
use rocket::futures::future::err;
use serde::{Deserialize, Serialize};

pub enum CreateTokenOutcome {
    Ok(String),
    Err,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: String,
}

pub fn create_jwt(id: ObjectId, secret: &'static str) -> CreateTokenOutcome {
    let my_claims = Claims {
        user_id: id.to_string(),
    };
    match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(token) => CreateTokenOutcome::Ok(token),
        Err(_) => CreateTokenOutcome::Err,
    }
}
