use crate::models::tokens::Token;
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::oid::ObjectId;
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

pub fn create_token_and_refresh(
    id: ObjectId,
    jwt_secret: &'static str,
    refresh_token_secret: &'static str,
) -> Result<Token, ()> {
    match create_jwt(id, jwt_secret) {
        CreateTokenOutcome::Ok(token) => match create_jwt(id, refresh_token_secret) {
            CreateTokenOutcome::Ok(refresh_token) => Ok(Token {
                token,
                refresh_token,
            }),
            CreateTokenOutcome::Err => Err(()),
        },
        CreateTokenOutcome::Err => Err(()),
    }
}
